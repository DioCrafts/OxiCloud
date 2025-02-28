use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Error", "ข้อผิดพลาด");
        m.insert("Nothing in here. Your trash bin is empty!", "ไม่มีอะไรอยู่ในนี้ ถังขยะของคุณยังว่างอยู่");
        m.insert("Name", "ชื่อ");
        m.insert("Restore", "คืนค่า");
        m.insert("Deleted", "ลบแล้ว");
        m.insert("Delete", "ลบ");
        m.insert("Deleted Files", "ไฟล์ที่ลบทิ้ง");
        m
    };
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}