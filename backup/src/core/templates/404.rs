use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use tera::{Context, Tera};

// Error handling for template not found
async fn not_found(tera: web::Data<Tera>, info: web::Query<NotFoundParams>) -> impl Responder {
    let mut context = Context::new();
    
    // For standalone error page case (similar to the original PHP check)
    if info.standalone.is_some() && info.standalone.unwrap() {
        return render_standalone_error_page(tera).await;
    }
    
    // Add content if available
    if let Some(content) = &info.content {
        context.insert("content", content);
    }
    
    // Add file path if available
    if let Some(file) = &info.file {
        context.insert("file", file);
    }
    
    // Add translation for 'Cloud not found'
    context.insert("cloud_not_found", "Cloud not found");
    
    match tera.render("404.html", &context) {
        Ok(rendered) => HttpResponse::NotFound().content_type("text/html").body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

// For standalone error case, similar to the PHP require_once path
async fn render_standalone_error_page(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    
    match tera.render("404.html", &context) {
        Ok(rendered) => HttpResponse::NotFound().content_type("text/html").body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

// Structure to capture potential query parameters
#[derive(serde::Deserialize)]
struct NotFoundParams {
    content: Option<String>,
    file: Option<String>,
    standalone: Option<bool>,
}

// Config for registering the 404 handler
pub fn config_404_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/404").route(web::get().to(not_found)));
}

// Register Tera templates
pub fn register_templates(tera: &mut Tera) -> Result<(), tera::Error> {
    tera.add_raw_template("404.html", r#"
        {% if content %}
            {{ content | safe }}
        {% else %}
            <ul>
                <li class="error">
                    {{ cloud_not_found }}<br/>
                    <p class='hint'>{% if file %}{{ file }}{% endif %}</p>
                </li>
            </ul>
        {% endif %}
    "#)?;
    
    Ok(())
}