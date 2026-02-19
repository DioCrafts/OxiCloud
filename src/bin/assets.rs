use anyhow::{Context, Result, anyhow, bail};
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use swc_common::{FileName, GLOBALS, Globals, Mark, SourceMap, sync::Lrc};
use swc_ecma_ast::Program;
use swc_ecma_codegen::{Config as CodegenConfig, Emitter, Node, text_writer::JsWriter};
use swc_ecma_minifier::{
    optimize,
    option::{ExtraOptions, MangleOptions, MinifyOptions},
};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::{fixer::fixer, resolver};
use swc_ecma_visit::VisitMutWith;

#[derive(Debug, Clone)]
struct PageAssetsRaw {
    html_file: PathBuf,
    html_name: String,
    js: Vec<String>,
    css: Vec<String>,
    has_dist_asset_refs: bool,
}

#[derive(Debug, Clone)]
struct ChunkBuild {
    kind: AssetKind,
    logical_name: String,
    output_web_path: String,
    inputs: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum AssetKind {
    Js,
    Css,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PageManifest {
    js: Vec<String>,
    css: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChunkManifest {
    kind: String,
    logical_name: String,
    file: String,
    inputs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BuildManifest {
    version: u32,
    generated_by: String,
    pages: BTreeMap<String, PageManifest>,
    chunks: Vec<ChunkManifest>,
}

#[derive(Debug, Clone)]
struct RecoveredPageInputs {
    js: Vec<String>,
    css: Vec<String>,
}

#[derive(Debug, Clone)]
struct Settings {
    static_dir: PathBuf,
    dist_dir: PathBuf,
    rewrite_html: bool,
}

const START_MARKER: &str = "<!-- OXICLOUD_ASSETS_START -->";
const END_MARKER: &str = "<!-- OXICLOUD_ASSETS_END -->";
const LAZY_MODULE_INPUTS: [(&str, &str); 4] = [
    ("search", "/js/search.js"),
    ("favorites", "/js/favorites.js"),
    ("recent", "/js/recent.js"),
    ("shared-view", "/js/components/sharedView.js"),
];

fn main() -> Result<()> {
    let settings = parse_args()?;

    if !settings.static_dir.exists() {
        bail!("static directory not found: {}", settings.static_dir.display());
    }

    println!("[assets] scanning HTML entries in {}", settings.static_dir.display());
    let mut pages = collect_pages(&settings.static_dir)?;
    if pages.is_empty() {
        bail!("No HTML files found in {}", settings.static_dir.display());
    }

    pages.sort_by(|a, b| a.html_name.cmp(&b.html_name));

    let recovered_inputs = recover_page_inputs_from_previous_manifest(&settings.dist_dir)?;
    for page in &mut pages {
        let needs_recovery = page.js.is_empty() && page.css.is_empty();
        if !needs_recovery {
            continue;
        }

        if let Some(recovered) = recovered_inputs.get(&page.html_name) {
            page.js = recovered.js.clone();
            page.css = recovered.css.clone();
            println!(
                "[assets] recovered inputs for {} from previous manifest (js: {}, css: {})",
                page.html_name,
                page.js.len(),
                page.css.len()
            );
            continue;
        }

        if page.has_dist_asset_refs {
            if let Some(fallback) = recover_page_inputs_from_known_layout(&page.html_name) {
                page.js = fallback.js;
                page.css = fallback.css;
                println!(
                    "[assets] recovered inputs for {} from built-in fallback (js: {}, css: {})",
                    page.html_name,
                    page.js.len(),
                    page.css.len()
                );
            }
        }
    }

    if settings.rewrite_html {
        let unrecoverable = pages
            .iter()
            .filter(|p| p.js.is_empty() && p.css.is_empty() && p.has_dist_asset_refs)
            .map(|p| p.html_name.clone())
            .collect::<Vec<_>>();

        if !unrecoverable.is_empty() {
            bail!(
                "Cannot rewrite HTML for pages without source assets or recoverable manifest inputs: {}",
                unrecoverable.join(", ")
            );
        }
    }

    let shared_js = find_shared_paths(pages.iter().map(|p| p.js.as_slice()));
    let shared_css = find_shared_paths(pages.iter().map(|p| p.css.as_slice()));

    println!(
        "[assets] found {} pages, shared js: {}, shared css: {}",
        pages.len(),
        shared_js.len(),
        shared_css.len()
    );

    if settings.dist_dir.exists() {
        fs::remove_dir_all(&settings.dist_dir)
            .with_context(|| format!("failed to clean {}", settings.dist_dir.display()))?;
    }
    fs::create_dir_all(settings.dist_dir.join("js"))?;
    fs::create_dir_all(settings.dist_dir.join("css"))?;

    let mut global_js_order = Vec::new();
    let mut global_css_order = Vec::new();
    push_unique_by_page_order(&pages, |p| &p.js, &mut global_js_order);
    push_unique_by_page_order(&pages, |p| &p.css, &mut global_css_order);

    let shared_js_inputs: Vec<String> = global_js_order
        .iter()
        .filter(|p| shared_js.contains(*p))
        .cloned()
        .collect();
    let shared_css_inputs: Vec<String> = global_css_order
        .iter()
        .filter(|p| shared_css.contains(*p))
        .cloned()
        .collect();

    let mut chunks = Vec::<ChunkBuild>::new();

    if !shared_js_inputs.is_empty() {
        chunks.push(build_chunk(
            &settings,
            AssetKind::Js,
            "shared",
            &shared_js_inputs,
        )?);
    }
    if !shared_css_inputs.is_empty() {
        chunks.push(build_chunk(
            &settings,
            AssetKind::Css,
            "shared",
            &shared_css_inputs,
        )?);
    }

    for (logical_suffix, input) in LAZY_MODULE_INPUTS {
        if !asset_exists(&settings.static_dir, input) {
            println!("[assets] lazy input not found, skipping: {input}");
            continue;
        }
        chunks.push(build_chunk(
            &settings,
            AssetKind::Js,
            &format!("lazy-{logical_suffix}"),
            &[input.to_string()],
        )?);
    }

    let mut page_manifest = BTreeMap::<String, PageManifest>::new();

    for page in &pages {
        let page_id = sanitize_stem(&page.html_name);

        let page_js_inputs: Vec<String> = page
            .js
            .iter()
            .filter(|p| !shared_js.contains(*p))
            .cloned()
            .collect();
        let page_css_inputs: Vec<String> = page
            .css
            .iter()
            .filter(|p| !shared_css.contains(*p))
            .cloned()
            .collect();

        let mut page_js_files = Vec::new();
        let mut page_css_files = Vec::new();

        if let Some(shared_js_chunk) = chunks
            .iter()
            .find(|c| matches!(c.kind, AssetKind::Js) && c.logical_name == "shared")
        {
            page_js_files.push(shared_js_chunk.output_web_path.clone());
        }

        if let Some(shared_css_chunk) = chunks
            .iter()
            .find(|c| matches!(c.kind, AssetKind::Css) && c.logical_name == "shared")
        {
            page_css_files.push(shared_css_chunk.output_web_path.clone());
        }

        if !page_js_inputs.is_empty() {
            let chunk = build_chunk(&settings, AssetKind::Js, &format!("page-{page_id}"), &page_js_inputs)?;
            page_js_files.push(chunk.output_web_path.clone());
            chunks.push(chunk);
        }

        if !page_css_inputs.is_empty() {
            let chunk = build_chunk(&settings, AssetKind::Css, &format!("page-{page_id}"), &page_css_inputs)?;
            page_css_files.push(chunk.output_web_path.clone());
            chunks.push(chunk);
        }

        page_manifest.insert(
            page.html_name.clone(),
            PageManifest {
                js: page_js_files,
                css: page_css_files,
            },
        );
    }

    let chunk_manifest = chunks
        .iter()
        .map(|c| ChunkManifest {
            kind: match c.kind {
                AssetKind::Js => "js".to_string(),
                AssetKind::Css => "css".to_string(),
            },
            logical_name: c.logical_name.clone(),
            file: c.output_web_path.clone(),
            inputs: c.inputs.clone(),
        })
        .collect::<Vec<_>>();

    let manifest = BuildManifest {
        version: 1,
        generated_by: "oxicloud-assets (swc_ecma_minifier)".to_string(),
        pages: page_manifest.clone(),
        chunks: chunk_manifest,
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    let manifest_path = settings.dist_dir.join("manifest.json");
    fs::write(&manifest_path, manifest_json)
        .with_context(|| format!("failed writing {}", manifest_path.display()))?;

    if settings.rewrite_html {
        println!("[assets] rewriting HTML to use bundled assets");
        for page in &pages {
            if page.js.is_empty() && page.css.is_empty() {
                println!(
                    "[assets] skipping rewrite for {} (no source assets detected)",
                    page.html_name
                );
                continue;
            }

            let bundles = page_manifest
                .get(&page.html_name)
                .ok_or_else(|| anyhow!("missing page bundle for {}", page.html_name))?;
            rewrite_html(&page.html_file, page, bundles)?;
        }
    }

    println!("[assets] done. Output: {}", settings.dist_dir.display());
    println!("[assets] manifest: {}", manifest_path.display());
    Ok(())
}

fn parse_args() -> Result<Settings> {
    let mut static_dir = PathBuf::from("static");
    let mut dist_dir = PathBuf::from("static/dist");
    let mut rewrite_html = false;

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--static-dir" => {
                i += 1;
                let value = args.get(i).ok_or_else(|| anyhow!("--static-dir expects a path"))?;
                static_dir = PathBuf::from(value);
            }
            "--dist-dir" => {
                i += 1;
                let value = args.get(i).ok_or_else(|| anyhow!("--dist-dir expects a path"))?;
                dist_dir = PathBuf::from(value);
            }
            "--rewrite-html" => rewrite_html = true,
            "--no-rewrite-html" => rewrite_html = false,
            "--help" | "-h" => {
                println!("oxicloud assets builder\n");
                println!("Options:");
                println!("  --static-dir <path>     Static root (default: static)");
                println!("  --dist-dir <path>       Output dist dir (default: static/dist)");
                println!("  --rewrite-html          Rewrite HTML entries with dist bundles");
                println!("  --no-rewrite-html       Keep HTML untouched (default)");
                std::process::exit(0);
            }
            other => bail!("Unknown argument: {other}"),
        }
        i += 1;
    }

    Ok(Settings {
        static_dir,
        dist_dir,
        rewrite_html,
    })
}

fn collect_pages(static_dir: &Path) -> Result<Vec<PageAssetsRaw>> {
    let mut pages = Vec::new();

    for entry in fs::read_dir(static_dir)
        .with_context(|| format!("failed reading {}", static_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("html") {
            continue;
        }

        let html_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow!("invalid html filename: {}", path.display()))?
            .to_string();

        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed reading {}", path.display()))?;

        let js = extract_script_sources(&content);
        let css = extract_css_hrefs(&content);
        let has_dist_asset_refs = has_dist_asset_refs(&content);

        pages.push(PageAssetsRaw {
            html_file: path,
            html_name,
            js,
            css,
            has_dist_asset_refs,
        });
    }

    Ok(pages)
}

fn extract_script_sources(html: &str) -> Vec<String> {
    let re = Regex::new(r#"(?is)<script\b[^>]*\bsrc\s*=\s*['\"]([^'\"]+)['\"][^>]*>\s*</script>"#)
        .expect("valid regex");

    re.captures_iter(html)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().trim().to_string()))
        .filter(|src| src.starts_with('/') && !src.starts_with("/dist/") && !src.starts_with("//"))
        .filter(|src| !src.starts_with("/http://") && !src.starts_with("/https://"))
        .collect()
}

fn extract_css_hrefs(html: &str) -> Vec<String> {
    let re = Regex::new(
        r#"(?is)<link\b[^>]*\brel\s*=\s*['\"]stylesheet['\"][^>]*\bhref\s*=\s*['\"]([^'\"]+)['\"][^>]*>"#,
    )
    .expect("valid regex");

    re.captures_iter(html)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().trim().to_string()))
        .filter(|href| href.starts_with('/') && !href.starts_with("/dist/") && !href.starts_with("//"))
        .collect()
}

fn has_dist_asset_refs(html: &str) -> bool {
    let script_re = Regex::new(r#"(?is)<script\b[^>]*\bsrc\s*=\s*['\"]([^'\"]+)['\"][^>]*>\s*</script>"#)
        .expect("valid script regex");

    if script_re
        .captures_iter(html)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().trim()))
        .any(|src| src.starts_with("/dist/"))
    {
        return true;
    }

    let css_re = Regex::new(
        r#"(?is)<link\b[^>]*\brel\s*=\s*['\"]stylesheet['\"][^>]*\bhref\s*=\s*['\"]([^'\"]+)['\"][^>]*>"#,
    )
    .expect("valid link regex");

    css_re
        .captures_iter(html)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().trim()))
        .any(|href| href.starts_with("/dist/"))
}

fn find_shared_paths<'a, I>(lists: I) -> HashSet<String>
where
    I: Iterator<Item = &'a [String]>,
{
    let mut counts = HashMap::<String, usize>::new();
    for list in lists {
        let uniq = list.iter().cloned().collect::<HashSet<_>>();
        for path in uniq {
            *counts.entry(path).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .map(|(path, _)| path)
        .collect()
}

fn push_unique_by_page_order<F>(pages: &[PageAssetsRaw], selector: F, out: &mut Vec<String>)
where
    F: Fn(&PageAssetsRaw) -> &[String],
{
    let mut seen = out.iter().cloned().collect::<HashSet<_>>();
    for page in pages {
        for item in selector(page) {
            if seen.insert(item.clone()) {
                out.push(item.clone());
            }
        }
    }
}

fn build_chunk(settings: &Settings, kind: AssetKind, logical_name: &str, inputs: &[String]) -> Result<ChunkBuild> {
    let source = match kind {
        AssetKind::Js => bundle_js_sources(&settings.static_dir, inputs)?,
        AssetKind::Css => bundle_css_sources(&settings.static_dir, inputs)?,
    };

    let minified = match kind {
        AssetKind::Js => minify_js(&source)?,
        AssetKind::Css => minify_css(&source),
    };

    let hash = short_hash(&minified);
    let ext = match kind {
        AssetKind::Js => "js",
        AssetKind::Css => "css",
    };
    let file_name = format!("{}.{}.{}", logical_name, hash, ext);

    let output_file_path = match kind {
        AssetKind::Js => settings.dist_dir.join("js").join(&file_name),
        AssetKind::Css => settings.dist_dir.join("css").join(&file_name),
    };

    fs::write(&output_file_path, minified)
        .with_context(|| format!("failed writing {}", output_file_path.display()))?;

    let output_web_path = match kind {
        AssetKind::Js => format!("/dist/js/{file_name}"),
        AssetKind::Css => format!("/dist/css/{file_name}"),
    };

    Ok(ChunkBuild {
        kind,
        logical_name: logical_name.to_string(),
        output_web_path,
        inputs: inputs.to_vec(),
    })
}

fn bundle_js_sources(static_dir: &Path, inputs: &[String]) -> Result<String> {
    let mut out = String::new();
    for input in inputs {
        let content = read_static_asset(static_dir, input)?;
        out.push_str("\n;");
        out.push_str("\n// source: ");
        out.push_str(input);
        out.push('\n');
        out.push_str(&content);
        out.push('\n');
    }
    Ok(out)
}

fn bundle_css_sources(static_dir: &Path, inputs: &[String]) -> Result<String> {
    let mut out = String::new();
    for input in inputs {
        let content = read_static_asset(static_dir, input)?;
        out.push_str("\n/* source: ");
        out.push_str(input);
        out.push_str(" */\n");
        out.push_str(&content);
        out.push('\n');
    }
    Ok(out)
}

fn read_static_asset(static_dir: &Path, web_path: &str) -> Result<String> {
    let rel = web_path.trim_start_matches('/');
    let path = static_dir.join(rel);
    fs::read_to_string(&path).with_context(|| format!("failed reading asset {}", path.display()))
}

fn asset_exists(static_dir: &Path, web_path: &str) -> bool {
    let rel = web_path.trim_start_matches('/');
    static_dir.join(rel).exists()
}

fn minify_js(source: &str) -> Result<String> {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("bundle.js".into()).into(), source.to_string());

    let syntax = Syntax::Es(EsSyntax {
        jsx: false,
        export_default_from: true,
        import_attributes: true,
        ..Default::default()
    });

    let mut parser = Parser::new(syntax, StringInput::from(&*fm), None);
    let module = parser
        .parse_module()
        .map_err(|err| anyhow!("SWC parse error: {err:?}"))?;

    let minified = GLOBALS.set(&Globals::new(), || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        let mut program = Program::Module(module);
        program.visit_mut_with(&mut resolver(unresolved_mark, top_level_mark, false));

        let mut minified = optimize(
            program,
            cm.clone(),
            None,
            None,
            &MinifyOptions {
                compress: Some(Default::default()),
                mangle: Some(MangleOptions {
                    top_level: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            &ExtraOptions {
                unresolved_mark,
                top_level_mark,
                mangle_name_cache: None,
            },
        );

        minified.visit_mut_with(&mut fixer(None));
        minified
    });

    let mut out = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: CodegenConfig::default().with_minify(true),
            comments: None,
            cm: cm.clone(),
            wr: JsWriter::new(cm.clone(), "\n", &mut out, None),
        };
        minified
            .emit_with(&mut emitter)
            .map_err(|err| anyhow!("SWC emit error: {err:?}"))?;
    }

    String::from_utf8(out).map_err(|err| anyhow!("Invalid UTF-8 in JS output: {err}"))
}

fn minify_css(source: &str) -> String {
    let comments = Regex::new(r"(?s)/\*.*?\*/").expect("valid css comment regex");
    let ws = Regex::new(r"\s+").expect("valid whitespace regex");
    let around = Regex::new(r"\s*([{}:;,>])\s*").expect("valid token regex");

    let mut out = comments.replace_all(source, "").to_string();
    out = ws.replace_all(&out, " ").to_string();
    out = around.replace_all(&out, "$1").to_string();
    out = out.replace(";}", "}");
    out.trim().to_string()
}

fn short_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let digest = hasher.finalize();
    hex::encode(digest)[..16].to_string()
}

fn sanitize_stem(html_name: &str) -> String {
    html_name
        .trim_end_matches(".html")
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>()
}

fn rewrite_html(html_path: &Path, raw: &PageAssetsRaw, manifest: &PageManifest) -> Result<()> {
    let original = fs::read_to_string(html_path)
        .with_context(|| format!("failed reading {}", html_path.display()))?;

    let mut updated = remove_managed_block(&original);

    let js_set = raw.js.iter().cloned().collect::<HashSet<_>>();
    let css_set = raw.css.iter().cloned().collect::<HashSet<_>>();

    let script_re = Regex::new(r#"(?is)<script\b[^>]*\bsrc\s*=\s*['\"]([^'\"]+)['\"][^>]*>\s*</script>\s*"#)
        .expect("valid script regex");
    updated = script_re
        .replace_all(&updated, |caps: &regex::Captures| {
            let Some(src) = caps.get(1).map(|m| m.as_str()) else {
                return caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string();
            };
            if js_set.contains(src) {
                String::new()
            } else {
                caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string()
            }
        })
        .to_string();

    let css_re = Regex::new(
        r#"(?is)<link\b[^>]*\brel\s*=\s*['\"]stylesheet['\"][^>]*\bhref\s*=\s*['\"]([^'\"]+)['\"][^>]*>\s*"#,
    )
    .expect("valid link regex");
    updated = css_re
        .replace_all(&updated, |caps: &regex::Captures| {
            let Some(href) = caps.get(1).map(|m| m.as_str()) else {
                return caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string();
            };
            if css_set.contains(href) {
                String::new()
            } else {
                caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string()
            }
        })
        .to_string();

    let mut block = String::new();
    block.push_str("\n");
    block.push_str(START_MARKER);
    block.push('\n');
    for href in &manifest.css {
        block.push_str(&format!("<link rel=\"stylesheet\" href=\"{}\">", href));
        block.push('\n');
    }
    for src in &manifest.js {
        block.push_str(&format!("<script defer src=\"{}\"></script>", src));
        block.push('\n');
    }
    block.push_str(END_MARKER);
    block.push_str("\n");

    let needle = "</head>";
    let head_index = updated
        .find(needle)
        .ok_or_else(|| anyhow!("no </head> tag in {}", html_path.display()))?;

    updated.insert_str(head_index, &block);

    fs::write(html_path, updated).with_context(|| format!("failed writing {}", html_path.display()))?;
    Ok(())
}

fn remove_managed_block(html: &str) -> String {
    if let (Some(start), Some(end)) = (html.find(START_MARKER), html.find(END_MARKER)) {
        let mut out = String::with_capacity(html.len());
        out.push_str(&html[..start]);
        let end_idx = end + END_MARKER.len();
        out.push_str(&html[end_idx..]);
        out
    } else {
        html.to_string()
    }
}

fn recover_page_inputs_from_previous_manifest(dist_dir: &Path) -> Result<HashMap<String, RecoveredPageInputs>> {
    let manifest_path = dist_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Ok(HashMap::new());
    }

    let manifest_raw = fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed reading {}", manifest_path.display()))?;

    let manifest: BuildManifest = serde_json::from_str(&manifest_raw)
        .with_context(|| format!("failed parsing {}", manifest_path.display()))?;

    let chunk_by_file = manifest
        .chunks
        .iter()
        .map(|chunk| (chunk.file.as_str(), chunk))
        .collect::<HashMap<_, _>>();

    let mut recovered = HashMap::<String, RecoveredPageInputs>::new();

    for (page_name, page_assets) in &manifest.pages {
        let mut js = Vec::<String>::new();
        let mut css = Vec::<String>::new();

        for file in &page_assets.js {
            if let Some(chunk) = chunk_by_file.get(file.as_str()) {
                if chunk.kind == "js" {
                    for input in &chunk.inputs {
                        if !js.contains(input) {
                            js.push(input.clone());
                        }
                    }
                }
            }
        }

        for file in &page_assets.css {
            if let Some(chunk) = chunk_by_file.get(file.as_str()) {
                if chunk.kind == "css" {
                    for input in &chunk.inputs {
                        if !css.contains(input) {
                            css.push(input.clone());
                        }
                    }
                }
            }
        }

        if !js.is_empty() || !css.is_empty() {
            recovered.insert(page_name.clone(), RecoveredPageInputs { js, css });
        }
    }

    Ok(recovered)
}

fn recover_page_inputs_from_known_layout(html_name: &str) -> Option<RecoveredPageInputs> {
    let inputs = match html_name {
        "index.html" => RecoveredPageInputs {
            js: vec![
                "/js/icons.js".to_string(),
                "/js/i18n.js".to_string(),
                "/js/languageSelector.js".to_string(),
                "/js/fileSharing.js".to_string(),
                "/js/notifications.js".to_string(),
                "/js/modal.js".to_string(),
                "/js/ui.js".to_string(),
                "/js/contextMenus.js".to_string(),
                "/js/fileOperations.js".to_string(),
                "/js/multiSelect.js".to_string(),
                "/js/inlineViewer.js".to_string(),
                "/js/app.js".to_string(),
            ],
            css: vec![
                "/css/style.css".to_string(),
                "/css/inlineViewer.css".to_string(),
                "/css/favorites.css".to_string(),
                "/css/recent.css".to_string(),
            ],
        },
        "login.html" => RecoveredPageInputs {
            js: vec![
                "/js/icons.js".to_string(),
                "/js/i18n.js".to_string(),
                "/js/languageSelector.js".to_string(),
                "/js/auth.js".to_string(),
            ],
            css: vec!["/css/style.css".to_string(), "/css/auth.css".to_string()],
        },
        "admin.html" => RecoveredPageInputs {
            js: vec![
                "/js/icons.js".to_string(),
                "/js/i18n.js".to_string(),
                "/js/languageSelector.js".to_string(),
                "/js/admin.js".to_string(),
            ],
            css: vec!["/css/style.css".to_string(), "/css/admin.css".to_string()],
        },
        "profile.html" => RecoveredPageInputs {
            js: vec![
                "/js/icons.js".to_string(),
                "/js/i18n.js".to_string(),
                "/js/languageSelector.js".to_string(),
            ],
            css: vec!["/css/style.css".to_string()],
        },
        "shared.html" => RecoveredPageInputs {
            js: vec![
                "/js/icons.js".to_string(),
                "/js/i18n.js".to_string(),
                "/js/languageSelector.js".to_string(),
                "/js/shared.js".to_string(),
            ],
            css: vec!["/css/style.css".to_string()],
        },
        _ => return None,
    };

    Some(inputs)
}
