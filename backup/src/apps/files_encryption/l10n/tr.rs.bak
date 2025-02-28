use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Kurtarma anahtarı başarıyla etkinleştirildi");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Kurtarma anahtarı etkinleştirilemedi. Lütfen kurtarma anahtarı parolanızı kontrol edin!");
        m.insert("Recovery key successfully disabled", "Kurtarma anahtarı başarıyla devre dışı bırakıldı");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Kurtarma anahtarı devre dışı bırakılamadı. Lütfen kurtarma anahtarı parolanızı kontrol edin!");
        m.insert("Password successfully changed.", "Şifreniz başarıyla değiştirildi.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Parola değiştirilemedi. Eski parolanız doğru olmayabilir");
        m.insert("Private key password successfully updated.", "Gizli anahtar parolası başarıyla güncellendi");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Gizli anahtar parolası güncellenemedi. Eski parola hatalı olabilir.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Şifreleme uygulaması başlatılamadı! Oturumunuz sırasında şifreleme uygulaması tekrar etkinleştirilmiş olabilir. Lütfen şifreleme uygulamasını başlatmak için oturumu kapatıp yeniden oturum açmayı deneyin.");
        m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "Gizli anahtarınız geçerli değil! Muhtemelen parolanız ownCloud sistemi %s dışarısında değiştirildi (örn. şirket dizininde). Gizli anahtar parolanızı kişisel ayarlarınızda güncelleyerek şifreli dosyalarınıza erişimi kurtarabilirsiniz.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Bu dosya muhtemelen bir paylaşılan dosya olduğundan şifresi çözülemiyor. Lütfen dosyayı sizinle bir daha paylaşması için dosya sahibi ile iletişime geçin.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Bilinmeyen hata. Lütfen sistem ayarlarınızı denetleyin veya yöneticiniz ile iletişime geçin");
        m.insert("Missing requirements.", "Gereklilikler eksik.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "PHP 5.3.3 veya daha sürümü ile birlikte OpenSSL ve OpenSSL PHP uzantısının birlikte etkin olduğunu ve doğru bir şekilde yapılandırıldığından emin olun. Şimdilik şifreleme uygulaması devre dışı bırakıldı");
        m.insert("Following users are not set up for encryption:", "Aşağıdaki kullanıcılar şifreleme için ayarlanmadılar:");
        m.insert("Saving...", "Kaydediliyor...");
        m.insert("Go directly to your ", "Doğrudan şuraya gidin:");
        m.insert("personal settings", "kişisel ayarlar");
        m.insert("Encryption", "Şifreleme");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Kurtarma anahtarını etkinleştir (parola kaybı durumunda kullanıcı dosyalarının kurtarılmasına izin verir):");
        m.insert("Recovery key password", "Kurtarma anahtarı parolası");
        m.insert("Repeat Recovery key password", "Kurtarma anahtarı parolasını yenileyin");
        m.insert("Enabled", "Etkinleştirildi");
        m.insert("Disabled", "Devre dışı");
        m.insert("Change recovery key password:", "Kurtarma anahtarı parolasını değiştir:");
        m.insert("Old Recovery key password", "Eski Kurtarma anahtar parolası");
        m.insert("New Recovery key password", "Yeni Kurtarma anahtar parolası");
        m.insert("Repeat New Recovery key password", "Yeni Kurtarma anahtarı parolasını yenileyin");
        m.insert("Change Password", "Parola değiştir");
        m.insert("Your private key password no longer match your log-in password:", "Özel anahtar parolanız artık oturum açma parolanızla eşleşmiyor:");
        m.insert("Set your old private key password to your current log-in password.", "Eski özel anahtar parolanızı geçerli oturum açma parolanız olarak ayarlayın.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Eğer eski parolanızı hatırlamıyorsanız, yöneticinizden dosyalarınızı kurtarmasını talep edebilirsiniz.");
        m.insert("Old log-in password", "Eski oturum açma parolası");
        m.insert("Current log-in password", "Geçerli oturum açma parolası");
        m.insert("Update Private Key Password", "Özel Anahtar Parolasını Güncelle");
        m.insert("Enable password recovery:", "Parola kurtarmayı etkinleştir:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Bu seçeneği etkinleştirmek, parola kaybı durumunda şifrelenmiş dosyalarınıza erişimi yeniden kazanmanızı sağlayacaktır");
        m.insert("File recovery settings updated", "Dosya kurtarma ayarları güncellendi");
        m.insert("Could not update file recovery", "Dosya kurtarma güncellenemedi");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}