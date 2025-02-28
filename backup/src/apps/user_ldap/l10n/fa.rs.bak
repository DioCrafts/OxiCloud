use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::locale::Locale;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to clear the mappings.", "عدم موفقیت در پاک کردن نگاشت.");
        m.insert("Failed to delete the server configuration", "عملیات حذف پیکربندی سرور ناموفق ماند");
        m.insert("The configuration is valid and the connection could be established!", "پیکربندی معتبر است و ارتباط می تواند برقرار شود");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "پیکربندی معتبراست، اما اتصال شکست خورد. لطفا تنظیمات و اعتبارهای سرور را بررسی کنید.");
        m.insert("Deletion failed", "حذف کردن انجام نشد");
        m.insert("Keep settings?", "آیا تنظیمات ذخیره شود ؟");
        m.insert("Cannot add server configuration", "نمی توان پیکربندی سرور را اضافه نمود");
        m.insert("mappings cleared", "نگاشت پاک شده است");
        m.insert("Success", "موفقیت");
        m.insert("Error", "خطا");
        m.insert("Connection test succeeded", "تست اتصال با موفقیت انجام گردید");
        m.insert("Connection test failed", "تست اتصال ناموفق بود");
        m.insert("Do you really want to delete the current Server Configuration?", "آیا واقعا می خواهید پیکربندی کنونی سرور را حذف کنید؟");
        m.insert("Confirm Deletion", "تایید حذف");
        m.insert("_%s group found_::_%s groups found_", "");
        m.insert("_%s user found_::_%s users found_", "");
        m.insert("Save", "ذخیره");
        m.insert("Test Configuration", "امتحان پیکربندی");
        m.insert("Help", "راه‌نما");
        m.insert("Add Server Configuration", "افزودن پیکربندی سرور");
        m.insert("Host", "میزبانی");
        m.insert("Port", "درگاه");
        m.insert("User DN", "کاربر DN");
        m.insert("Password", "گذرواژه");
        m.insert("For anonymous access, leave DN and Password empty.", "برای دسترسی ناشناس، DN را رها نموده و رمزعبور را خالی بگذارید.");
        m.insert("One Base DN per line", "یک پایه DN در هر خط");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "شما می توانید پایه DN را برای کاربران و گروه ها در زبانه Advanced مشخص کنید.");
        m.insert("Back", "بازگشت");
        m.insert("Connection Settings", "تنظیمات اتصال");
        m.insert("Configuration Active", "پیکربندی فعال");
        m.insert("When unchecked, this configuration will be skipped.", "زمانیکه انتخاب نشود، این پیکربندی نادیده گرفته خواهد شد.");
        m.insert("User Login Filter", "فیلتر ورودی کاربر");
        m.insert("Backup (Replica) Host", "پشتیبان گیری (بدل) میزبان");
        m.insert("Backup (Replica) Port", "پشتیبان گیری (بدل) پورت");
        m.insert("Disable Main Server", "غیر فعال کردن سرور اصلی");
        m.insert("Case insensitve LDAP server (Windows)", "غیر حساس به بزرگی و کوچکی حروف LDAP سرور (ویندوز)");
        m.insert("Turn off SSL certificate validation.", "غیرفعال کردن اعتبار گواهی نامه SSL .");
        m.insert("Directory Settings", "تنظیمات پوشه");
        m.insert("User Display Name Field", "فیلد نام کاربر");
        m.insert("Base User Tree", "کاربر درخت پایه");
        m.insert("One User Base DN per line", "یک کاربر پایه DN در هر خط");
        m.insert("User Search Attributes", "ویژگی های جستجوی کاربر");
        m.insert("Optional; one attribute per line", "اختیاری؛ یک ویژگی در هر خط");
        m.insert("Group Display Name Field", "فیلد نام گروه");
        m.insert("Base Group Tree", "گروه درخت پایه ");
        m.insert("One Group Base DN per line", "یک گروه پایه DN در هر خط");
        m.insert("Group Search Attributes", "گروه صفات جستجو");
        m.insert("Group-Member association", "انجمن گروه کاربران");
        m.insert("Special Attributes", "ویژگی های مخصوص");
        m.insert("Quota Field", "سهمیه بندی انجام نشد.");
        m.insert("Quota Default", "سهمیه بندی پیش فرض");
        m.insert("in bytes", "در بایت");
        m.insert("Email Field", "ایمیل ارسال نشد.");
        m.insert("User Home Folder Naming Rule", "قانون نامگذاری پوشه خانه کاربر");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "خالی گذاشتن برای نام کاربری (پیش فرض). در غیر این صورت، تعیین یک ویژگی LDAP/AD.");
        m.insert("Internal Username", "نام کاربری داخلی");
        m.insert("Internal Username Attribute:", "ویژگی نام کاربری داخلی:");
        m.insert("Override UUID detection", "نادیده گرفتن تشخیص UUID ");
        m.insert("Username-LDAP User Mapping", "نام کاربری - نگاشت کاربر LDAP ");
        m.insert("Clear Username-LDAP User Mapping", "پاک کردن نام کاربری- LDAP  نگاشت کاربر ");
        m.insert("Clear Groupname-LDAP Group Mapping", "پاک کردن نام گروه -LDAP گروه نقشه برداری");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

pub fn register_locale() {
    let locale = Locale::new("fa", TRANSLATIONS.clone(), Some(get_plural_forms()));
    rust_i18n::set_locale("fa", locale);
}