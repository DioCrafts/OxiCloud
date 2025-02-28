use std::collections::HashMap;
use once_cell::sync::Lazy;

// Definimos las traducciones como un HashMap estático lazy
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("%s shared »%s« with you", "%s  sizinle »%s« paylaşımında bulundu");
    translations.insert("Couldn't send mail to following users: %s ", "Şu kullanıcılara posta gönderilemedi: %s");
    translations.insert("Turned on maintenance mode", "Bakım kipi etkinleştirildi");
    translations.insert("Turned off maintenance mode", "Bakım kipi kapatıldı");
    translations.insert("Updated database", "Veritabanı güncellendi");
    translations.insert("Updating filecache, this may take really long...", "Dosya önbelleği güncelleniyor. Bu, gerçekten uzun sürebilir.");
    translations.insert("Updated filecache", "Dosya önbelleği güncellendi");
    translations.insert("... %d%% done ...", "%%%d tamamlandı ...");
    translations.insert("No image or file provided", "Resim veya dosya belirtilmedi");
    translations.insert("Unknown filetype", "Bilinmeyen dosya türü");
    translations.insert("Invalid image", "Geçersiz resim");
    translations.insert("No temporary profile picture available, try again", "Kullanılabilir geçici profil resmi yok, tekrar deneyin");
    translations.insert("No crop data provided", "Kesme verisi sağlanmamış");
    translations.insert("Sunday", "Pazar");
    translations.insert("Monday", "Pazartesi");
    translations.insert("Tuesday", "Salı");
    translations.insert("Wednesday", "Çarşamba");
    translations.insert("Thursday", "Perşembe");
    translations.insert("Friday", "Cuma");
    translations.insert("Saturday", "Cumartesi");
    translations.insert("January", "Ocak");
    translations.insert("February", "Şubat");
    translations.insert("March", "Mart");
    translations.insert("April", "Nisan");
    translations.insert("May", "Mayıs");
    translations.insert("June", "Haziran");
    translations.insert("July", "Temmuz");
    translations.insert("August", "Ağustos");
    translations.insert("September", "Eylül");
    translations.insert("October", "Ekim");
    translations.insert("November", "Kasım");
    translations.insert("December", "Aralık");
    translations.insert("Settings", "Ayarlar");
    translations.insert("seconds ago", "saniye önce");
    translations.insert("today", "bugün");
    translations.insert("yesterday", "dün");
    translations.insert("last month", "geçen ay");
    translations.insert("months ago", "ay önce");
    translations.insert("last year", "geçen yıl");
    translations.insert("years ago", "yıl önce");
    translations.insert("Choose", "seç");
    translations.insert("Error loading file picker template: {error}", "Dosya seçici şablonu yüklenirken hata: {error}");
    translations.insert("Yes", "Evet");
    translations.insert("No", "Hayır");
    translations.insert("Ok", "Tamam");
    translations.insert("Error loading message template: {error}", "İleti şablonu yüklenirken hata: {error}");
    translations.insert("One file conflict", "Bir dosya çakışması");
    translations.insert("Which files do you want to keep?", "Hangi dosyaları saklamak istiyorsunuz?");
    translations.insert("If you select both versions, the copied file will have a number added to its name.", "Eğer iki sürümü de seçerseniz, kopyalanan dosya ismine eklenmiş bir sayı içerecektir.");
    translations.insert("Cancel", "İptal");
    translations.insert("Continue", "Devam et");
    translations.insert("(all selected)", "(tümü seçildi)");
    translations.insert("Error loading file exists template", "Dosya mevcut şablonu yüklenirken hata");
    translations.insert("Shared", "Paylaşılan");
    translations.insert("Share", "Paylaş");
    translations.insert("Error", "Hata");
    translations.insert("Error while sharing", "Paylaşım sırasında hata  ");
    translations.insert("Error while unsharing", "Paylaşım iptal ediliyorken hata");
    translations.insert("Error while changing permissions", "İzinleri değiştirirken hata oluştu");
    translations.insert("Shared with you and the group {group} by {owner}", " {owner} tarafından sizinle ve {group} ile paylaştırılmış");
    translations.insert("Shared with you by {owner}", "{owner} trafından sizinle paylaştırıldı");
    translations.insert("Share with user or group …", "Kullanıcı veya grup ile paylaş..");
    translations.insert("Share link", "Paylaşma bağlantısı");
    translations.insert("Password protect", "Parola koruması");
    translations.insert("Password", "Parola");
    translations.insert("Allow Public Upload", "Herkes tarafından yüklemeye izin ver");
    translations.insert("Email link to person", "Bağlantıyı e-posta ile gönder");
    translations.insert("Send", "Gönder");
    translations.insert("Set expiration date", "Son kullanma tarihini ayarla");
    translations.insert("Expiration date", "Son kullanım tarihi");
    translations.insert("Share via email:", "E-posta ile paylaş");
    translations.insert("No people found", "Kişi bulunamadı");
    translations.insert("group", "grup");
    translations.insert("Resharing is not allowed", "Tekrar paylaşmaya izin verilmiyor");
    translations.insert("Shared in {item} with {user}", " {item} içinde  {user} ile paylaşılanlarlar");
    translations.insert("Unshare", "Paylaşılmayan");
    translations.insert("notify by email", "e-posta ile bildir");
    translations.insert("can edit", "düzenleyebilir");
    translations.insert("access control", "erişim kontrolü");
    translations.insert("create", "oluştur");
    translations.insert("update", "güncelle");
    translations.insert("delete", "sil");
    translations.insert("share", "paylaş");
    translations.insert("Password protected", "Parola korumalı");
    translations.insert("Error unsetting expiration date", "Geçerlilik tarihi tanımlama kaldırma hatası");
    translations.insert("Error setting expiration date", "Geçerlilik tarihi tanımlama hatası");
    translations.insert("Sending ...", "Gönderiliyor...");
    translations.insert("Email sent", "E-posta gönderildi");
    translations.insert("Warning", "Uyarı");
    translations.insert("The object type is not specified.", "Nesne türü belirtilmemiş.");
    translations.insert("Enter new", "Yeni girin");
    translations.insert("Delete", "Sil");
    translations.insert("Add", "Ekle");
    translations.insert("Edit tags", "Etiketleri düzenle");
    translations.insert("Error loading dialog template: {error}", "İletişim şablonu yüklenirken hata: {error}");
    translations.insert("No tags selected for deletion.", "Silmek için bir etiket seçilmedi.");
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Güncelleme başarılı olmadı. Lütfen bu hatayı bildirin <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.");
    translations.insert("The update was successful. Redirecting you to ownCloud now.", "Güncelleme başarılı. ownCloud'a yönlendiriliyor.");
    translations.insert("%s password reset", "%s parola sıfırlama");
    translations.insert("Use the following link to reset your password: {link}", "Bu bağlantıyı kullanarak parolanızı sıfırlayın: {link}");
    translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Parolanızı değiştirme bağlantısı e-posta adresinize gönderildi.<br>Eğer makül bir süre içerisinde mesajı almadıysanız spam/junk dizinini kontrol ediniz.<br> Eğer orada da bulamazsanız sistem yöneticinize sorunuz.");
    translations.insert("Request failed!<br>Did you make sure your email/username was right?", "Isteği başarısız oldu!<br>E-posta / kullanıcı adınızı doğru olduğundan emin misiniz?");
    translations.insert("You will receive a link to reset your password via Email.", "Parolanızı sıfırlamak için bir bağlantıyı e-posta olarak alacaksınız.");
    translations.insert("Username", "Kullanıcı Adı");
    translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Dosyalarınız şifrelenmiş. Eğer kurtarma anahtarını aktif etmediyseniz parola sıfırlama işleminden sonra verilerinize erişmeniz imkansız olacak. Eğer ne yaptığınızdan emin değilseniz, devam etmeden önce sistem yöneticiniz ile irtibata geçiniz. Gerçekten devam etmek istiyor musunuz?");
    translations.insert("Yes, I really want to reset my password now", "Evet,Şu anda parolamı sıfırlamak istiyorum.");
    translations.insert("Reset", "Sıfırla");
    translations.insert("Your password was reset", "Parolanız sıfırlandı");
    translations.insert("To login page", "Giriş sayfasına git");
    translations.insert("New password", "Yeni parola");
    translations.insert("Reset password", "Parolayı sıfırla");
    translations.insert("Personal", "Kişisel");
    translations.insert("Users", "Kullanıcılar");
    translations.insert("Apps", "Uygulamalar");
    translations.insert("Admin", "Yönetici");
    translations.insert("Help", "Yardım");
    translations.insert("Error loading tags", "Etiketler yüklenirken hata");
    translations.insert("Tag already exists", "Etiket zaten mevcut");
    translations.insert("Error deleting tag(s)", "Etiket(ler) silinirken hata");
    translations.insert("Error tagging", "Etiketleme hatası");
    translations.insert("Error untagging", "Etiket kaldırılırken hata");
    translations.insert("Error favoriting", "Beğenilirken hata");
    translations.insert("Error unfavoriting", "Beğeniden kaldırılırken hata");
    translations.insert("Access forbidden", "Erişim yasaklı");
    translations.insert("Cloud not found", "Bulut bulunamadı");
    translations.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n", "Merhaba,\n\nSadece %s sizinle %s paylaşımını yaptığını bildiriyoruz.\nBuradan bakabilirsiniz: %s\n\n");
    translations.insert("The share will expire on %s.\n\n", "Paylaşım %s tarihinde bitecektir.\n\n");
    translations.insert("Cheers!", "Şerefe!");
    translations.insert("Security Warning", "Güvenlik Uyarisi");
    translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "PHP sürümünüz NULL Byte saldırısına açık (CVE-2006-7243)");
    translations.insert("Please update your PHP installation to use %s securely.", "%s güvenli olarak kullanmak için, lütfen PHP kurulumunuzu güncelleyin.");
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Güvenli rasgele sayı üreticisi bulunamadı. Lütfen PHP OpenSSL eklentisini etkinleştirin.");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Güvenli rasgele sayı üreticisi olmadan saldırganlar parola sıfırlama simgelerini tahmin edip hesabınızı ele geçirebilir.");
    translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Veri klasörünüz ve dosyalarınız .htaccess dosyası çalışmadığı için internet'ten erişime açık.");
    translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Server'ınızı nasıl ayarlayacağınıza dair bilgi için, lütfen <a href=\"%s\" target=\"_blank\">dokümantasyon sayfasını</a> ziyaret edin.");
    translations.insert("Create an <strong>admin account</strong>", "Bir <strong>yönetici hesabı</strong> oluşturun");
    translations.insert("Advanced", "Gelişmiş");
    translations.insert("Data folder", "Veri klasörü");
    translations.insert("Configure the database", "Veritabanını ayarla");
    translations.insert("will be used", "kullanılacak");
    translations.insert("Database user", "Veritabanı kullanıcı adı");
    translations.insert("Database password", "Veritabanı parolası");
    translations.insert("Database name", "Veritabanı adı");
    translations.insert("Database tablespace", "Veritabanı tablo alanı");
    translations.insert("Database host", "Veritabanı sunucusu");
    translations.insert("Finish setup", "Kurulumu tamamla");
    translations.insert("Finishing …", "Tamamlanıyor ..");
    translations.insert("%s is available. Get more information on how to update.", "%s mevcuttur. Güncelleştirme hakkında daha fazla bilgi alın.");
    translations.insert("Log out", "Çıkış yap");
    translations.insert("Automatic logon rejected!", "Otomatik oturum açma reddedildi!");
    translations.insert("If you did not change your password recently, your account may be compromised!", "Yakın zamanda parolanızı değiştirmediyseniz hesabınız tehlikede olabilir!");
    translations.insert("Please change your password to secure your account again.", "Hesabınızı tekrar güvene almak için lütfen parolanızı değiştirin.");
    translations.insert("Server side authentication failed!", "Sunucu taraflı yetkilendirme başarısız!");
    translations.insert("Please contact your administrator.", "Lütfen sistem yöneticisi ile iletişime geçin.");
    translations.insert("Lost your password?", "Parolanızı mı unuttunuz?");
    translations.insert("remember", "hatırla");
    translations.insert("Log in", "Giriş yap");
    translations.insert("Alternative Logins", "Alternatif Girişler");
    translations.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>", "Merhaba, <br><br> %s sizinle »%s« paylaşımında bulundu.<br><a href=\"%s\">Paylaşımı gör!</a><br><br>İyi günler!");
    translations.insert("The share will expire on %s.<br><br>", "Bu paylaşım %s tarihinde dolacaktır.<br><br>");
    translations.insert("Updating ownCloud to version %s, this may take a while.", "Owncloud %s versiyonuna güncelleniyor. Biraz zaman alabilir.");
    translations.insert("This ownCloud instance is currently being updated, which may take a while.", "Bu ownCloud örneği şu anda güncelleniyor, bu biraz zaman alabilir.");
    translations.insert("Please reload this page after a short time to continue using ownCloud.", "ownCloud kullanmaya devam etmek için kısa bir süre sonra lütfen sayfayı yenileyin.");
    translations.insert("Contact your system administrator if this message persists or appeared unexpectedly.", "Eğer bu ileti görünmeye devam ederse veya beklenmedik şekilde ortaya çıkmışsa sistem yöneticinizle iletişime geçin.");
    translations.insert("Thank you for your patience.", "Sabrınız için teşekkür ederiz.");
    
    // Manejamos las traducciones con plurales
    translations.insert("_%n minute ago_::_%n minutes ago_", "%n dakika önce");
    translations.insert("_%n hour ago_::_%n hours ago_", "%n saat önce");
    translations.insert("_%n day ago_::_%n days ago_", "%n gün önce");
    translations.insert("_%n month ago_::_%n months ago_", "%n ay önce");
    translations.insert("_{count} file conflict_::_{count} file conflicts_", "{count} dosya çakışması");
    translations.insert("({count} selected)", "({count} seçildi)");

    translations
});

/// Defines plural forms rule for Turkish language
pub fn get_plural_form(n: i64) -> usize {
    if n > 1 { 1 } else { 0 }
}

/// Gets translation for a string
pub fn translate(text: &str) -> &'static str {
    TRANSLATIONS.get(text).copied().unwrap_or(text)
}

/// Gets plural translation for a string based on count
pub fn translate_plural(singular: &str, plural: &str, count: i64) -> &'static str {
    let key = format!("{}::{}", singular, plural);
    let form = get_plural_form(count);
    TRANSLATIONS.get(key.as_str()).copied().unwrap_or(if form == 0 { singular } else { plural })
}