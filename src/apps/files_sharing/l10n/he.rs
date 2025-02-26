use rust_i18n::i18n;

i18n!("he", {
    "Password": "סיסמא",
    "%s shared the folder %s with you": "%s שיתף עמך את התיקייה %s",
    "%s shared the file %s with you": "%s שיתף עמך את הקובץ %s",
    "Download": "הורדה",
    "Upload": "העלאה",
    "Cancel upload": "ביטול ההעלאה",
    "No preview available for": "אין תצוגה מקדימה זמינה עבור"
});

#[allow(dead_code)]
fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}