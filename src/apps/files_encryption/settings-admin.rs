use axum::{response::Html, Extension};
use serde::Deserialize;
use std::sync::Arc;

/**
 * Copyright (c) 2011 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

#[derive(Deserialize)]
pub struct AppConfig {
    recovery_admin_enabled: Option<String>,
}

pub struct TemplateEngine {
    template_name: String,
    template_vars: std::collections::HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new(app_name: &str, template_name: &str) -> Self {
        TemplateEngine {
            template_name: format!("{}/{}", app_name, template_name),
            template_vars: std::collections::HashMap::new(),
        }
    }

    pub fn assign(&mut self, key: &str, value: &str) {
        self.template_vars.insert(key.to_string(), value.to_string());
    }

    pub fn fetch_page(&self) -> Result<String, anyhow::Error> {
        // Implementation would render the template with the assigned variables
        Ok(format!("Rendered template: {}", self.template_name))
    }
}

pub struct AppState {
    pub config: Arc<dyn ConfigService>,
}

#[async_trait::async_trait]
pub trait ConfigService: Send + Sync {
    async fn get_app_config(&self, app_name: &str, key: &str, default: &str) -> Result<String, anyhow::Error>;
}

pub struct AuthService;

impl AuthService {
    pub fn check_admin_user(&self) -> Result<(), anyhow::Error> {
        // Implementation would check if the current user is an admin
        Ok(())
    }
}

pub async fn settings_admin(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Html<String>, anyhow::Error> {
    // Check if the user is an admin
    AuthService.check_admin_user()?;

    // Create a new template
    let mut tmpl = TemplateEngine::new("files_encryption", "settings-admin");

    // Check if an adminRecovery account is enabled for recovering files after lost pwd
    let recovery_admin_enabled = state.config.get_app_config(
        "files_encryption", 
        "recoveryAdminEnabled", 
        "0"
    ).await?;

    tmpl.assign("recoveryEnabled", &recovery_admin_enabled);

    // In a real implementation, you would likely add the scripts to a response header
    // or include them directly in the template

    // Return the rendered template
    let page = tmpl.fetch_page()?;
    Ok(Html(page))
}