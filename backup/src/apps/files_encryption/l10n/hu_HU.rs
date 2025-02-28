use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully disabled", "Visszaállítási kulcs sikeresen kikapcsolva");
        m.insert("Password successfully changed.", "Jelszó sikeresen megváltoztatva.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "A jelszót nem lehet megváltoztatni! Lehet, hogy hibás volt a régi jelszó.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Kérlek győződj meg arról, hogy PHP 5.3.3 vagy annál frissebb van telepítve, valamint a PHP-hez tartozó OpenSSL bővítmény be van-e kapcsolva és az helyesen van-e konfigurálva! Ki lett kapcsolva ideiglenesen a titkosító alkalmazás.");
        m.insert("Saving...", "Mentés...");
        m.insert("personal settings", "személyes beállítások");
        m.insert("Encryption", "Titkosítás");
        m.insert("Enabled", "Bekapcsolva");
        m.insert("Disabled", "Kikapcsolva");
        m.insert("Change Password", "Jelszó megváltoztatása");
        m.insert("Old log-in password", "Régi bejelentkezési jelszó");
        m.insert("Current log-in password", "Jelenlegi bejelentkezési jelszó");
        m.insert("Update Private Key Password", "Privát kulcs jelszó frissítése");
        m.insert("Enable password recovery:", "Jelszó-visszaállítás bekapcsolása");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}