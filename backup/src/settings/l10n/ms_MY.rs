use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Malaysian language translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Authentication error", "Ralat pengesahan");
    translations.insert("Email saved", "Emel disimpan");
    translations.insert("Invalid email", "Emel tidak sah");
    translations.insert("Language changed", "Bahasa diubah");
    translations.insert("Invalid request", "Permintaan tidak sah");
    translations.insert("Disable", "Nyahaktif");
    translations.insert("Enable", "Aktif");
    translations.insert("Error", "Ralat");
    translations.insert("Update", "Kemaskini");
    translations.insert("Saving...", "Simpan...");
    translations.insert("deleted", "dihapus");
    translations.insert("Groups", "Kumpulan");
    translations.insert("Delete", "Padam");
    translations.insert("__language_name__", "_nama_bahasa_");
    translations.insert("Security Warning", "Amaran keselamatan");
    translations.insert("Log", "Log");
    translations.insert("Log level", "Tahap Log");
    translations.insert("More", "Lanjutan");
    translations.insert("Add your App", "Tambah apps anda");
    translations.insert("Select an App", "Pilih aplikasi");
    translations.insert("See application page at apps.owncloud.com", "Lihat halaman applikasi di apps.owncloud.com");
    translations.insert("Password", "Kata laluan");
    translations.insert("Unable to change your password", "Gagal mengubah kata laluan anda ");
    translations.insert("Current password", "Kata laluan semasa");
    translations.insert("New password", "Kata laluan baru");
    translations.insert("Change password", "Ubah kata laluan");
    translations.insert("Email", "Email");
    translations.insert("Your email address", "Alamat emel anda");
    translations.insert("Fill in an email address to enable password recovery", "Isi alamat emel anda untuk membolehkan pemulihan kata laluan");
    translations.insert("Profile picture", "Gambar profil");
    translations.insert("Language", "Bahasa");
    translations.insert("Help translate", "Bantu terjemah");
    translations.insert("Create", "Buat");
    translations.insert("Other", "Lain");
    translations.insert("Username", "Nama pengguna");
    translations
});

/// Plural forms for Malaysian language
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";