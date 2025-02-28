use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn load_translations() -> (HashMap<&'static str, &'static str>, &'static str) {
    let translations = HashMap::from([
        ("The password is wrong. Try again.", "رمزعبور اشتباه می باشد. دوباره امتحان کنید."),
        ("Password", "گذرواژه"),
        ("%s shared the folder %s with you", "%sپوشه %s را با شما به اشتراک گذاشت"),
        ("%s shared the file %s with you", "%sفایل %s را با شما به اشتراک گذاشت"),
        ("Download", "دانلود"),
        ("Upload", "بارگزاری"),
        ("Cancel upload", "متوقف کردن بار گذاری"),
        ("No preview available for", "هیچگونه پیش نمایشی موجود نیست"),
    ]);
    
    let plural_forms = "nplurals=1; plural=0;";
    
    (translations, plural_forms)
}

pub fn get_catalog() -> Catalog {
    let (translations, plural_forms) = load_translations();
    
    let mut catalog = Catalog::new();
    catalog.set_plural_forms(plural_forms);
    
    for (key, value) in translations {
        catalog.add_message(key, value);
    }
    
    catalog
}