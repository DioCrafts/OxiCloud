use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Akses diberikan");
        m.insert("Error configuring Dropbox storage", "Kesalahan dalam mengonfigurasi penyimpanan Dropbox");
        m.insert("Grant access", "Berikan hak akses");
        m.insert("Please provide a valid Dropbox app key and secret.", "Masukkan kunci dan sandi aplikasi Dropbox yang benar.");
        m.insert("Error configuring Google Drive storage", "Kesalahan dalam mengkonfigurasi penyimpanan Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Peringatan:</b> \"smbclient\" tidak terpasang. Mount direktori CIFS/SMB tidak dapat dilakukan. Silakan minta administrator sistem untuk memasangnya.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Peringatan:</b> Dukungan FTP di PHP tidak aktif atau tidak terpasang. Mount direktori FTP tidak dapat dilakukan. Silakan minta administrator sistem untuk memasangnya.");
        m.insert("External Storage", "Penyimpanan Eksternal");
        m.insert("Folder name", "Nama folder");
        m.insert("External storage", "Penyimpanan eksternal");
        m.insert("Configuration", "Konfigurasi");
        m.insert("Options", "Opsi");
        m.insert("Applicable", "Berlaku");
        m.insert("Add storage", "Tambahkan penyimpanan");
        m.insert("None set", "Tidak satupun di set");
        m.insert("All Users", "Semua Pengguna");
        m.insert("Groups", "Grup");
        m.insert("Users", "Pengguna");
        m.insert("Delete", "Hapus");
        m.insert("Enable User External Storage", "Aktifkan Penyimpanan Eksternal Pengguna");
        m.insert("Allow users to mount their own external storage", "Izinkan pengguna untuk mengaitkan penyimpanan eksternal mereka");
        m.insert("SSL root certificates", "Sertifikat root SSL");
        m.insert("Import Root Certificate", "Impor Sertifikat Root");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}