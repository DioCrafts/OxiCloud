//! Módulo de interfaz de usuario web para OxiCloud
//! 
//! Este módulo proporciona una interfaz web básica para interactuar
//! con OxiCloud a través de un navegador.

pub mod auth;
pub mod files;
pub mod home;
pub mod templates;
pub mod assets;

use actix_web::{web, HttpResponse, error};
use serde::Serialize;
use tera::Context;

use crate::AppState;

/// Respuesta con plantilla
pub fn render_template(
    tmpl_name: &str,
    ctx: &Context,
    state: &AppState,
) -> Result<HttpResponse, error::Error> {
    let rendered = state.templates.render(tmpl_name, ctx)
        .map_err(|e| {
            error::ErrorInternalServerError(format!("Error de plantilla: {}", e))
        })?;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}

/// Contexto de página con datos comunes
#[derive(Serialize)]
pub struct PageContext<T>
where 
    T: Serialize,
{
    pub title: String,
    pub user: Option<crate::core::users::User>,
    pub flash_messages: Vec<FlashMessage>,
    pub data: T,
}

impl<T: Serialize> PageContext<T> {
    pub fn new(title: impl Into<String>, data: T) -> Self {
        Self {
            title: title.into(),
            user: None,
            flash_messages: Vec::new(),
            data,
        }
    }
    
    pub fn with_user(mut self, user: Option<crate::core::users::User>) -> Self {
        self.user = user.map(|mut u| {
            // Eliminar password_hash para no enviarlo al cliente
            u.password_hash = String::new();
            u
        });
        self
    }
    
    pub fn with_flash(mut self, message: FlashMessage) -> Self {
        self.flash_messages.push(message);
        self
    }
    
    pub fn with_flashes(mut self, messages: Vec<FlashMessage>) -> Self {
        self.flash_messages.extend(messages);
        self
    }
    
    pub fn into_context(self) -> Context {
        let mut ctx = Context::new();
        
        // Serializar este objeto a un valor que Tera pueda usar
        if let Ok(value) = serde_json::to_value(&self) {
            if let Some(obj) = value.as_object() {
                for (k, v) in obj {
                    ctx.insert(k, v);
                }
            }
        }
        
        ctx
    }
}

/// Tipo de mensaje flash
#[derive(Debug, Clone, Serialize)]
pub enum FlashType {
    Success,
    Error,
    Info,
    Warning,
}

/// Mensaje flash para notificaciones al usuario
#[derive(Debug, Clone, Serialize)]
pub struct FlashMessage {
    pub typ: FlashType,
    pub message: String,
}

impl FlashMessage {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            typ: FlashType::Success,
            message: message.into(),
        }
    }
    
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            typ: FlashType::Error,
            message: message.into(),
        }
    }
    
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            typ: FlashType::Info,
            message: message.into(),
        }
    }
    
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            typ: FlashType::Warning,
            message: message.into(),
        }
    }
}