use actix_web::{web, HttpResponse, Result};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "403.html")]
struct ForbiddenTemplate {
    file: Option<String>,
}

// Standalone error handler function
pub async fn handle_forbidden_standalone() -> Result<HttpResponse> {
    // Equivalent to requiring base.php in standalone mode
    let template = ForbiddenTemplate { file: None };
    Ok(HttpResponse::Forbidden()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap_or_else(|_| String::from("Error rendering template"))))
}

// Regular handler when called from main application
pub async fn handle_forbidden(file: Option<web::Path<PathBuf>>) -> Result<HttpResponse> {
    let file_str = file.map(|f| f.to_string_lossy().to_string());
    
    let template = ForbiddenTemplate { file: file_str };
    
    Ok(HttpResponse::Forbidden()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap_or_else(|_| String::from("Error rendering template"))))
}

// HTML template equivalent (to be placed in templates/403.html):
/*
<ul>
    <li class='error'>
        Access forbidden<br/>
        {% if let Some(file) = file %}
        <p class='hint'>{{ file }}</p>
        {% endif %}
    </li>
</ul>
*/