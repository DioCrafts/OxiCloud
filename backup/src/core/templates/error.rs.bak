use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use html_escape::encode_text;

struct ErrorInfo {
    error: String,
    hint: Option<String>,
}

impl Display for ErrorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        if let Some(hint) = &self.hint {
            write!(f, "<br/><p class='hint'>{}</p>", hint)?;
        } else {
            write!(f, "<br/><p class='hint'></p>")?;
        }
        Ok(())
    }
}

fn render_error_template(errors: &[ErrorInfo]) -> String {
    let mut html = String::from("<ul class=\"error-wide\">\n");
    
    for error in errors {
        html.push_str(&format!("\t<li class='error'>\n\t\t{}\n\t</li>\n", error));
    }
    
    html.push_str("</ul>\n");
    html
}

// Helper functions to mimic PHP behavior
fn p(text: &str) -> String {
    encode_text(text).to_string()
}

fn print_unescaped(text: &str) -> String {
    text.to_string()
}