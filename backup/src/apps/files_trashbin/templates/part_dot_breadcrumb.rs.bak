use std::path::Path;
use askama::Template;

#[derive(Template)]
#[template(path = "part.breadcrumb.html")]
pub struct BreadcrumbTemplate<'a> {
    home: &'a str,
    base_url: &'a str,
    breadcrumb: &'a [Crumb<'a>],
    l: &'a dyn Translator,
}

pub struct Crumb<'a> {
    dir: &'a str,
    name: &'a str,
}

pub trait Translator {
    fn t(&self, key: &str) -> String;
}

pub fn image_path(app: &str, file: &str) -> String {
    Path::new(app).join("places").join(file).to_string_lossy().into_owned()
}

pub fn encode_path(path: &str) -> String {
    // Implementation of OCP\Util::encodePath equivalent
    urlencoding::encode(path).into_owned()
}

// Template HTML file (part.breadcrumb.html):
// 
// <div class="crumb home">
//     <a href="{{ home }}">
//         <img src="{{ image_path("core", "places/home.svg") }}" class="svg" />
//     </a>
// </div>
// {% if breadcrumb.len() > 0 %}
//     <div class="crumb svg" data-dir='/'>
//         <a href="{{ base_url }}">{{ l.t("Deleted Files") }}</a>
//     </div>
// {% endif %}
// {% for crumb in breadcrumb %}
//     <div class="crumb {% if loop.index == breadcrumb.len() %}last{% endif %} svg"
//          data-dir='{{ encode_path(crumb.dir) }}'>
//         <a href="{{ base_url }}{{ encode_path(crumb.dir) }}">{{ crumb.name }}</a>
//     </div>
// {% endfor %}