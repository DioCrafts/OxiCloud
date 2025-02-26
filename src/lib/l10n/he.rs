use std::collections::HashMap;
use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

pub fn create_he_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Help".to_string(), "עזרה".to_string());
    translations.insert("Personal".to_string(), "אישי".to_string());
    translations.insert("Settings".to_string(), "הגדרות".to_string());
    translations.insert("Users".to_string(), "משתמשים".to_string());
    translations.insert("Admin".to_string(), "מנהל".to_string());
    translations.insert("web services under your control".to_string(), "שירותי רשת תחת השליטה שלך".to_string());
    translations.insert("ZIP download is turned off.".to_string(), "הורדת ZIP כבויה".to_string());
    translations.insert("Files need to be downloaded one by one.".to_string(), "יש להוריד את הקבצים אחד אחרי השני.".to_string());
    translations.insert("Back to Files".to_string(), "חזרה לקבצים".to_string());
    translations.insert("Selected files too large to generate zip file.".to_string(), "הקבצים הנבחרים גדולים מידי ליצירת קובץ zip.".to_string());
    translations.insert("Application is not enabled".to_string(), "יישומים אינם מופעלים".to_string());
    translations.insert("Authentication error".to_string(), "שגיאת הזדהות".to_string());
    translations.insert("Token expired. Please reload page.".to_string(), "פג תוקף. נא לטעון שוב את הדף.".to_string());
    translations.insert("Files".to_string(), "קבצים".to_string());
    translations.insert("Text".to_string(), "טקסט".to_string());
    translations.insert("Images".to_string(), "תמונות".to_string());
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(), "שרת האינטרנט שלך אינו מוגדר לצורכי סנכרון קבצים עדיין כיוון שמנשק ה־WebDAV כנראה אינו תקין.".to_string());
    translations.insert("Please double check the <a href='%s'>installation guides</a>.".to_string(), "נא לעיין שוב ב<a href='%s'>מדריכי ההתקנה</a>.".to_string());
    translations.insert("Could not find category \"%s\"".to_string(), "לא ניתן למצוא את הקטגוריה „%s"".to_string());
    translations.insert("seconds ago".to_string(), "שניות".to_string());
    translations.insert("today".to_string(), "היום".to_string());
    translations.insert("yesterday".to_string(), "אתמול".to_string());
    translations.insert("last month".to_string(), "חודש שעבר".to_string());
    translations.insert("last year".to_string(), "שנה שעברה".to_string());
    translations.insert("years ago".to_string(), "שנים".to_string());
    
    translations
}

pub fn create_he_bundle() -> FluentBundle<FluentResource> {
    let langid_he: LanguageIdentifier = "he".parse().expect("Failed to parse language identifier");
    let mut bundle = FluentBundle::new(vec![langid_he]);
    
    // Add plural rules
    let ftl_string = r#"
    # Plural forms for Hebrew
    minute = { $n ->
        [1] minute ago
        *[other] { $n } minutes ago
    }
    
    hour = { $n ->
        [1] hour ago
        *[other] { $n } hours ago
    }
    
    day = { $n ->
        [1] day ago
        *[other] { $n } days ago
    }
    
    month = { $n ->
        [1] month ago
        *[other] { $n } months ago
    }
    "#;
    
    let resource = FluentResource::try_new(ftl_string.to_string())
        .expect("Failed to parse FTL resource");
    
    bundle.add_resource(resource)
        .expect("Failed to add FTL resource to bundle");
    
    bundle
}

pub fn get_he_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}