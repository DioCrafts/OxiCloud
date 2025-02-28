use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

use crate::minimizer::Minimizer;
use crate::mediawiki::cssmin::CSSMin;

pub struct CssMinimizer {
    content_type: String,
}

impl Default for CssMinimizer {
    fn default() -> Self {
        Self {
            content_type: "text/css".to_string(),
        }
    }
}

impl Minimizer for CssMinimizer {
    fn get_content_type(&self) -> &str {
        &self.content_type
    }

    fn minimize_files(&self, files: &[([String; 3])]) -> Result<String, Box<dyn Error>> {
        let mut css_out = String::new();
        let webroot = crate::OC::get_webroot();
        
        for file_info in files {
            let file_path = format!("{}/{}", file_info[0], file_info[2]);
            css_out.push_str(&format!("/* {} */\n", file_path));
            
            let css = fs::read_to_string(&file_path)?;
            
            let mut in_root = None;
            for app_root in crate::OC::get_apps_roots() {
                if file_path.starts_with(&format!("{}/", app_root.path)) {
                    let url = app_root.url.trim_end_matches('/');
                    in_root = Some(format!("{}{}", webroot, url));
                    break;
                }
            }
            
            let css = if let Some(root) = in_root {
                css.replace("%appswebroot%", &root).replace("%webroot%", &webroot)
            } else {
                css
            };
            
            let remote = format!("{}/{}", file_info[1], Path::new(&file_info[2]).parent().unwrap_or_else(|| Path::new("")));
            let dir_name = Path::new(&file_path).parent().unwrap_or_else(|| Path::new("")).to_string_lossy().to_string();
            
            css_out.push_str(&CSSMin::remap(&css, &dir_name, &remote, true)?);
        }
        
        if !cfg!(debug_assertions) {
            css_out = CSSMin::minify(&css_out)?;
        }
        
        Ok(css_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_css_minimizer() {
        // Add tests here
    }
}