use std::collections::HashMap;
use rust_fluent::FluentResource;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Deletion failed", "Pemadaman gagal");
    translations.insert("Error", "Ralat");
    translations.insert("_%s group found_::_%s groups found_", "");
    translations.insert("_%s user found_::_%s users found_", "");
    translations.insert("Save", "Simpan");
    translations.insert("Help", "Bantuan");
    translations.insert("Password", "Kata laluan");
    translations.insert("Back", "Kembali");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

pub fn create_fluent_resource() -> FluentResource {
    let mut source = String::new();
    
    for (key, value) in get_translations() {
        source.push_str(&format!("{} = {}\n", key, value));
    }
    
    FluentResource::try_new(source)
        .expect("Failed to parse translations as Fluent resource")
}