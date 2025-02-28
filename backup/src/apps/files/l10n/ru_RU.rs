use lazy_static::lazy_static;
use rust_i18n::Plural;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Files", "Файлы");
        m.insert("Share", "Сделать общим");
        m.insert("Rename", "Переименовать");
        m.insert("Error", "Ошибка");
        m.insert("Upload", "Загрузка");
        m.insert("Save", "Сохранить");
        m.insert("Cancel upload", "Отмена загрузки");
        m.insert("Download", "Загрузка");
        m
    };

    pub static ref PLURAL_FORMS: Plural = Plural {
        nplurals: 3,
        plural_fn: |n| {
            if n % 10 == 1 && n % 100 != 11 {
                0
            } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
                1
            } else {
                2
            }
        },
    };

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec!["", "", ""]);
        m.insert("_%n file_::_%n files_", vec!["", "", ""]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec!["", "", ""]);
        m
    };
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, count: usize) -> &'static str {
    if let Some(forms) = PLURAL_TRANSLATIONS.get(key) {
        let idx = (PLURAL_FORMS.plural_fn)(count as u64) as usize;
        if idx < forms.len() && !forms[idx].is_empty() {
            return forms[idx];
        }
    }
    key
}