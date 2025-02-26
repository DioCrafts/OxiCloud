use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_translation_catalog() -> Catalog {
    let mut translations = HashMap::new();
    translations.insert("WebDAV Authentication".to_string(), "تأكد شخصية ال WebDAV".to_string());
    
    Catalog::new(
        translations,
        "nplurals=6; plural=n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5;".to_string()
    )
}