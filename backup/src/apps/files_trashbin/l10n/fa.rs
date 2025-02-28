use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "%s را نمی توان برای همیشه حذف کرد");
        m.insert("Couldn't restore %s", "%s را نمی توان بازگرداند");
        m.insert("Error", "خطا");
        m.insert("Nothing in here. Your trash bin is empty!", "هیچ چیزی اینجا نیست. سطل زباله ی شما خالی است.");
        m.insert("Name", "نام");
        m.insert("Restore", "بازیابی");
        m.insert("Deleted", "حذف شده");
        m.insert("Delete", "حذف");
        m.insert("Deleted Files", "فایلهای حذف شده");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}