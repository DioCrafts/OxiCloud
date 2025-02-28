use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "Owncloud yazılımının bu sürümü ile uyumlu  olmadığı için \"%s\" uygulaması kurulamaz.");
        m.insert("No app name specified", "Uygulama adı belirtimedli");
        m.insert("Help", "Yardım");
        m.insert("Personal", "Kişisel");
        m.insert("Settings", "Ayarlar");
        m.insert("Users", "Kullanıcılar");
        m.insert("Admin", "Yönetici");
        m.insert("Failed to upgrade \"%s\".", "\"%s\" yükseltme başarısız oldu.");
        m.insert("Unknown filetype", "Bilinmeyen dosya türü");
        m.insert("Invalid image", "Geçersiz resim");
        m.insert("web services under your control", "Bilgileriniz güvenli ve şifreli");
        m.insert("cannot open \"%s\"", "\"%s\" açılamıyor");
        m.insert("ZIP download is turned off.", "ZIP indirmeleri kapatılmıştır.");
        m.insert("Files need to be downloaded one by one.", "Dosyaların birer birer indirilmesi gerekmektedir.");
        m.insert("Back to Files", "Dosyalara dön");
        m.insert("Selected files too large to generate zip file.", "Seçilen dosyalar bir zip dosyası oluşturmak için fazla büyüktür.");
        m.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Dosyaları ayrı ayrı, küçük parçalar halinde indirin ya da yöneticinizden yardım isteyin. ");
        m.insert("No source specified when installing app", "Uygulama kurulurken bir kaynak belirtilmedi");
        m.insert("No href specified when installing app from http", "Uygulama kuruluyorken http'de href belirtilmedi.");
        m.insert("No path specified when installing app from local file", "Uygulama yerel dosyadan kuruluyorken dosya yolu belirtilmedi");
        m.insert("Archives of type %s are not supported", "%s arşiv tipi desteklenmiyor");
        m.insert("Failed to open archive when installing app", "Uygulama kuruluyorken arşiv dosyası açılamadı");
        m.insert("App does not provide an info.xml file", "Uygulama info.xml dosyası sağlamıyor");
        m.insert("App can't be installed because of not allowed code in the App", "Uygulamada izin verilmeyeden kodlar olduğu için kurulamıyor.");
        m.insert("App can't be installed because it is not compatible with this version of ownCloud", "Owncloud versiyonunuz ile uyumsuz olduğu için uygulama kurulamıyor.");
        m.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "Uygulama kurulamıyor. Çünkü  \"non shipped\" uygulamalar için <shipped>true</shipped> tag içermektedir.");
        m.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "Uygulama kurulamıyor çünkü info.xml/version ile uygulama marketde belirtilen sürüm aynı değil.");
        m.insert("App directory already exists", "App dizini zaten mevcut");
        m.insert("Can't create app folder. Please fix permissions. %s", "app dizini oluşturulamıyor. Lütfen izinleri düzeltin. %s");
        m.insert("Application is not enabled", "Uygulama etkinleştirilmedi");
        m.insert("Authentication error", "Kimlik doğrulama hatası");
        m.insert("Token expired. Please reload page.", "Jetonun süresi geçti. Lütfen sayfayı yenileyin.");
        m.insert("Files", "Dosyalar");
        m.insert("Text", "Metin");
        m.insert("Images", "Resimler");
        m.insert("%s enter the database username.", "%s veritabanı kullanıcı adını gir.");
        m.insert("%s enter the database name.", "%s veritabanı adını gir.");
        m.insert("%s you may not use dots in the database name", "%s veritabanı adında nokta kullanamayabilirsiniz");
        m.insert("MS SQL username and/or password not valid: %s", "MS SQL kullanıcı adı ve/veya parolası geçersiz: %s");
        m.insert("You need to enter either an existing account or the administrator.", "Bir konto veya kullanici birlemek ihtiyacin. ");
        m.insert("MySQL username and/or password not valid", "MySQL kullanıcı adı ve/veya parolası geçerli değil");
        m.insert("DB Error: \"%s\"", "DB Hata: ''%s''");
        m.insert("Offending command was: \"%s\"", "Komut rahasiz ''%s''. ");
        m.insert("MySQL user '%s'@'localhost' exists already.", "MySQL kullanici '%s @local host zatan var. ");
        m.insert("Drop this user from MySQL", "Bu kullanici MySQLden list disari koymak. ");
        m.insert("MySQL user '%s'@'%%' already exists", "MySQL kullanici '%s @ % % zaten var (zaten yazili)");
        m.insert("Drop this user from MySQL.", "Bu kulanıcıyı MySQL veritabanından kaldır");
        m.insert("Oracle connection could not be established", "Oracle bağlantısı kurulamadı");
        m.insert("Oracle username and/or password not valid", "Adi klullanici ve/veya parola Oracle mantikli değildir. ");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Hatalı komut: \"%s\", ad: %s, parola: %s");
        m.insert("PostgreSQL username and/or password not valid", "PostgreSQL adi kullanici ve/veya parola yasal degildir. ");
        m.insert("Set an admin username.", "Bir adi kullanici vermek. ");
        m.insert("Set an admin password.", "Parola yonetici birlemek. ");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Web sunucunuz dosya transferi için düzgün bir şekilde yapılandırılmamış. WevDAV arabirimini sorunlu gözüküyor.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Lütfen <a href='%s'>kurulum kılavuzlarını</a> iki kez kontrol edin.");
        m.insert("Could not find category \"%s\"", "\"%s\" kategorisi bulunamadı");
        m.insert("seconds ago", "saniye önce");
        m.insert("_%n minute ago_::_%n minutes ago_", "%n dakika önce");
        m.insert("_%n hour ago_::_%n hours ago_", "%n saat önce");
        m.insert("today", "bugün");
        m.insert("yesterday", "dün");
        m.insert("_%n day go_::_%n days ago_", "%n gün önce");
        m.insert("last month", "geçen ay");
        m.insert("_%n month ago_::_%n months ago_", "%n ay önce");
        m.insert("last year", "geçen yıl");
        m.insert("years ago", "yıl önce");
        m.insert("Caused by:", "Neden olan:");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn has_translation(key: &str) -> bool {
    TRANSLATIONS.contains_key(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}