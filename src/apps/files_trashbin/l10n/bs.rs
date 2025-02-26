use std::collections::HashMap;
use rust_i18n::locale::LocaleForm;

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("Name", "Ime");
    map
}

pub fn plural_forms() -> LocaleForm {
    LocaleForm::new("nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);")
}