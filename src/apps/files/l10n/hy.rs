use std::collections::HashMap;
use fluent::FluentResource;
use unic_langid::LanguageIdentifier;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Save", "Պահպանել");
    translations.insert("Download", "Բեռնել");
    translations.insert("Delete", "Ջնջել");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%n folder_::_%n folders_", vec!["", ""]);
    plural_translations.insert("_%n file_::_%n files_", vec!["", ""]);
    plural_translations.insert("_Uploading %n file_::_Uploading %n files_", vec!["", ""]);
    plural_translations
}

pub fn get_fluent_resource() -> FluentResource {
    let ftl_string = r#"
# Plural forms
folder = { $count ->
    [one] folder
    *[other] folders
}
file = { $count ->
    [one] file
    *[other] files
}
uploading = { $count ->
    [one] Uploading file
    *[other] Uploading files
}

# Simple translations
save = Պահպանել
download = Բեռնել
delete = Ջնջել
"#;
    
    FluentResource::try_new(ftl_string.to_string())
        .expect("Failed to parse Fluent resources")
}

pub fn get_language_id() -> LanguageIdentifier {
    "hy".parse()
        .expect("Failed to parse language identifier")
}