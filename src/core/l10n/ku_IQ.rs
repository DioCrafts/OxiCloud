use rust_i18n::i18n;

i18n!("ku_IQ", {
    "Settings": "ده‌ستكاری",
    "_%n minute ago_::_%n minutes ago_": ["", ""],
    "_%n hour ago_::_%n hours ago_": ["", ""],
    "_%n day ago_::_%n days ago_": ["", ""],
    "_%n month ago_::_%n months ago_": ["", ""],
    "_{count} file conflict_::_{count} file conflicts_": ["", ""],
    "Share": "هاوبەشی کردن",
    "Error": "هه‌ڵه",
    "Password": "وشەی تێپەربو",
    "Warning": "ئاگاداری",
    "Add": "زیادکردن",
    "Username": "ناوی به‌کارهێنه‌ر",
    "New password": "وشەی نهێنی نوێ",
    "Reset password": "دووباره‌ كردنه‌وه‌ی وشه‌ی نهێنی",
    "Users": "به‌كارهێنه‌ر",
    "Apps": "به‌رنامه‌كان",
    "Admin": "به‌ڕێوه‌به‌ری سه‌ره‌كی",
    "Help": "یارمەتی",
    "Cloud not found": "هیچ نه‌دۆزرایه‌وه‌",
    "Advanced": "هه‌ڵبژاردنی پیشكه‌وتوو",
    "Data folder": "زانیاری فۆڵده‌ر",
    "Database user": "به‌كارهێنه‌ری داتابه‌یس",
    "Database password": "وشه‌ی نهێنی داتا به‌یس",
    "Database name": "ناوی داتابه‌یس",
    "Database host": "هۆستی داتابه‌یس",
    "Finish setup": "كۆتایی هات ده‌ستكاریه‌كان",
    "Log out": "چوونەدەرەوە"
});

// Plural forms configuration
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}