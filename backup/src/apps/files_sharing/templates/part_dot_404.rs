use std::collections::HashMap;

pub struct Localization {
    translations: HashMap<String, String>,
}

impl Localization {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        // Default translations
        translations.insert(
            "Sorry, this link doesn't seem to work anymore.".to_string(),
            "Sorry, this link doesn't seem to work anymore.".to_string(),
        );
        translations.insert(
            "Reasons might be:".to_string(),
            "Reasons might be:".to_string(),
        );
        translations.insert(
            "the item was removed".to_string(),
            "the item was removed".to_string(),
        );
        translations.insert(
            "the link expired".to_string(),
            "the link expired".to_string(),
        );
        translations.insert(
            "sharing is disabled".to_string(),
            "sharing is disabled".to_string(),
        );
        translations.insert(
            "For more info, please ask the person who sent this link.".to_string(),
            "For more info, please ask the person who sent this link.".to_string(),
        );
        Self { translations }
    }

    pub fn t(&self, text: &str) -> &str {
        self.translations.get(text).map_or(text, |s| s.as_str())
    }
}

pub fn render_404_template(l: &Localization) -> String {
    format!(
        r#"<ul>
    <li class="error error-broken-link">
        <p>{}</p>
        <p>{}</p>
        <ul>
            <li>{}</li>
            <li>{}</li>
            <li>{}</li>
        </ul>
        <p>{}</p>
    </li>
</ul>"#,
        l.t("Sorry, this link doesn't seem to work anymore."),
        l.t("Reasons might be:"),
        l.t("the item was removed"),
        l.t("the link expired"),
        l.t("sharing is disabled"),
        l.t("For more info, please ask the person who sent this link.")
    )
}