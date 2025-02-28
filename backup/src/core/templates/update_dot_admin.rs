use tera::{Tera, Context};

/// Render the update admin template
pub fn render_update_admin(templates: &Tera, version: &str, l10n: &impl Translator) -> Result<String, tera::Error> {
    let mut context = Context::new();
    let update_message = l10n.t("Updating ownCloud to version %s, this may take a while.", &[version]);
    
    context.insert("update_message", &update_message);
    
    templates.render("update_admin.html", &context)
}

/// Template file: templates/update_admin.html
/// ```html
/// <ul>
///     <li class='update'>
///         {{ update_message }}<br /><br />
///     </li>
/// </ul>
///

/// Trait for translation functionality
pub trait Translator {
    fn t(&self, text: &str, params: &[&str]) -> String;
}