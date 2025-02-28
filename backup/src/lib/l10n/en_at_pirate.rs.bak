use std::collections::HashMap;
use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

pub struct PirateEnglish;

impl PirateEnglish {
    pub fn get_translations() -> HashMap<String, String> {
        let mut translations = HashMap::new();
        translations.insert(
            "web services under your control".to_string(),
            "web services under your control".to_string(),
        );
        translations
    }

    pub fn get_plural_forms() -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }

    pub fn create_bundle() -> FluentBundle<FluentResource> {
        let lang_id: LanguageIdentifier = "en-PIRATE".parse().unwrap();
        let mut bundle = FluentBundle::new(vec![lang_id]);

        // Add plural forms for minutes
        let min_ftl = FluentResource::try_new(
            r#"
            minutes-ago = { $n ->
                [1] { $n } minute ago
                *[other] { $n } minutes ago
            }
            "#.to_string(),
        )
        .expect("Failed to parse minutes FTL");
        bundle.add_resource(min_ftl).expect("Failed to add minutes resource");

        // Add plural forms for hours
        let hour_ftl = FluentResource::try_new(
            r#"
            hours-ago = { $n ->
                [1] { $n } hour ago
                *[other] { $n } hours ago
            }
            "#.to_string(),
        )
        .expect("Failed to parse hours FTL");
        bundle.add_resource(hour_ftl).expect("Failed to add hours resource");

        // Add plural forms for days
        let day_ftl = FluentResource::try_new(
            r#"
            days-ago = { $n ->
                [1] { $n } day go
                *[other] { $n } days ago
            }
            "#.to_string(),
        )
        .expect("Failed to parse days FTL");
        bundle.add_resource(day_ftl).expect("Failed to add days resource");

        // Add plural forms for months
        let month_ftl = FluentResource::try_new(
            r#"
            months-ago = { $n ->
                [1] { $n } month ago
                *[other] { $n } months ago
            }
            "#.to_string(),
        )
        .expect("Failed to parse months FTL");
        bundle.add_resource(month_ftl).expect("Failed to add months resource");

        bundle
    }
}