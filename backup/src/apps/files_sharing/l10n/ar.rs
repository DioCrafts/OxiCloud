use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: std::collections::HashMap<&'static str, &'static str> = {
        let mut m = std::collections::HashMap::new();
        m.insert("Password", "كلمة المرور");
        m.insert("%s shared the folder %s with you", "%s شارك المجلد %s معك");
        m.insert("%s shared the file %s with you", "%s شارك الملف %s معك");
        m.insert("Download", "تحميل");
        m.insert("Upload", "رفع");
        m.insert("Cancel upload", "إلغاء رفع الملفات");
        m.insert("No preview available for", "لا يوجد عرض مسبق لـ");
        m
    };
}

#[derive(Clone, Copy)]
pub struct PluralForms;

impl rust_i18n::plurals::PluralForms for PluralForms {
    fn get_plural_form(&self, n: f64) -> usize {
        let n = n as i64;
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else if n == 2 {
            2
        } else if n % 100 >= 3 && n % 100 <= 10 {
            3
        } else if n % 100 >= 11 && n % 100 <= 99 {
            4
        } else {
            5
        }
    }

    fn get_plural_forms_count(&self) -> usize {
        6
    }
}