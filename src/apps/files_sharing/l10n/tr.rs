use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("This share is password-protected", "Bu paylaşım parola korumalı");
        map.insert("The password is wrong. Try again.", "Parola hatalı. Yeniden deneyin.");
        map.insert("Password", "Parola");
        map.insert("Sorry, this link doesn't seem to work anymore.", "Üzgünüz, bu bağlantı artık çalışıyor gibi görünmüyor");
        map.insert("Reasons might be:", "Sebepleri şunlar olabilir:");
        map.insert("the item was removed", "öge kaldırılmış");
        map.insert("the link expired", "bağlantı süresi dolmuş");
        map.insert("sharing is disabled", "paylaşım devre dışı");
        map.insert("For more info, please ask the person who sent this link.", "Daha fazla bilgi için bu bağlantıyı aldığınız kişi ile iletişime geçin.");
        map.insert("%s shared the folder %s with you", "%s, %s klasörünü sizinle paylaştı");
        map.insert("%s shared the file %s with you", "%s, %s dosyasını sizinle paylaştı");
        map.insert("Download", "İndir");
        map.insert("Upload", "Yükle");
        map.insert("Cancel upload", "Yüklemeyi iptal et");
        map.insert("No preview available for", "Kullanılabilir önizleme yok");
        map.insert("Direct link", "Doğrudan bağlantı");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}