use std::collections::HashMap;

/**
 * Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

pub struct TemplateLayout {
    template: Template,
}

impl TemplateLayout {
    pub fn new(render_as: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Decide which page we show
        let template = if render_as == "user" {
            let mut template = Template::new("core", "layout.user")?;
            
            if vec!["settings", "admin", "help"].contains(&App::get_current_app()) {
                template.assign("bodyid", "body-settings");
            } else {
                template.assign("bodyid", "body-user");
            }

            // Update notification
            if Config::get_value("updatechecker", true) {
                let data = Updater::check()?;
                let current_user = User::get_user();
                
                if let Some(version) = data.get("version") {
                    if !version.is_empty() && !version.is_array() && User::is_admin_user(&current_user) {
                        template.assign("updateAvailable", true);
                        template.assign("updateVersion", data.get("versionstring").unwrap_or(&String::new()));
                        template.assign("updateLink", data.get("web").unwrap_or(&String::new()));
                    } else {
                        template.assign("updateAvailable", false); // No update available or not an admin user
                    }
                } else {
                    template.assign("updateAvailable", false);
                }
            } else {
                template.assign("updateAvailable", false); // Update check is disabled
            }

            // Add navigation entry
            template.assign("application", "");
            let navigation = App::get_navigation();
            template.assign("navigation", &navigation);
            template.assign("settingsnavigation", &App::get_settings_navigation());
            
            for entry in &navigation {
                if entry.get("active").unwrap_or(&false) == &true {
                    template.assign("application", entry.get("name").unwrap_or(&String::new()));
                    break;
                }
            }
            
            let user_displayname = User::get_display_name();
            template.assign("user_displayname", &user_displayname);
            template.assign("user_uid", &User::get_user());
            template.assign("enableAvatars", Config::get_value("enable_avatars", true));
            
            template
        } else if render_as == "guest" || render_as == "error" {
            Template::new("core", "layout.guest")?
        } else {
            Template::new("core", "layout.base")?
        };
        
        let mut layout = Self { template };
        
        let version_parameter = format!("?v={}", md5::compute(Util::get_version().join("")).to_string());
        
        // Add the js files
        let js_files = Self::find_javascript_files(&Util::get_scripts());
        layout.template.assign("jsfiles", Vec::<String>::new());
        
        if Config::get_value("installed", false) && render_as != "error" {
            layout.template.append("jsfiles", &format!("{}{}", Helper::link_to_route("js_config"), version_parameter));
        }
        
        if !Util::get_core_scripts().is_empty() {
            layout.template.append("jsfiles", &format!("{}{}", Helper::link_to_remote_base("core.js", false), version_parameter));
        }
        
        for info in js_files {
            let root = &info[0];
            let web = &info[1];
            let file = &info[2];
            layout.template.append("jsfiles", &format!("{}/{}{}", web, file, version_parameter));
        }

        // Add the css files
        let css_files = Self::find_stylesheet_files(&Util::get_styles());
        layout.template.assign("cssfiles", Vec::<String>::new());
        
        if !Util::get_core_styles().is_empty() {
            layout.template.append("cssfiles", &format!("{}{}", Helper::link_to_remote_base("core.css", false), version_parameter));
        }
        
        for info in css_files {
            let root = &info[0];
            let web = &info[1];
            let file = &info[2];
            layout.template.append("cssfiles", &format!("{}/{}{}", web, file, version_parameter));
        }
        
        Ok(layout)
    }

    pub fn find_stylesheet_files(styles: &[String]) -> Vec<[String; 3]> {
        // Read the selected theme from the config file
        let theme = Util::get_theme();

        // Read the detected formfactor and use the right file name.
        let fext = Self::get_form_factor_extension();

        let server_map = [(ServerRoot::get(), WebRoot::get())];
        let third_party_map = [(ThirdPartyRoot::get(), ThirdPartyWebRoot::get())];
        
        let mut locator = CSSResourceLocator::new(
            &theme, 
            &fext,
            &server_map,
            &third_party_map
        );
        
        locator.find(styles);
        locator.get_resources()
    }

    pub fn find_javascript_files(scripts: &[String]) -> Vec<[String; 3]> {
        // Read the selected theme from the config file
        let theme = Util::get_theme();

        // Read the detected formfactor and use the right file name.
        let fext = Self::get_form_factor_extension();

        let server_map = [(ServerRoot::get(), WebRoot::get())];
        let third_party_map = [(ThirdPartyRoot::get(), ThirdPartyWebRoot::get())];
        
        let mut locator = JSResourceLocator::new(
            &theme, 
            &fext,
            &server_map,
            &third_party_map
        );
        
        locator.find(scripts);
        locator.get_resources()
    }
    
    fn get_form_factor_extension() -> String {
        // Implementation would depend on how form factor detection works
        String::new()
    }
}

// Mock types to complete the translation
struct Template {
    // Fields would be defined based on OC_Template implementation
}

impl Template {
    fn new(app: &str, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Implementation would create a template based on the app and template name
        Ok(Self {})
    }
    
    fn assign(&mut self, key: &str, value: impl Into<TemplateValue>) {
        // Implementation would assign a value to a key in the template
    }
    
    fn append(&mut self, key: &str, value: &str) {
        // Implementation would append a value to an array in the template
    }
}

enum TemplateValue {
    String(String),
    Bool(bool),
    Vec(Vec<HashMap<String, TemplateValue>>),
    // Other types as needed
}

impl From<&str> for TemplateValue {
    fn from(s: &str) -> Self {
        TemplateValue::String(s.to_owned())
    }
}

impl From<bool> for TemplateValue {
    fn from(b: bool) -> Self {
        TemplateValue::Bool(b)
    }
}

struct App;
impl App {
    fn get_current_app() -> &'static str {
        // Implementation would return the current app
        ""
    }
    
    fn get_navigation() -> Vec<HashMap<String, TemplateValue>> {
        // Implementation would return the navigation entries
        Vec::new()
    }
    
    fn get_settings_navigation() -> Vec<HashMap<String, TemplateValue>> {
        // Implementation would return the settings navigation entries
        Vec::new()
    }
}

struct Config;
impl Config {
    fn get_value(key: &str, default: impl Into<TemplateValue>) -> TemplateValue {
        // Implementation would get a configuration value
        default.into()
    }
}

struct Updater;
impl Updater {
    fn check() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // Implementation would check for updates
        Ok(HashMap::new())
    }
}

struct User;
impl User {
    fn get_user() -> String {
        // Implementation would return the current user
        String::new()
    }
    
    fn is_admin_user(user: &str) -> bool {
        // Implementation would check if the user is an admin
        false
    }
    
    fn get_display_name() -> String {
        // Implementation would return the display name of the current user
        String::new()
    }
}

struct Helper;
impl Helper {
    fn link_to_route(route: &str) -> String {
        // Implementation would return a link to a route
        String::new()
    }
    
    fn link_to_remote_base(file: &str, allow_caching: bool) -> String {
        // Implementation would return a link to a remote base file
        String::new()
    }
}

struct Util;
impl Util {
    fn get_theme() -> String {
        // Implementation would return the current theme
        String::new()
    }
    
    fn get_version() -> Vec<String> {
        // Implementation would return the current version
        Vec::new()
    }
    
    fn get_scripts() -> Vec<String> {
        // Implementation would return the scripts to be included
        Vec::new()
    }
    
    fn get_styles() -> Vec<String> {
        // Implementation would return the styles to be included
        Vec::new()
    }
    
    fn get_core_scripts() -> Vec<String> {
        // Implementation would return the core scripts
        Vec::new()
    }
    
    fn get_core_styles() -> Vec<String> {
        // Implementation would return the core styles
        Vec::new()
    }
}

struct ServerRoot;
impl ServerRoot {
    fn get() -> String {
        // Implementation would return the server root
        String::new()
    }
}

struct WebRoot;
impl WebRoot {
    fn get() -> String {
        // Implementation would return the web root
        String::new()
    }
}

struct ThirdPartyRoot;
impl ThirdPartyRoot {
    fn get() -> String {
        // Implementation would return the third party root
        String::new()
    }
}

struct ThirdPartyWebRoot;
impl ThirdPartyWebRoot {
    fn get() -> String {
        // Implementation would return the third party web root
        String::new()
    }
}

struct CSSResourceLocator {
    // Fields would be defined based on CSSResourceLocator implementation
}

impl CSSResourceLocator {
    fn new(
        theme: &str, 
        fext: &str, 
        server_map: &[(String, String)], 
        third_party_map: &[(String, String)]
    ) -> Self {
        // Implementation would create a CSS resource locator
        Self {}
    }
    
    fn find(&mut self, styles: &[String]) {
        // Implementation would find CSS resources
    }
    
    fn get_resources(&self) -> Vec<[String; 3]> {
        // Implementation would return the found resources
        Vec::new()
    }
}

struct JSResourceLocator {
    // Fields would be defined based on JSResourceLocator implementation
}

impl JSResourceLocator {
    fn new(
        theme: &str, 
        fext: &str, 
        server_map: &[(String, String)], 
        third_party_map: &[(String, String)]
    ) -> Self {
        // Implementation would create a JS resource locator
        Self {}
    }
    
    fn find(&mut self, scripts: &[String]) {
        // Implementation would find JS resources
    }
    
    fn get_resources(&self) -> Vec<[String; 3]> {
        // Implementation would return the found resources
        Vec::new()
    }
}