use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("January", "جنوری");
        m.insert("February", "فرورئ");
        m.insert("March", "مارچ");
        m.insert("April", "اپریل");
        m.insert("May", "مئی");
        m.insert("June", "جون");
        m.insert("July", "جولائی");
        m.insert("August", "اگست");
        m.insert("September", "ستمبر");
        m.insert("October", "اکتوبر");
        m.insert("November", "نومبر");
        m.insert("December", "دسمبر");
        m.insert("Settings", "سیٹینگز");
        m.insert("Choose", "منتخب کریں");
        m.insert("Yes", "ہاں");
        m.insert("No", "نہیں");
        m.insert("Ok", "اوکے");
        m.insert("Cancel", "منسوخ کریں");
        m.insert("Error", "ایرر");
        m.insert("Error while sharing", "شئیرنگ کے دوران ایرر");
        m.insert("Error while unsharing", "شئیرنگ ختم کرنے  کے دوران ایرر");
        m.insert("Error while changing permissions", "اختیارات کو تبدیل کرنے کے دوران ایرر");
        m.insert("Password protect", "پاسورڈ سے محفوظ کریں");
        m.insert("Password", "پاسورڈ");
        m.insert("Set expiration date", "تاریخ معیاد سیٹ کریں");
        m.insert("Expiration date", "تاریخ معیاد");
        m.insert("No people found", "کوئی لوگ نہیں ملے۔");
        m.insert("Resharing is not allowed", "دوبارہ شئیر کرنے کی اجازت نہیں");
        m.insert("Unshare", "شئیرنگ ختم کریں");
        m.insert("can edit", "ایڈٹ کر سکے");
        m.insert("access control", "اسیس کنٹرول");
        m.insert("create", "نیا بنائیں");
        m.insert("update", "اپ ڈیٹ");
        m.insert("delete", "ختم کریں");
        m.insert("share", "شئیر کریں");
        m.insert("Password protected", "پاسورڈ سے محفوظ کیا گیا ہے");
        m.insert("Add", "شامل کریں");
        m.insert("Use the following link to reset your password: {link}", "اپنا پاسورڈ ری سیٹ کرنے کے لیے اس لنک پر کلک کریں۔  {link}");
        m.insert("You will receive a link to reset your password via Email.", "آپ ای میل کے ذریعے اپنے پاسورڈ ری سیٹ کا لنک موصول کریں گے");
        m.insert("Username", "یوزر نیم");
        m.insert("Your password was reset", "آپ کا پاسورڈ ری سیٹ کر دیا گیا ہے");
        m.insert("To login page", "لاگ ان صفحے کی طرف");
        m.insert("New password", "نیا پاسورڈ");
        m.insert("Reset password", "ری سیٹ پاسورڈ");
        m.insert("Personal", "ذاتی");
        m.insert("Users", "یوزرز");
        m.insert("Apps", "ایپز");
        m.insert("Admin", "ایڈمن");
        m.insert("Help", "مدد");
        m.insert("Access forbidden", "پہنچ کی اجازت نہیں");
        m.insert("Cloud not found", "نہیں مل سکا");
        m.insert("Create an <strong>admin account</strong>", "ایک<strong> ایڈمن اکاؤنٹ</strong> بنائیں");
        m.insert("Advanced", "ایڈوانسڈ");
        m.insert("Data folder", "ڈیٹا فولڈر");
        m.insert("Configure the database", "ڈیٹا بیس کونفگر کریں");
        m.insert("will be used", "استعمال ہو گا");
        m.insert("Database user", "ڈیٹابیس یوزر");
        m.insert("Database password", "ڈیٹابیس پاسورڈ");
        m.insert("Database name", "ڈیٹابیس کا نام");
        m.insert("Database tablespace", "ڈیٹابیس ٹیبل سپیس");
        m.insert("Database host", "ڈیٹابیس ہوسٹ");
        m.insert("Finish setup", "سیٹ اپ ختم کریں");
        m.insert("Log out", "لاگ آؤٹ");
        m.insert("Lost your password?", "کیا آپ پاسورڈ بھول گئے ہیں؟");
        m.insert("remember", "یاد رکھیں");
        m.insert("Log in", "لاگ ان");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", ("", ""));
        m.insert("_%n hour ago_::_%n hours ago_", ("", ""));
        m.insert("_%n day ago_::_%n days ago_", ("", ""));
        m.insert("_%n month ago_::_%n months ago_", ("", ""));
        m.insert("_{count} file conflict_::_{count} file conflicts_", ("", ""));
        m
    };

    pub static ref PLURAL_RULE: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form(key: &str, count: i64) -> &'static str {
    if let Some((form1, form2)) = PLURAL_FORMS.get(key) {
        if count == 1 {
            form1
        } else {
            form2
        }
    } else {
        key
    }
}