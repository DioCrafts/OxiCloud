use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "Giriş kabul edildi");
    m.insert("Error configuring Dropbox storage", "Dropbox depo yapılandırma hatası");
    m.insert("Grant access", "Erişim sağlandı");
    m.insert("Please provide a valid Dropbox app key and secret.", "Lütfen Dropbox app key ve secret temin ediniz");
    m.insert("Error configuring Google Drive storage", "Google Drive depo yapılandırma hatası");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Uyarı:</b> \"smbclient\" kurulu değil. CIFS/SMB paylaşımlarını bağlama işlemi mümkün olmadı. Lütfen kurulumu için sistem yöneticinize danışın.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Uyarı:</b> PHP içerisinde FTP desteği etkin veya yüklü değil. FTP paylaşımlarını bağlama işlemi mümkün olmadı. Lütfen kurulumu için sistem yöneticinize danışın.");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Uyarı:</b> PHP içerisinde Curl desteği etkin veya yüklü değil. OwnCloud / WebDAV veya GoogleDrive bağlama işlemi mümkün olmadı. Lütfen kurulumu için sistem yöneticinizde danışın.");
    m.insert("External Storage", "Harici Depolama");
    m.insert("Folder name", "Dizin ismi");
    m.insert("External storage", "Harici depolama");
    m.insert("Configuration", "Yapılandırma");
    m.insert("Options", "Seçenekler");
    m.insert("Applicable", "Uygulanabilir");
    m.insert("Add storage", "Depo ekle");
    m.insert("None set", "Hiçbiri");
    m.insert("All Users", "Tüm Kullanıcılar");
    m.insert("Groups", "Gruplar");
    m.insert("Users", "Kullanıcılar");
    m.insert("Delete", "Sil");
    m.insert("Enable User External Storage", "Kullanıcılar için Harici Depolamayı Etkinleştir");
    m.insert("Allow users to mount their own external storage", "Kullanıcıların kendi harici depolamalarını bağlamalarına izin ver");
    m.insert("SSL root certificates", "SSL kök sertifikaları");
    m.insert("Import Root Certificate", "Kök Sertifikalarını İçe Aktar");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n > 1);";