use askama::Template;
use std::path::PathBuf;

/// Template for LDAP settings controls part
#[derive(Template)]
#[template(path = "ldap/part.settingcontrols.html")]
pub struct LdapSettingControlsTemplate<'a> {
    pub save_text: &'a str,
    pub test_configuration_text: &'a str,
    pub help_text: &'a str,
    pub doc_base_url: &'a str,
    pub info_icon_path: &'a str,
}

impl<'a> LdapSettingControlsTemplate<'a> {
    pub fn new(
        l: &'a dyn Translate, 
        theme: &'a dyn Theme,
        util: &'a dyn Util,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let info_icon_path = util.image_path("", "actions/info.png")?;
        
        Ok(Self {
            save_text: l.t("Save"),
            test_configuration_text: l.t("Test Configuration"),
            help_text: l.t("Help"),
            doc_base_url: &theme.get_doc_base_url(),
            info_icon_path: &info_icon_path,
        })
    }
}

/// Traits representing the necessary interfaces
pub trait Translate {
    fn t(&self, text: &str) -> &str;
}

pub trait Theme {
    fn get_doc_base_url(&self) -> String;
}

pub trait Util {
    fn image_path(&self, app: &str, file: &str) -> Result<String, Box<dyn std::error::Error>>;
}