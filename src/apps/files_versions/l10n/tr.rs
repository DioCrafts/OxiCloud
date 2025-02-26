use rust_i18n::t;

// Defines Turkish translations
pub fn register_tr_translations() {
    rust_i18n::set_locale("tr");
    
    rust_i18n::translations! {
        tr {
            "Could not revert: %s" => "Geri alınamıyor: %s",
            "Versions" => "Sürümler",
            "Failed to revert {file} to revision {timestamp}." => "{file} dosyası {timestamp} gözden geçirmesine geri alınamadı.",
            "More versions..." => "Daha fazla sürüm...",
            "No other versions available" => "Başka sürüm mevcut değil",
            "Restore" => "Geri yükle"
        }
    }
}

// Set up plural forms for Turkish
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}