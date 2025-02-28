use std::path::Path;

/**
 * Default strings and values which differ between the enterprise and the
 * community edition. Use the get methods to always get the right strings.
 */

pub struct OcDefaults {
    theme: Option<OcTheme>,
    l: OcL10n,

    default_entity: String,
    default_name: String,
    default_title: String,
    default_base_url: String,
    default_sync_client_url: String,
    default_doc_base_url: String,
    default_slogan: String,
    default_logo_claim: String,
}

impl OcDefaults {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load theme if exists
        let theme_name = OcUtil::get_theme();
        if !theme_name.is_empty() {
            let theme_path = Path::new(OC::server_root())
                .join("themes")
                .join(&theme_name)
                .join("defaults.php");
            
            if theme_path.exists() {
                // In Rust we would import the module instead of requiring it
                // This is a stub for the PHP require_once functionality
            }
        }

        let l = OcL10n::get("core")?;
        
        let mut defaults = Self {
            theme: None,
            l,
            default_entity: "ownCloud".to_string(), /* e.g. company name, used for footers and copyright notices */
            default_name: "ownCloud".to_string(), /* short name, used when referring to the software */
            default_title: "ownCloud".to_string(), /* can be a longer name, for titles */
            default_base_url: "http://owncloud.org".to_string(),
            default_sync_client_url: "http://owncloud.org/sync-clients/".to_string(),
            default_doc_base_url: "http://doc.owncloud.org".to_string(),
            default_slogan: l.t("web services under your control"),
            default_logo_claim: "".to_string(),
        };
        
        // Check if theme class exists and initialize it
        if OcTheme::exists() {
            defaults.theme = Some(OcTheme::new()?);
        }
        
        Ok(defaults)
    }

    fn theme_exist(&self, method: &str) -> bool {
        if OcUtil::get_theme() != "" && self.theme.as_ref().map_or(false, |theme| theme.has_method(method)) {
            true
        } else {
            false
        }
    }

    pub fn get_base_url(&self) -> String {
        if self.theme_exist("get_base_url") {
            self.theme.as_ref().unwrap().get_base_url()
        } else {
            self.default_base_url.clone()
        }
    }

    pub fn get_sync_client_url(&self) -> String {
        if self.theme_exist("get_sync_client_url") {
            self.theme.as_ref().unwrap().get_sync_client_url()
        } else {
            self.default_sync_client_url.clone()
        }
    }

    pub fn get_doc_base_url(&self) -> String {
        if self.theme_exist("get_doc_base_url") {
            self.theme.as_ref().unwrap().get_doc_base_url()
        } else {
            self.default_doc_base_url.clone()
        }
    }

    pub fn get_title(&self) -> String {
        if self.theme_exist("get_title") {
            self.theme.as_ref().unwrap().get_title()
        } else {
            self.default_title.clone()
        }
    }

    pub fn get_name(&self) -> String {
        if self.theme_exist("get_name") {
            self.theme.as_ref().unwrap().get_name()
        } else {
            self.default_name.clone()
        }
    }

    pub fn get_entity(&self) -> String {
        if self.theme_exist("get_entity") {
            self.theme.as_ref().unwrap().get_entity()
        } else {
            self.default_entity.clone()
        }
    }

    pub fn get_slogan(&self) -> String {
        if self.theme_exist("get_slogan") {
            self.theme.as_ref().unwrap().get_slogan()
        } else {
            self.default_slogan.clone()
        }
    }

    pub fn get_logo_claim(&self) -> String {
        if self.theme_exist("get_logo_claim") {
            self.theme.as_ref().unwrap().get_logo_claim()
        } else {
            self.default_logo_claim.clone()
        }
    }

    pub fn get_short_footer(&self) -> String {
        if self.theme_exist("get_short_footer") {
            self.theme.as_ref().unwrap().get_short_footer()
        } else {
            format!(
                "<a href=\"{}\" target=\"_blank\">{}</a> – {}", 
                self.get_base_url(), 
                self.get_entity(), 
                self.get_slogan()
            )
        }
    }

    pub fn get_long_footer(&self) -> String {
        if self.theme_exist("get_long_footer") {
            self.theme.as_ref().unwrap().get_long_footer()
        } else {
            self.get_short_footer()
        }
    }
}

// These are stubs for the external types referenced in the code
struct OcL10n;
impl OcL10n {
    fn get(domain: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self)
    }
    
    fn t(&self, text: &str) -> String {
        text.to_string()
    }
}

struct OcTheme;
impl OcTheme {
    fn exists() -> bool {
        // Implementation would check if the theme class exists
        false
    }
    
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self)
    }
    
    fn has_method(&self, _method: &str) -> bool {
        false
    }
    
    fn get_base_url(&self) -> String {
        "".to_string()
    }
    
    fn get_sync_client_url(&self) -> String {
        "".to_string()
    }
    
    fn get_doc_base_url(&self) -> String {
        "".to_string()
    }
    
    fn get_title(&self) -> String {
        "".to_string()
    }
    
    fn get_name(&self) -> String {
        "".to_string()
    }
    
    fn get_entity(&self) -> String {
        "".to_string()
    }
    
    fn get_slogan(&self) -> String {
        "".to_string()
    }
    
    fn get_logo_claim(&self) -> String {
        "".to_string()
    }
    
    fn get_short_footer(&self) -> String {
        "".to_string()
    }
    
    fn get_long_footer(&self) -> String {
        "".to_string()
    }
}

struct OcUtil;
impl OcUtil {
    fn get_theme() -> String {
        "".to_string()
    }
}

struct OC;
impl OC {
    fn server_root() -> &'static str {
        ""
    }
}