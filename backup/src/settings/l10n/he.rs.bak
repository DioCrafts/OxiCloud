use std::collections::HashMap;
use fluent_templates::static_loader;
use unic_langid::LanguageIdentifier;

// Define the static language loader for Fluent
static_loader! {
    static TRANSLATIONS = {
        locales: "./l10n",
        fallback_language: "en-US",
        core_locales: ["he"],
    };
}

/// Hebrew translations for the application
pub fn get_hebrew_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Unable to load list from App Store".to_string(), "לא ניתן לטעון רשימה מה־App Store".to_string());
    translations.insert("Authentication error".to_string(), "שגיאת הזדהות".to_string());
    translations.insert("Group already exists".to_string(), "הקבוצה כבר קיימת".to_string());
    translations.insert("Unable to add group".to_string(), "לא ניתן להוסיף קבוצה".to_string());
    translations.insert("Email saved".to_string(), "הדוא״ל נשמר".to_string());
    translations.insert("Invalid email".to_string(), "דוא״ל לא חוקי".to_string());
    translations.insert("Unable to delete group".to_string(), "לא ניתן למחוק את הקבוצה".to_string());
    translations.insert("Unable to delete user".to_string(), "לא ניתן למחוק את המשתמש".to_string());
    translations.insert("Language changed".to_string(), "שפה השתנתה".to_string());
    translations.insert("Invalid request".to_string(), "בקשה לא חוקית".to_string());
    translations.insert("Admins can't remove themself from the admin group".to_string(), "מנהלים לא יכולים להסיר את עצמם מקבוצת המנהלים".to_string());
    translations.insert("Unable to add user to group %s".to_string(), "לא ניתן להוסיף משתמש לקבוצה %s".to_string());
    translations.insert("Unable to remove user from group %s".to_string(), "לא ניתן להסיר משתמש מהקבוצה %s".to_string());
    translations.insert("Couldn't update app.".to_string(), "לא ניתן לעדכן את היישום.".to_string());
    translations.insert("Update to {appversion}".to_string(), "עדכון לגרסה {appversion}".to_string());
    translations.insert("Disable".to_string(), "בטל".to_string());
    translations.insert("Enable".to_string(), "הפעלה".to_string());
    translations.insert("Please wait....".to_string(), "נא להמתין…".to_string());
    translations.insert("Updating....".to_string(), "מתבצע עדכון…".to_string());
    translations.insert("Error while updating app".to_string(), "אירעה שגיאה בעת עדכון היישום".to_string());
    translations.insert("Error".to_string(), "שגיאה".to_string());
    translations.insert("Update".to_string(), "עדכון".to_string());
    translations.insert("Updated".to_string(), "מעודכן".to_string());
    translations.insert("Saving...".to_string(), "שמירה…".to_string());
    translations.insert("deleted".to_string(), "נמחק".to_string());
    translations.insert("undo".to_string(), "ביטול".to_string());
    translations.insert("Unable to remove user".to_string(), "לא ניתן להסיר את המשתמש".to_string());
    translations.insert("Groups".to_string(), "קבוצות".to_string());
    translations.insert("Group Admin".to_string(), "מנהל הקבוצה".to_string());
    translations.insert("Delete".to_string(), "מחיקה".to_string());
    translations.insert("add group".to_string(), "הוספת קבוצה".to_string());
    translations.insert("A valid username must be provided".to_string(), "יש לספק שם משתמש תקני".to_string());
    translations.insert("Error creating user".to_string(), "יצירת המשתמש נכשלה".to_string());
    translations.insert("A valid password must be provided".to_string(), "יש לספק ססמה תקנית".to_string());
    translations.insert("__language_name__".to_string(), "עברית".to_string());
    translations.insert("Security Warning".to_string(), "אזהרת אבטחה".to_string());
    translations.insert("Setup Warning".to_string(), "שגיאת הגדרה".to_string());
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(), "שרת האינטרנט שלך אינו מוגדר לצורכי סנכרון קבצים עדיין כיוון שמנשק ה־WebDAV כנראה אינו תקין.".to_string());
    translations.insert("Module 'fileinfo' missing".to_string(), "המודול „fileinfo" חסר".to_string());
    translations.insert("Internet connection not working".to_string(), "החיבור לאינטרנט אינו פעיל".to_string());
    translations.insert("Cron".to_string(), "Cron".to_string());
    translations.insert("Execute one task with each page loaded".to_string(), "יש להפעיל משימה אחת עם כל עמוד שנטען".to_string());
    translations.insert("Sharing".to_string(), "שיתוף".to_string());
    translations.insert("Enable Share API".to_string(), "הפעלת API השיתוף".to_string());
    translations.insert("Allow apps to use the Share API".to_string(), "לאפשר ליישום להשתמש ב־API השיתוף".to_string());
    translations.insert("Allow links".to_string(), "לאפשר קישורים".to_string());
    translations.insert("Allow users to share items to the public with links".to_string(), "לאפשר למשתמשים לשתף פריטים ".to_string());
    translations.insert("Allow resharing".to_string(), "לאפשר שיתוף מחדש".to_string());
    translations.insert("Allow users to share items shared with them again".to_string(), "לאפשר למשתמשים לשתף הלאה פריטים ששותפו אתם".to_string());
    translations.insert("Allow users to share with anyone".to_string(), "לאפשר למשתמשים לשתף עם כל אחד".to_string());
    translations.insert("Allow users to only share with users in their groups".to_string(), "לאפשר למשתמשים לשתף עם משתמשים בקבוצות שלהם בלבד".to_string());
    translations.insert("Security".to_string(), "אבטחה".to_string());
    translations.insert("Enforce HTTPS".to_string(), "לאלץ HTTPS".to_string());
    translations.insert("Log".to_string(), "יומן".to_string());
    translations.insert("Log level".to_string(), "רמת הדיווח".to_string());
    translations.insert("More".to_string(), "יותר".to_string());
    translations.insert("Less".to_string(), "פחות".to_string());
    translations.insert("Version".to_string(), "גרסא".to_string());
    translations.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.".to_string(), "פותח על די <a href=\"http://ownCloud.org/contact\" target=\"_blank\">קהילתownCloud</a>, <a href=\"https://github.com/owncloud\" target=\"_blank\">קוד המקור</a> מוגן ברישיון <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.".to_string());
    translations.insert("Add your App".to_string(), "הוספת היישום שלך".to_string());
    translations.insert("More Apps".to_string(), "יישומים נוספים".to_string());
    translations.insert("Select an App".to_string(), "בחירת יישום".to_string());
    translations.insert("See application page at apps.owncloud.com".to_string(), "צפה בעמוד הישום ב apps.owncloud.com".to_string());
    translations.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>".to_string(), "ברישיון <span class=\"licence\"></span>לטובת <span class=\"author\"></span>".to_string());
    translations.insert("User Documentation".to_string(), "תיעוד משתמש".to_string());
    translations.insert("Administrator Documentation".to_string(), "תיעוד מנהלים".to_string());
    translations.insert("Online Documentation".to_string(), "תיעוד מקוון".to_string());
    translations.insert("Forum".to_string(), "פורום".to_string());
    translations.insert("Bugtracker".to_string(), "עוקב תקלות".to_string());
    translations.insert("Commercial Support".to_string(), "תמיכה בתשלום".to_string());
    translations.insert("Get the apps to sync your files".to_string(), "השג את האפליקציות על מנת לסנכרן את הקבצים שלך".to_string());
    translations.insert("Show First Run Wizard again".to_string(), "הצגת אשף ההפעלה הראשונית שוב".to_string());
    translations.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>".to_string(), "השתמשת ב־<strong>%s</strong> מתוך <strong>%s</strong> הזמינים לך".to_string());
    translations.insert("Password".to_string(), "סיסמא".to_string());
    translations.insert("Your password was changed".to_string(), "הססמה שלך הוחלפה".to_string());
    translations.insert("Unable to change your password".to_string(), "לא ניתן לשנות את הססמה שלך".to_string());
    translations.insert("Current password".to_string(), "ססמה נוכחית".to_string());
    translations.insert("New password".to_string(), "ססמה חדשה".to_string());
    translations.insert("Change password".to_string(), "שינוי ססמה".to_string());
    translations.insert("Email".to_string(), "דואר אלקטרוני".to_string());
    translations.insert("Your email address".to_string(), "כתובת הדוא״ל שלך".to_string());
    translations.insert("Fill in an email address to enable password recovery".to_string(), "נא למלא את כתובת הדוא״ל שלך כדי לאפשר שחזור ססמה".to_string());
    translations.insert("Profile picture".to_string(), "תמונת פרופיל".to_string());
    translations.insert("Language".to_string(), "פה".to_string());
    translations.insert("Help translate".to_string(), "עזרה בתרגום".to_string());
    translations.insert("WebDAV".to_string(), "WebDAV".to_string());
    translations.insert("Encryption".to_string(), "הצפנה".to_string());
    translations.insert("Login Name".to_string(), "שם כניסה".to_string());
    translations.insert("Create".to_string(), "יצירה".to_string());
    translations.insert("Admin Recovery Password".to_string(), "ססמת השחזור של המנהל".to_string());
    translations.insert("Default Storage".to_string(), "אחסון בררת המחדל".to_string());
    translations.insert("Unlimited".to_string(), "ללא הגבלה".to_string());
    translations.insert("Other".to_string(), "אחר".to_string());
    translations.insert("Username".to_string(), "שם משתמש".to_string());
    translations.insert("Storage".to_string(), "אחסון".to_string());
    translations.insert("set new password".to_string(), "הגדרת ססמה חדשה".to_string());
    translations.insert("Default".to_string(), "בררת מחדל".to_string());
    
    translations
}

pub fn get_plural_form() -> String {
    "nplurals=2; plural=(n != 1);".to_string()
}

/// Get a specific translation
pub fn get_translation(key: &str) -> Option<String> {
    get_hebrew_translations().get(key).cloned()
}

/// Format a translation with parameters
pub fn format_translation(key: &str, params: &[(String, String)]) -> Option<String> {
    let mut text = get_translation(key)?;
    
    for (param_name, param_value) in params {
        let placeholder = format!("{{{}}}", param_name);
        text = text.replace(&placeholder, param_value);
    }
    
    Some(text)
}

/// Get the language identifier for Hebrew
pub fn get_language_id() -> Result<LanguageIdentifier, unic_langid::parser::ParserError> {
    "he".parse()
}