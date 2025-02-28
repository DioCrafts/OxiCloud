use std::collections::HashMap;

pub fn render_altmail(
    l: &impl Translator,
    params: &HashMap<String, String>,
    theme: &impl Theme,
) -> String {
    let mut output = String::new();
    
    // Main message part
    output.push_str(&l.t(
        "Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n",
        &[
            params.get("user_displayname").unwrap_or(&String::new()),
            params.get("filename").unwrap_or(&String::new()),
            params.get("link").unwrap_or(&String::new()),
        ],
    ));
    
    // Optional expiration part
    if let Some(expiration) = params.get("expiration") {
        output.push_str(&l.t(
            "The share will expire on %s.\n\n",
            &[expiration],
        ));
    }
    
    // Closing part
    output.push_str(&l.t("Cheers!"));
    
    // Signature
    output.push_str("\n\n--\n");
    output.push_str(&format!("{} - {}", theme.get_name(), theme.get_slogan()));
    output.push_str(&format!("\n{}", theme.get_base_url()));
    
    output
}

pub trait Translator {
    fn t(&self, text: &str, params: &[&str]) -> String;
}

pub trait Theme {
    fn get_name(&self) -> String;
    fn get_slogan(&self) -> String;
    fn get_base_url(&self) -> String;
}