use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// Translations for Turkish
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "%s alıcı olarak silinemedi");
        m.insert("Couldn't restore %s", "%s eri yüklenemedi");
        m.insert("Error", "Hata");
        m.insert("restored", "geri yüklendi");
        m.insert("Nothing in here. Your trash bin is empty!", "Burada hiçbir şey yok. Çöp kutunuz tamamen boş!");
        m.insert("Name", "İsim");
        m.insert("Restore", "Geri yükle");
        m.insert("Deleted", "Silindi");
        m.insert("Delete", "Sil");
        m.insert("Deleted Files", "Silinen Dosyalar");
        m
    };

    /// Plural forms rule for Turkish
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}