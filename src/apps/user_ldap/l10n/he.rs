use std::collections::HashMap;
use rust_i18n::t;

// Hebrew translation file
pub fn register_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Deletion failed".to_string(), "מחיקה נכשלה".to_string());
    translations.insert("Keep settings?".to_string(), "האם לשמור את ההגדרות?".to_string());
    translations.insert("Cannot add server configuration".to_string(), "לא ניתן להוסיף את הגדרות השרת".to_string());
    translations.insert("Error".to_string(), "שגיאה".to_string());
    translations.insert("Connection test succeeded".to_string(), "בדיקת החיבור עברה בהצלחה".to_string());
    translations.insert("Connection test failed".to_string(), "בדיקת החיבור נכשלה".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?".to_string(), "האם אכן למחוק את הגדרות השרת הנוכחיות?".to_string());
    translations.insert("Confirm Deletion".to_string(), "אישור המחיקה".to_string());
    translations.insert("_%s group found_::_%s groups found_".to_string(), "".to_string());
    translations.insert("_%s user found_::_%s users found_".to_string(), "".to_string());
    translations.insert("Save".to_string(), "שמירה".to_string());
    translations.insert("Help".to_string(), "עזרה".to_string());
    translations.insert("Add Server Configuration".to_string(), "הוספת הגדרות השרת".to_string());
    translations.insert("Host".to_string(), "מארח".to_string());
    translations.insert("Port".to_string(), "פורט".to_string());
    translations.insert("User DN".to_string(), "DN משתמש".to_string());
    translations.insert("Password".to_string(), "סיסמא".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "לגישה אנונימית, השאר את הDM והסיסמא ריקים.".to_string());
    translations.insert("Back".to_string(), "אחורה".to_string());
    translations.insert("User Login Filter".to_string(), "סנן כניסת משתמש".to_string());
    translations.insert("in seconds. A change empties the cache.".to_string(), "בשניות. שינוי מרוקן את המטמון.".to_string());
    translations.insert("in bytes".to_string(), "בבתים".to_string());
    
    translations
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Register translations with the i18n system
pub fn init_translations() {
    let translations = register_translations();
    for (key, value) in translations {
        rust_i18n::set("he", &key, &value);
    }
    rust_i18n::set_plural_rule("he", get_plural_form());
}