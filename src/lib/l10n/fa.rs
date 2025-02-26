use std::collections::HashMap;
use once_cell::sync::Lazy;
use rust_i18n::translation_hashmap;

/// Persian (fa) translations
pub static FA_TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    translation_hashmap!(
        "Help" => "راه‌نما",
        "Personal" => "شخصی",
        "Settings" => "تنظیمات",
        "Users" => "کاربران",
        "Admin" => "مدیر",
        "web services under your control" => "سرویس های تحت وب در کنترل شما",
        "ZIP download is turned off." => "دانلود به صورت فشرده غیر فعال است",
        "Files need to be downloaded one by one." => "فایل ها باید به صورت یکی یکی دانلود شوند",
        "Back to Files" => "بازگشت به فایل ها",
        "Selected files too large to generate zip file." => "فایل های انتخاب شده بزرگتر از آن هستند که بتوان یک فایل فشرده تولید کرد",
        "Application is not enabled" => "برنامه فعال نشده است",
        "Authentication error" => "خطا در اعتبار سنجی",
        "Token expired. Please reload page." => "رمز منقضی شده است. لطفا دوباره صفحه را بارگذاری نمایید.",
        "Files" => "پرونده‌ها",
        "Text" => "متن",
        "Images" => "تصاویر",
        "%s enter the database username." => "%s نام کاربری پایگاه داده را وارد نمایید.",
        "%s enter the database name." => "%s نام پایگاه داده را وارد نمایید.",
        "%s you may not use dots in the database name" => "%s شما نباید از نقطه در نام پایگاه داده استفاده نمایید.",
        "MS SQL username and/or password not valid: %s" => "نام کاربری و / یا رمزعبور MS SQL معتبر نیست:  %s",
        "You need to enter either an existing account or the administrator." => "شما نیاز به وارد کردن یک حساب کاربری موجود یا حساب مدیریتی دارید.",
        "MySQL username and/or password not valid" => "نام کاربری و / یا رمزعبور MySQL  معتبر نیست.",
        "DB Error: \"%s\"" => "خطای پایگاه داده: \"%s\"",
        "Offending command was: \"%s\"" => "دستور متخلف عبارت است از: \"%s\"",
        "MySQL user '%s'@'localhost' exists already." => "کاربرMySQL '%s'@'localhost' درحال حاضر موجود است.",
        "Drop this user from MySQL" => "این کاربر را از MySQL حذف نمایید.",
        "MySQL user '%s'@'%%' already exists" => "کاربر'%s'@'%%'  MySQL  در حال حاضر موجود است.",
        "Drop this user from MySQL." => "این کاربر را از MySQL حذف نمایید.",
        "Oracle connection could not be established" => "ارتباط اراکل نمیتواند برقرار باشد.",
        "Oracle username and/or password not valid" => "نام کاربری و / یا رمزعبور اراکل معتبر نیست.",
        "Offending command was: \"%s\", name: %s, password: %s" => "دستور متخلف عبارت است از: \"%s\"، نام: \"%s\"، رمزعبور:\"%s\"",
        "PostgreSQL username and/or password not valid" => "PostgreSQL نام کاربری و / یا رمزعبور معتبر نیست.",
        "Set an admin username." => "یک نام کاربری برای مدیر تنظیم نمایید.",
        "Set an admin password." => "یک رمزعبور برای مدیر تنظیم نمایید.",
        "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken." => "احتمالاً وب سرور شما طوری تنظیم نشده است که اجازه ی همگام سازی فایلها را بدهد زیرا به نظر میرسد رابط WebDAV از کار افتاده است.",
        "Please double check the <a href='%s'>installation guides</a>." => "لطفاً دوباره <a href='%s'>راهنمای نصب</a>را بررسی کنید.",
        "Could not find category \"%s\"" => "دسته بندی %s یافت نشد",
        "seconds ago" => "ثانیه‌ها پیش",
        "_%n minute ago_::_%n minutes ago_" => "دقیقه قبل",
        "_%n hour ago_::_%n hours ago_" => "ساعت قبل",
        "today" => "امروز",
        "yesterday" => "دیروز",
        "_%n day go_::_%n days ago_" => "روز قبل",
        "last month" => "ماه قبل",
        "_%n month ago_::_%n months ago_" => "ماه قبل",
        "last year" => "سال قبل",
        "years ago" => "سال‌های قبل"
    )
});

/// Persian (fa) plural forms definition
pub const FA_PLURAL_FORMS: &str = "nplurals=1; plural=0;";

/// Get translation for the given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    FA_TRANSLATIONS.get(key).copied()
}