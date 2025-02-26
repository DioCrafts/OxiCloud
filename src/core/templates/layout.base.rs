use std::collections::HashMap;

struct Theme {
    title: String,
}

impl Theme {
    fn new(title: String) -> Self {
        Self { title }
    }

    fn get_title(&self) -> &str {
        &self.title
    }
}

struct TemplateContext {
    theme: Theme,
    cssfiles: Vec<String>,
    jsfiles: Vec<String>,
    headers: Vec<Header>,
    content: String,
}

struct Header {
    tag: String,
    attributes: HashMap<String, String>,
}

fn p(content: &str) -> String {
    htmlescape::encode_minimal(content)
}

fn print_unescaped(content: &str) -> String {
    content.to_string()
}

fn image_path(app: &str, file: &str) -> String {
    if app.is_empty() {
        format!("/assets/{}", file)
    } else {
        format!("/apps/{}/assets/{}", app, file)
    }
}

fn render_layout_base(context: &TemplateContext) -> String {
    let mut html = String::new();
    
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<!--[if lt IE 7]><html class=\"ng-csp ie ie6 lte9 lte8 lte7\"><![endif]-->\n");
    html.push_str("<!--[if IE 7]><html class=\"ng-csp ie ie7 lte9 lte8 lte7\"><![endif]-->\n");
    html.push_str("<!--[if IE 8]><html class=\"ng-csp ie ie8 lte9 lte8\"><![endif]-->\n");
    html.push_str("<!--[if IE 9]><html class=\"ng-csp ie ie9 lte9\"><![endif]-->\n");
    html.push_str("<!--[if gt IE 9]><html class=\"ng-csp ie\"><![endif]-->\n");
    html.push_str("<!--[if !IE]><!--><html class=\"ng-csp\"><!--<![endif]-->\n\n");
    
    html.push_str("\t<head>\n");
    html.push_str("\t\t<title>\n");
    html.push_str(&format!("\t\t{}\n", p(context.theme.get_title())));
    html.push_str("\t\t</title>\n");
    html.push_str("\t\t<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" />\n");
    html.push_str("\t\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("\t\t<link rel=\"shortcut icon\" href=\"{}\" />\n", 
        print_unescaped(&image_path("", "favicon.png"))));
    html.push_str(&format!("\t\t<link rel=\"apple-touch-icon-precomposed\" href=\"{}\" />\n", 
        print_unescaped(&image_path("", "favicon-touch.png"))));
    
    for cssfile in &context.cssfiles {
        html.push_str(&format!("\t\t\t<link rel=\"stylesheet\" href=\"{}\" type=\"text/css\" media=\"screen\" />\n", 
            print_unescaped(cssfile)));
    }
    
    for jsfile in &context.jsfiles {
        html.push_str(&format!("\t\t\t<script type=\"text/javascript\" src=\"{}\"></script>\n", 
            print_unescaped(jsfile)));
    }
    
    for header in &context.headers {
        let mut header_html = format!("\t\t\t<{} ", header.tag);
        for (name, value) in &header.attributes {
            header_html.push_str(&format!("{}='{}' ", name, value));
        }
        header_html.push_str("/>\n");
        html.push_str(&print_unescaped(&header_html));
    }
    
    html.push_str("\t</head>\n\n");
    html.push_str("\t<body>\n");
    html.push_str(&format!("\t\t{}\n", print_unescaped(&context.content)));
    html.push_str("\t</body>\n");
    html.push_str("</html>\n");
    
    html
}