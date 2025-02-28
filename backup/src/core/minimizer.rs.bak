use std::error::Error;

use crate::app;
use crate::minimizer::{CssMinimizer, JsMinimizer, Minimizer};
use crate::template_layout;
use crate::util;
use crate::session;

#[derive(Debug, thiserror::Error)]
pub enum MinimizerError {
    #[error("Unknown service requested: {0}")]
    UnknownService(String),
    #[error("Session error: {0}")]
    SessionError(String),
    #[error("App loading error: {0}")]
    AppError(String),
    #[error("Minimizer error: {0}")]
    MinimizerError(String),
}

pub async fn handle_minimizer_request(service: &str) -> Result<(), Box<dyn Error>> {
    // Close the session write immediately
    session::write_close()
        .map_err(|e| MinimizerError::SessionError(e.to_string()))?;

    // Load all apps
    app::load_apps()
        .map_err(|e| MinimizerError::AppError(e.to_string()))?;

    match service {
        "core.css" => {
            let minimizer = CssMinimizer::new();
            let files = template_layout::find_stylesheet_files(&util::core_styles())
                .map_err(|e| MinimizerError::MinimizerError(e.to_string()))?;
            minimizer.output(&files, service)
                .map_err(|e| MinimizerError::MinimizerError(e.to_string()))?;
        },
        "core.js" => {
            let minimizer = JsMinimizer::new();
            let files = template_layout::find_javascript_files(&util::core_scripts())
                .map_err(|e| MinimizerError::MinimizerError(e.to_string()))?;
            minimizer.output(&files, service)
                .map_err(|e| MinimizerError::MinimizerError(e.to_string()))?;
        },
        _ => {
            return Err(Box::new(MinimizerError::UnknownService(service.to_string())));
        }
    }

    Ok(())
}