use askama::Template;
use std::path::Path;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
struct FilesIndexTemplate {
    breadcrumb: String,
    is_creatable: bool,
    upload_max_human_filesize: String,
    upload_max_filesize: i64,
    dir: String,
    trash: bool,
    trash_empty: bool,
    permissions: u32,
    is_public: bool,
    empty_content: bool,
    disable_sharing: bool,
    public_upload_enabled: bool,
    file_header: bool,
    allow_zip_download: bool,
    file_list: String,
    ajax_load: bool,
    used_space_percent: u32,
    encrypted_files: bool,
    encryption_init_status: String,
    mail_notification_enabled: bool,
    allow_share_with_link: bool,
}

// Constantes para los permisos
const PERMISSION_DELETE: u32 = 0x00000004;

pub struct L10n {
    translations: HashMap<String, String>,
}

impl L10n {
    pub fn new() -> Self {
        // Inicialización del sistema de traducción
        Self {
            translations: HashMap::new(),
        }
    }

    pub fn t(&self, text: &str) -> String {
        self.translations.get(text).cloned().unwrap_or_else(|| text.to_string())
    }
}

pub struct MimeType;

impl MimeType {
    pub fn icon(mime_type: &str) -> String {
        // Implementación para obtener el icono del tipo MIME
        format!("/core/img/filetypes/{}.svg", mime_type.replace('/', "-"))
    }
}

pub struct Util;

impl Util {
    pub fn link_to(app: &str, file: &str) -> String {
        // Implementación para generar enlaces
        format!("/{}/ajax/{}", app, file)
    }

    pub fn image_path(app: &str, file: &str) -> String {
        // Implementación para obtener la ruta de la imagen
        format!("/{}/img/{}", app, file)
    }
}

pub fn render_files_index(context: HashMap<String, serde_json::Value>) -> String {
    let l = L10n::new();
    
    let template = FilesIndexTemplate {
        breadcrumb: context.get("breadcrumb").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        is_creatable: context.get("isCreatable").and_then(|v| v.as_bool()).unwrap_or(false),
        upload_max_human_filesize: context.get("uploadMaxHumanFilesize").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        upload_max_filesize: context.get("uploadMaxFilesize").and_then(|v| v.as_i64()).unwrap_or(-1),
        dir: context.get("dir").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        trash: context.get("trash").and_then(|v| v.as_bool()).unwrap_or(false),
        trash_empty: context.get("trashEmpty").and_then(|v| v.as_bool()).unwrap_or(false),
        permissions: context.get("permissions").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        is_public: context.get("isPublic").and_then(|v| v.as_bool()).unwrap_or(false),
        empty_content: context.get("emptyContent").and_then(|v| v.as_bool()).unwrap_or(false),
        disable_sharing: context.get("disableSharing").and_then(|v| v.as_bool()).unwrap_or(false),
        public_upload_enabled: context.get("publicUploadEnabled").and_then(|v| v.as_bool()).unwrap_or(false),
        file_header: context.get("fileHeader").and_then(|v| v.as_bool()).unwrap_or(true),
        allow_zip_download: context.get("allowZipDownload").and_then(|v| v.as_bool()).unwrap_or(false),
        file_list: context.get("fileList").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        ajax_load: context.get("ajaxLoad").and_then(|v| v.as_bool()).unwrap_or(false),
        used_space_percent: context.get("usedSpacePercent").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        encrypted_files: context.get("encryptedFiles").and_then(|v| v.as_bool()).unwrap_or(false),
        encryption_init_status: context.get("encryptionInitStatus").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        mail_notification_enabled: context.get("mailNotificationEnabled").and_then(|v| v.as_bool()).unwrap_or(false),
        allow_share_with_link: context.get("allowShareWithLink").and_then(|v| v.as_bool()).unwrap_or(false),
    };

    template.render().unwrap_or_else(|e| format!("Error rendering template: {}", e))
}