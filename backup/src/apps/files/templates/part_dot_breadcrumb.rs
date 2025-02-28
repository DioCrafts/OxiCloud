use askama::Template;
use url::Url;

// Define el struct para los datos de breadcrumb
#[derive(Debug, Clone)]
pub struct Breadcrumb {
    pub name: String,
    pub dir: String,
}

// Template con Askama para renderizar el breadcrumb
#[derive(Template)]
#[template(path = "part.breadcrumb.html")]
pub struct BreadcrumbTemplate {
    pub breadcrumb: Vec<Breadcrumb>,
    pub base_url: String,
    pub home_icon_path: String,
}

impl BreadcrumbTemplate {
    pub fn new(breadcrumb: Vec<Breadcrumb>, base_url: String) -> Self {
        // Asumimos que la función image_path proviene de una biblioteca externa
        let home_icon_path = get_image_path("core", "places/home.svg");
        
        Self {
            breadcrumb,
            base_url,
            home_icon_path,
        }
    }
    
    pub fn encode_path(path: &str) -> String {
        // Emulamos la funcionalidad de OCP\Util::encodePath
        // Una implementación real podría usar url::percent_encode o similar
        let url = Url::parse("http://example.com").unwrap();
        url.join(path)
            .map(|u| u.path().trim_start_matches('/').to_string())
            .unwrap_or_else(|_| path.to_string())
    }
}

// Función auxiliar para emular OCP\image_path
fn get_image_path(app: &str, file: &str) -> String {
    format!("/apps/{}/img/{}", app, file)
}

// El HTML template correspondiente (que se guardaría en "templates/part.breadcrumb.html"):
/*
{% if breadcrumb.len() > 0 %}
    <div class="crumb" data-dir="">
        <a href="{{ base_url }}">
            <img src="{{ home_icon_path }}" class="svg" />
        </a>
    </div>
{% endif %}

{% for crumb in breadcrumb %}
    {% set dir = self::encode_path(&crumb.dir) %}
    <div class="crumb {% if loop.index == breadcrumb.len() %}last{% endif %} svg"
         data-dir="{{ dir }}">
        <a href="{{ base_url }}{{ dir }}">{{ crumb.name }}</a>
    </div>
{% endfor %}
*/