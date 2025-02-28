use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("he");

pub fn initialize_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Access granted", "הוענקה גישה");
    translations.insert("Error configuring Dropbox storage", "אירעה שגיאה בעת הגדרת אחסון ב־Dropbox");
    translations.insert("Grant access", "הענקת גישה");
    translations.insert("Please provide a valid Dropbox app key and secret.", "נא לספק קוד יישום וסוד תקניים של Dropbox.");
    translations.insert("Error configuring Google Drive storage", "אירעה שגיאה בעת הגדרת אחסון ב־Google Drive");
    translations.insert("External Storage", "אחסון חיצוני");
    translations.insert("Folder name", "שם התיקייה");
    translations.insert("Configuration", "הגדרות");
    translations.insert("Options", "אפשרויות");
    translations.insert("Applicable", "ניתן ליישום");
    translations.insert("None set", "לא הוגדרה");
    translations.insert("All Users", "כל המשתמשים");
    translations.insert("Groups", "קבוצות");
    translations.insert("Users", "משתמשים");
    translations.insert("Delete", "מחיקה");
    translations.insert("Enable User External Storage", "הפעלת אחסון חיצוני למשתמשים");
    translations.insert("Allow users to mount their own external storage", "יאפשר למשתמשים לעגן את האחסון החיצוני שלהם");
    translations.insert("SSL root certificates", "שורש אישורי אבטחת SSL ");
    translations.insert("Import Root Certificate", "ייבוא אישור אבטחת שורש");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}