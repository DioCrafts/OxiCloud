use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "تعذّر حذف%s بشكل دائم");
        m.insert("Couldn't restore %s", "تعذّر استرجاع %s ");
        m.insert("Error", "خطأ");
        m.insert("Nothing in here. Your trash bin is empty!", "لا يوجد شيء هنا. سلة المهملات خاليه.");
        m.insert("Name", "اسم");
        m.insert("Restore", "استعيد");
        m.insert("Deleted", "تم الحذف");
        m.insert("Delete", "إلغاء");
        m.insert("Deleted Files", "الملفات المحذوفه");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=6; plural=n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5;";
}