use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password successfully changed.", "Đã đổi mật khẩu.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Không thể đổi mật khẩu. Có lẽ do mật khẩu cũ không đúng.");
        m.insert("Saving...", "Đang lưu...");
        m.insert("Encryption", "Mã hóa");
        m.insert("Enabled", "Bật");
        m.insert("Disabled", "Tắt");
        m.insert("Change Password", "Đổi Mật khẩu");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";