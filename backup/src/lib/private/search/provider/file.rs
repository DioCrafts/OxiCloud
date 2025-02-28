use std::path::{Path, PathBuf};

struct SearchResult {
    name: String,
    text: String,
    link: String,
    type_: String,
    container: String,
}

trait SearchProvider {
    fn search(&self, query: &str) -> Vec<SearchResult>;
}

struct FileSearchProvider {
    l10n: L10n,
}

struct FileData {
    path: String,
    mimetype: String,
    mimepart: String,
}

struct L10n;

impl L10n {
    fn get(module: &str) -> Self {
        Self
    }

    fn t(&self, text: &str) -> String {
        text.to_string()
    }
}

struct Filesystem;

impl Filesystem {
    fn search(query: &str, include_hidden: bool) -> Vec<FileData> {
        // Implementación simulada
        vec![]
    }
}

struct Helper;

impl Helper {
    fn link_to(app: &str, file: &str, params: Vec<(&str, &str)>) -> String {
        // Implementación simulada
        format!("/{}/{}", app, file)
    }

    fn link_to_route(route: &str, params: Vec<(&str, &str)>) -> String {
        // Implementación simulada
        route.to_string()
    }
}

impl FileSearchProvider {
    fn new() -> Self {
        Self {
            l10n: L10n::get("lib"),
        }
    }
}

impl SearchProvider for FileSearchProvider {
    fn search(&self, query: &str) -> Vec<SearchResult> {
        let files = Filesystem::search(query, true);
        let mut results = Vec::new();

        for file_data in files {
            let path = &file_data.path;
            let mime = &file_data.mimetype;

            let path_obj = Path::new(path);
            let name = path_obj.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            
            let container = path_obj.parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            
            let text = String::new();
            let mut skip = false;
            
            let (link, type_) = if mime == "httpd/unix-directory" {
                let link_params = vec![("dir", path.as_str())];
                let link = Helper::link_to("files", "index.php", link_params);
                let type_ = self.l10n.t("Files");
                (link, type_)
            } else {
                let link_params = vec![("file", path.as_str())];
                let link = Helper::link_to_route("download", link_params);
                let mime_base = &file_data.mimepart;
                
                let type_ = match mime_base.as_str() {
                    "audio" => {
                        skip = true;
                        String::new()
                    },
                    "text" => self.l10n.t("Text"),
                    "image" => self.l10n.t("Images"),
                    _ => {
                        if mime == "application/xml" {
                            self.l10n.t("Text")
                        } else {
                            self.l10n.t("Files")
                        }
                    }
                };
                
                (link, type_)
            };
            
            if !skip {
                results.push(SearchResult {
                    name,
                    text,
                    link,
                    type_,
                    container,
                });
            }
        }
        
        results
    }
}