use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "รหัสผ่าน");
        m.insert("%s shared the folder %s with you", "%s ได้แชร์โฟลเดอร์ %s ให้กับคุณ");
        m.insert("%s shared the file %s with you", "%s ได้แชร์ไฟล์ %s ให้กับคุณ");
        m.insert("Download", "ดาวน์โหลด");
        m.insert("Upload", "อัพโหลด");
        m.insert("Cancel upload", "ยกเลิกการอัพโหลด");
        m.insert("No preview available for", "ไม่สามารถดูตัวอย่างได้สำหรับ");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}