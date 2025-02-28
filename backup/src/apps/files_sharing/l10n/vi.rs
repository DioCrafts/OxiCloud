use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Mật khẩu");
        m.insert("%s shared the folder %s with you", "%s đã chia sẻ thư mục %s với bạn");
        m.insert("%s shared the file %s with you", "%s đã chia sẻ tập tin %s với bạn");
        m.insert("Download", "Tải về");
        m.insert("Upload", "Tải lên");
        m.insert("Cancel upload", "Hủy upload");
        m.insert("No preview available for", "Không có xem trước cho");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}