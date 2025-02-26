use std::collections::HashMap;
use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

pub fn get_translation_bundle() -> FluentBundle<FluentResource> {
    let he_lang: LanguageIdentifier = "he".parse().expect("Failed to parse language identifier");
    
    let translations = r#"
# General file operations
could-not-move-file-exists = לא ניתן להעביר את { $name } - קובץ בשם הזה כבר קיים
could-not-move = לא ניתן להעביר את { $name }
file-name-empty = שם קובץ אינו יכול להיות ריק
no-file-uploaded-unknown = לא הועלה קובץ. טעות בלתי מזוהה.
upload-success = לא התרחשה שגיאה, הקובץ הועלה בהצלחה
upload-exceeds-php-limit = הקבצים שנשלחו חורגים מהגודל שצוין בהגדרה upload_max_filesize שבקובץ php.ini:
upload-exceeds-html-limit = הקובץ שהועלה גדוך מהערך MAX_FILE_SIZE שהוגדר בתופס HTML
upload-partially = הקובץ הועלה באופן חלקי בלבד
no-file-uploaded = שום קובץ לא הועלה
missing-temp-folder = תקיה זמנית חסרה
failed-write-disk = הכתיבה לכונן נכשלה
storage-not-enough = אין די שטח פנוי באחסון
upload-failed-info = העלאה נכשלה. לא ניתן להשיג את פרטי הקובץ.
invalid-directory = תיקייה שגויה.
files = קבצים
upload-cancelled = ההעלאה בוטלה.
server-no-result = לא ניתן לגשת לתוצאות מהשרת.
upload-in-progress = מתבצעת כעת העלאת קבצים. עזיבה של העמוד תבטל את ההעלאה.
name-already-exists = { $new_name } כבר קיים
share = שתף
delete-permanently = מחק לצמיתות
rename = שינוי שם
pending = ממתין
replaced-name = { $new_name } הוחלף ב־{ $old_name }
undo = ביטול

# Plurals
folders = { $count ->
    [one] תיקייה
   *[other] תיקיות
}

files-count = { $count ->
    [one] קובץ
   *[other] קבצים
}

uploading-files = { $count ->
    [one] מעלה קובץ
   *[other] מעלה קבצים
}

invalid-name = השם שגוי, אסור להשתמש בתווים '\\', '/', '<', '>', ':', '\"', '|', '?' ו־'*'.
storage-almost-full = שטח האחסון שלך כמעט מלא ({ $percent }%)
error = שגיאה
name = שם
size = גודל
modified = זמן שינוי
upload = העלאה
file-handling = טיפול בקבצים
max-upload-size = גודל העלאה מקסימלי
max-possible = המרבי האפשרי: 
multi-download-note = נחוץ להורדה של ריבוי קבצים או תיקיות.
enable-zip = הפעלת הורדת ZIP
unlimited = 0 - ללא הגבלה
max-zip-size = גודל הקלט המרבי לקובצי ZIP
save = שמירה
new = חדש
text-file = קובץ טקסט
folder = תיקייה
from-link = מקישור
deleted-files = קבצים שנמחקו
cancel-upload = ביטול ההעלאה
upload-something = אין כאן שום דבר. אולי ברצונך להעלות משהו?
download = הורדה
unshare = הסר שיתוף
delete = מחיקה
upload-too-large = העלאה גדולה מידי
upload-exceeds-server = הקבצים שניסית להעלות חרגו מהגודל המקסימלי להעלאת קבצים על שרת זה.
scanning-wait = הקבצים נסרקים, נא להמתין.
current-scanning = הסריקה הנוכחית
"#;

    let resource = FluentResource::try_new(translations.to_string())
        .expect("Failed to parse translations");

    let mut bundle = FluentBundle::new(vec![he_lang]);
    bundle.add_resource(resource).expect("Failed to add resource to bundle");
    
    // Configure plural rules for Hebrew
    bundle.set_use_isolating(false);
    
    bundle
}

// Function to get the plural forms string
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}