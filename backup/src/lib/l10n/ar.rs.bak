use lazy_static::lazy_static;
use std::collections::HashMap;
use fluent_templates::static_loader;
use unic_langid::LanguageIdentifier;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "المساعدة");
        m.insert("Personal", "شخصي");
        m.insert("Settings", "إعدادات");
        m.insert("Users", "المستخدمين");
        m.insert("Admin", "المدير");
        m.insert("web services under your control", "خدمات الشبكة تحت سيطرتك");
        m.insert("ZIP download is turned off.", "تحميل ملفات ZIP متوقف");
        m.insert("Files need to be downloaded one by one.", "الملفات بحاجة الى ان يتم تحميلها واحد تلو الاخر");
        m.insert("Back to Files", "العودة الى الملفات");
        m.insert("Selected files too large to generate zip file.", "الملفات المحددة كبيرة جدا ليتم ضغطها في ملف zip");
        m.insert("Application is not enabled", "التطبيق غير مفعّل");
        m.insert("Authentication error", "لم يتم التأكد من الشخصية بنجاح");
        m.insert("Token expired. Please reload page.", "انتهت صلاحية الكلمة , يرجى اعادة تحميل الصفحة");
        m.insert("Files", "الملفات");
        m.insert("Text", "معلومات إضافية");
        m.insert("Images", "صور");
        m.insert("%s enter the database username.", "%s ادخل اسم المستخدم الخاص بقاعدة البيانات.");
        m.insert("%s enter the database name.", "%s ادخل اسم فاعدة البيانات");
        m.insert("%s you may not use dots in the database name", "%s لا يسمح لك باستخدام نقطه (.) في اسم قاعدة البيانات");
        m.insert("MS SQL username and/or password not valid: %s", "اسم المستخدم  و/أو  كلمة المرور لنظام MS SQL غير صحيح : %s");
        m.insert("You need to enter either an existing account or the administrator.", "انت بحاجة لكتابة اسم مستخدم موجود أو حساب المدير.");
        m.insert("MySQL username and/or password not valid", "اسم المستخدم  و/أو  كلمة المرور لنظام MySQL غير صحيح");
        m.insert("DB Error: \"%s\"", "خطأ في قواعد البيانات : \"%s\"");
        m.insert("Offending command was: \"%s\"", "الأمر المخالف كان : \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "أسم المستخدم  '%s'@'localhost' الخاص بـ MySQL موجود مسبقا");
        m.insert("Drop this user from MySQL", "احذف اسم المستخدم هذا من الـ MySQL");
        m.insert("MySQL user '%s'@'%%' already exists", "أسم المستخدم  '%s'@'%%' الخاص بـ MySQL موجود مسبقا");
        m.insert("Drop this user from MySQL.", "احذف اسم المستخدم هذا من الـ MySQL.");
        m.insert("Oracle username and/or password not valid", "اسم المستخدم  و/أو  كلمة المرور لنظام Oracle غير صحيح");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "الأمر المخالف كان : \"%s\", اسم المستخدم : %s, كلمة المرور: %s");
        m.insert("PostgreSQL username and/or password not valid", "اسم المستخدم / أو كلمة المرور الخاصة بـPostgreSQL غير صحيحة");
        m.insert("Set an admin username.", "اعداد اسم مستخدم للمدير");
        m.insert("Set an admin password.", "اعداد كلمة مرور للمدير");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "اعدادات خادمك غير صحيحة بشكل تسمح لك بمزامنة ملفاتك وذلك بسبب أن واجهة WebDAV تبدو معطلة");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "الرجاء التحقق من <a href='%s'>دليل التنصيب</a>.");
        m.insert("Could not find category \"%s\"", "تعذر العثور على المجلد \"%s\"");
        m.insert("seconds ago", "منذ ثواني");
        m.insert("today", "اليوم");
        m.insert("yesterday", "يوم أمس");
        m.insert("last month", "الشهر الماضي");
        m.insert("last year", "السنةالماضية");
        m.insert("years ago", "سنة مضت");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=6; plural=n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5;";
    
    // Mapa para plurales
    pub static ref PLURAL_MINUTES: [&'static str; 6] = ["", "", "", "", "", ""];
    pub static ref PLURAL_HOURS: [&'static str; 6] = ["", "", "", "", "", ""];
    pub static ref PLURAL_DAYS: [&'static str; 6] = ["", "", "", "", "", ""];
    pub static ref PLURAL_MONTHS: [&'static str; 6] = ["", "", "", "", "", ""];
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if n == 2 {
        2
    } else if n % 100 >= 3 && n % 100 <= 10 {
        3
    } else if n % 100 >= 11 && n % 100 <= 99 {
        4
    } else {
        5
    }
}

pub fn get_plural_translation(key: &str, n: usize) -> Option<&'static str> {
    let index = get_plural_form(n);
    match key {
        "_%n minute ago_::_%n minutes ago_" => Some(PLURAL_MINUTES[index]),
        "_%n hour ago_::_%n hours ago_" => Some(PLURAL_HOURS[index]),
        "_%n day go_::_%n days ago_" => Some(PLURAL_DAYS[index]),
        "_%n month ago_::_%n months ago_" => Some(PLURAL_MONTHS[index]),
        _ => None,
    }
}