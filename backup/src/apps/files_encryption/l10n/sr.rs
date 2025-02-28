use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: std::collections::HashMap<&'static str, &'static str> = {
        let mut m = std::collections::HashMap::new();
        m.insert("Saving...", "Чување у току...");
        m.insert("Encryption", "Шифровање");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";