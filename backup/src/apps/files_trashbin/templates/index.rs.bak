use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
struct TrashbinIndexTemplate<'a> {
    breadcrumb: &'a str,
    files: Option<Vec<HashMap<String, String>>>,
    dirlisting: bool,
    ajax_load: bool,
    disable_sharing: bool,
    dir: &'a str,
    file_list: &'a str,
    l: &'a dyn Translator,
}

trait Translator {
    fn t(&self, text: &str) -> String;
}

struct L10n;

impl Translator for L10n {
    fn t(&self, text: &str) -> String {
        // Implementación real de traducción
        text.to_string()
    }
}

fn image_path(app: &str, file: &str) -> String {
    format!("/apps/{}/img/{}", app, file)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_template_rendering() {
        let l10n = L10n {};
        let template = TrashbinIndexTemplate {
            breadcrumb: "<div class='breadcrumb'>Home</div>",
            files: Some(vec![]),
            dirlisting: false,
            ajax_load: false,
            disable_sharing: true,
            dir: "/",
            file_list: "<tr><td>example.txt</td></tr>",
            l: &l10n,
        };
        
        let result = template.render();
        assert!(result.is_ok());
    }
}