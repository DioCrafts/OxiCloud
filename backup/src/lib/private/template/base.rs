use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Base template class for rendering templates
pub struct Base {
    template: String,
    vars: HashMap<String, Value>,
    l10n: L10n,
    theme: String,
}

/// Value types that can be stored in template variables
#[derive(Clone)]
pub enum Value {
    String(String),
    Array(Vec<Value>),
    // Other types could be added as needed
}

/// L10n object for internationalization
pub struct L10n {
    // Implementation details would go here
}

impl Base {
    /// Create a new template instance
    pub fn new(template: String, request_token: String, l10n: L10n, theme: String) -> Self {
        let mut vars = HashMap::new();
        vars.insert("requesttoken".to_string(), Value::String(request_token));
        
        Self {
            template,
            vars,
            l10n,
            theme,
        }
    }

    /// Get template directories for an app
    pub fn get_app_template_dirs(&self, theme: &str, app: &str, server_root: &str, app_dir: &str) -> Vec<String> {
        if Path::new(&format!("{}/templates/", app_dir)).exists() {
            vec![
                format!("{}/themes/{}/apps/{}/templates/", server_root, theme, app),
                format!("{}/templates/", app_dir),
            ]
        } else {
            vec![
                format!("{}/themes/{}/{}/templates/", server_root, theme, app),
                format!("{}/{}/templates/", server_root, app),
            ]
        }
    }

    /// Get template directories for core
    pub fn get_core_template_dirs(&self, theme: &str, server_root: &str) -> Vec<String> {
        vec![
            format!("{}/themes/{}/core/templates/", server_root, theme),
            format!("{}/core/templates/", server_root),
        ]
    }

    /// Assign variables
    ///
    /// This function assigns a variable. It can be accessed via $_[$key] in
    /// the template.
    ///
    /// If the key existed before, it will be overwritten
    pub fn assign(&mut self, key: &str, value: Value) -> bool {
        self.vars.insert(key.to_string(), value);
        true
    }

    /// Appends a variable
    ///
    /// This function assigns a variable in an array context. If the key already
    /// exists, the value will be appended. It can be accessed via
    /// $_[$key][$position] in the template.
    pub fn append(&mut self, key: &str, value: Value) {
        match self.vars.get_mut(key) {
            Some(Value::Array(array)) => {
                array.push(value);
            }
            Some(existing_value) => {
                let existing_clone = existing_value.clone();
                self.vars.insert(key.to_string(), Value::Array(vec![existing_clone, value]));
            }
            None => {
                self.vars.insert(key.to_string(), Value::Array(vec![value]));
            }
        }
    }

    /// Prints the proceeded template
    ///
    /// This function proceeds the template and prints its output.
    pub fn print_page(&self) -> io::Result<bool> {
        match self.fetch_page() {
            Ok(data) => {
                io::stdout().write_all(data.as_bytes())?;
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    /// Process the template
    ///
    /// This function processes the template.
    pub fn fetch_page(&self) -> io::Result<String> {
        self.load(&self.template, None)
    }

    /// Doing the actual work
    ///
    /// Reads the template file and processes it with the variables
    fn load(&self, file: &str, additional_params: Option<HashMap<String, Value>>) -> io::Result<String> {
        // Merge additional parameters with existing variables if provided
        let vars = match additional_params {
            Some(params) => {
                let mut merged = self.vars.clone();
                for (k, v) in params {
                    merged.insert(k, v);
                }
                merged
            }
            None => self.vars.clone(),
        };

        // Read the template file
        let template_content = fs::read_to_string(file)?;
        
        // In a real implementation, we would need to process the template here
        // For now, we'll just return the file content as a placeholder
        // A full implementation would need a template engine to replace variables
        
        // Note: PHP's ob_start/ob_get_contents/ob_end_clean functionality
        // would be replaced by the template engine's rendering process
        
        Ok(template_content)
    }
}