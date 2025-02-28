// ms_MY.rs

use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Sunday", "Ahad");
    map.insert("Monday", "Isnin");
    map.insert("Tuesday", "Selasa");
    map.insert("Wednesday", "Rabu");
    map.insert("Thursday", "Khamis");
    map.insert("Friday", "Jumaat");
    map.insert("Saturday", "Sabtu");
    map.insert("January", "Januari");
    map.insert("February", "Februari");
    map.insert("March", "Mac");
    map.insert("April", "April");
    map.insert("May", "Mei");
    map.insert("June", "Jun");
    map.insert("July", "Julai");
    map.insert("August", "Ogos");
    map.insert("September", "September");
    map.insert("October", "Oktober");
    map.insert("November", "November");
    map.insert("December", "Disember");
    map.insert("Settings", "Tetapan");
    map.insert("Yes", "Ya");
    map.insert("No", "Tidak");
    map.insert("Ok", "Ok");
    map.insert("Cancel", "Batal");
    map.insert("Share", "Kongsi");
    map.insert("Error", "Ralat");
    map.insert("Password", "Kata laluan");
    map.insert("Warning", "Amaran");
    map.insert("Delete", "Padam");
    map.insert("Add", "Tambah");
    map.insert("Use the following link to reset your password: {link}", "Guna pautan berikut untuk menetapkan semula kata laluan anda: {link}");
    map.insert("You will receive a link to reset your password via Email.", "Anda akan menerima pautan untuk menetapkan semula kata laluan anda melalui emel");
    map.insert("Username", "Nama pengguna");
    map.insert("Your password was reset", "Kata laluan anda telah diset semula");
    map.insert("To login page", "Ke halaman log masuk");
    map.insert("New password", "Kata laluan baru");
    map.insert("Reset password", "Penetapan semula kata laluan");
    map.insert("Personal", "Peribadi");
    map.insert("Users", "Pengguna");
    map.insert("Apps", "Aplikasi");
    map.insert("Admin", "Admin");
    map.insert("Help", "Bantuan");
    map.insert("Access forbidden", "Larangan akses");
    map.insert("Cloud not found", "Awan tidak dijumpai");
    map.insert("Security Warning", "Amaran keselamatan");
    map.insert("Create an <strong>admin account</strong>", "buat <strong>akaun admin</strong>");
    map.insert("Advanced", "Maju");
    map.insert("Data folder", "Fail data");
    map.insert("Configure the database", "Konfigurasi pangkalan data");
    map.insert("will be used", "akan digunakan");
    map.insert("Database user", "Nama pengguna pangkalan data");
    map.insert("Database password", "Kata laluan pangkalan data");
    map.insert("Database name", "Nama pangkalan data");
    map.insert("Database host", "Hos pangkalan data");
    map.insert("Finish setup", "Setup selesai");
    map.insert("Log out", "Log keluar");
    map.insert("Lost your password?", "Hilang kata laluan?");
    map.insert("remember", "ingat");
    map.insert("Log in", "Log masuk");
    
    // Plural forms with empty translations
    map.insert("_%n minute ago_::_%n minutes ago_", "");
    map.insert("_%n hour ago_::_%n hours ago_", "");
    map.insert("_%n day ago_::_%n days ago_", "");
    map.insert("_%n month ago_::_%n months ago_", "");
    map.insert("_{count} file conflict_::_{count} file conflicts_", "");
    
    map
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}