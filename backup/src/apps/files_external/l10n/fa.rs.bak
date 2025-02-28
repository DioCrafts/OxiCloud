// l10n/fa.rs

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "مجوز دسترسی صادر شد");
        m.insert("Error configuring Dropbox storage", "خطا به هنگام تنظیم فضای دراپ باکس");
        m.insert("Grant access", " مجوز اعطا دسترسی");
        m.insert("Please provide a valid Dropbox app key and secret.", "لطفا یک کلید و کد امنیتی صحیح دراپ باکس وارد کنید.");
        m.insert("Error configuring Google Drive storage", "خطا به هنگام تنظیم فضای Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "خطا: \"smbclient\" نصب نشده است. نصب و راه اندازی سهام  CIFS/SMB امکان پذیر نمیباشد. لطفا از مدیریت سازمان خود برای راه اندازی آن درخواست نمایید.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "خطا: پشتیبانی FTP در PHP فعال نمی باشد یا نصب نشده است. نصب و راه اندازی از سهم های FTP امکان پذیر نمی باشد. لطفا از مدیر سیستم خود برای راه اندازی آن درخواست\nکنید.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "خطا: پشتیبانی Curl  فعال نمی باشد یا نصب نشده است. نصب و راه اندازی  ownCloud / WebDAV یا GoogleDrive امکان پذیر نیست. لطفا از مدیر سیستم خود برای نصب آن درخواست کنید.");
        m.insert("External Storage", "حافظه خارجی");
        m.insert("Folder name", "نام پوشه");
        m.insert("External storage", "حافظه خارجی");
        m.insert("Configuration", "پیکربندی");
        m.insert("Options", "تنظیمات");
        m.insert("Applicable", "قابل اجرا");
        m.insert("Add storage", "اضافه کردن حافظه");
        m.insert("None set", "تنظیم نشده");
        m.insert("All Users", "تمام کاربران");
        m.insert("Groups", "گروه ها");
        m.insert("Users", "کاربران");
        m.insert("Delete", "حذف");
        m.insert("Enable User External Storage", "فعال سازی حافظه خارجی کاربر");
        m.insert("Allow users to mount their own external storage", "اجازه به کاربران برای متصل کردن منابع ذخیره ی خارجی خودشان");
        m.insert("SSL root certificates", "گواهی های اصلی SSL ");
        m.insert("Import Root Certificate", "وارد کردن گواهی اصلی");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}