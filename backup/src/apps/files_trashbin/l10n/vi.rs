use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Không thể óa %s vĩnh viễn");
        m.insert("Couldn't restore %s", "Không thể khôi phục %s");
        m.insert("Error", "Lỗi");
        m.insert("Nothing in here. Your trash bin is empty!", "Không có gì ở đây. Thùng rác của bạn rỗng!");
        m.insert("Name", "Tên");
        m.insert("Restore", "Khôi phục");
        m.insert("Deleted", "Đã xóa");
        m.insert("Delete", "Xóa");
        m.insert("Deleted Files", "File đã xóa");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}