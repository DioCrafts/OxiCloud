use askama::Template;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "public.html")]
pub struct PublicTemplate<'a> {
    dir: &'a str,
    download_url: &'a str,
    filename: &'a str,
    mimetype: &'a str,
    theme_name: &'a str,
    theme_logo_claim: &'a str,
    display_name: &'a str,
    is_folder: bool,
    folder_content: Option<Cow<'a, str>>,
    allow_zip_download: bool,
    allow_public_upload_enabled: bool,
    request_token: &'a str,
    dir_token: &'a str,
    upload_max_filesize: i64,
    upload_max_human_filesize: &'a str,
    directory_path: &'a str,
    is_mime_supported: bool,
    l: &'a Translator,
    theme: &'a Theme,
}

pub struct Translator {
    translations: HashMap<String, String>,
}

impl Translator {
    pub fn t(&self, key: &str, args: &[&str]) -> String {
        let mut result = self.translations.get(key).cloned().unwrap_or_else(|| key.to_string());
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("%{}", i+1), arg);
        }
        result
    }
}

pub struct Theme {
    name: String,
    logo_claim: String,
    long_footer: String,
}

impl Theme {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_logo_claim(&self) -> &str {
        &self.logo_claim
    }

    pub fn get_long_footer(&self) -> &str {
        &self.long_footer
    }
}

pub fn link_to(app: &str, file: &str) -> String {
    format!("/{}/{}", app, file)
}

pub fn image_path(app: &str, file: &str) -> String {
    format!("/core/img/{}/{}", app, file)
}

pub fn handle_public_template(
    params: HashMap<String, String>,
    theme: &Theme,
    translator: &Translator,
) -> Result<String, askama::Error> {
    let dir = params.get("dir").unwrap_or(&String::new());
    let download_url = params.get("downloadURL").unwrap_or(&String::new());
    let filename = params.get("filename").unwrap_or(&String::new());
    let mimetype = params.get("mimetype").unwrap_or(&String::new());
    let display_name = params.get("displayName").unwrap_or(&String::new());
    let is_folder = params.contains_key("folder");
    let folder_content = if is_folder {
        params.get("folder").map(|s| Cow::Borrowed(s.as_str()))
    } else {
        None
    };
    
    let allow_zip_download = params.get("allowZipDownload")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);
        
    let allow_public_upload_enabled = params.get("allowPublicUploadEnabled")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);
        
    let request_token = params.get("requesttoken").unwrap_or(&String::new());
    let dir_token = params.get("dirToken").unwrap_or(&String::new());
    let upload_max_filesize = params.get("uploadMaxFilesize")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(-1);
    let upload_max_human_filesize = params.get("uploadMaxHumanFilesize").unwrap_or(&String::new());
    let directory_path = params.get("directory_path").unwrap_or(&String::new());
    
    // This is a simplification - in a real app we would have proper mime type checking
    let is_mime_supported = true;
    
    let template = PublicTemplate {
        dir,
        download_url,
        filename,
        mimetype,
        theme_name: theme.get_name(),
        theme_logo_claim: theme.get_logo_claim(),
        display_name,
        is_folder,
        folder_content,
        allow_zip_download,
        allow_public_upload_enabled,
        request_token,
        dir_token,
        upload_max_filesize,
        upload_max_human_filesize,
        directory_path,
        is_mime_supported,
        l: translator,
        theme,
    };
    
    template.render()
}