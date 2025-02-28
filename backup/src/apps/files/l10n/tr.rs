use std::collections::HashMap;
use rust_i18n::t;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists", "%s taşınamadı. Bu isimde dosya zaten var.");
    translations.insert("Could not move %s", "%s taşınamadı");
    translations.insert("File name cannot be empty.", "Dosya adı boş olamaz.");
    translations.insert("File name must not contain \"/\". Please choose a different name.", "Dosya adı \"/\" içermemelidir. Lütfen farklı bir isim seçin.");
    translations.insert("The name %s is already used in the folder %s. Please choose a different name.", "%s ismi zaten %s klasöründe kullanılıyor. Lütfen farklı bir isim seçin.");
    translations.insert("Not a valid source", "Geçerli bir kaynak değil");
    translations.insert("Error while downloading %s to %s", "%s, %s içine indirilirken hata");
    translations.insert("Error when creating the file", "Dosya oluşturulurken hata");
    translations.insert("Folder name cannot be empty.", "Klasör adı boş olamaz.");
    translations.insert("Folder name must not contain \"/\". Please choose a different name.", "Klasör adı \"/\" içermemelidir. Lütfen farklı bir isim seçin.");
    translations.insert("Error when creating the folder", "Klasör oluşturulurken hata");
    translations.insert("Unable to set upload directory.", "Yükleme dizini tanımlanamadı.");
    translations.insert("Invalid Token", "Geçeriz simge");
    translations.insert("No file was uploaded. Unknown error", "Dosya yüklenmedi. Bilinmeyen hata");
    translations.insert("There is no error, the file uploaded with success", "Dosya başarıyla yüklendi, hata oluşmadı");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "php.ini dosyasında upload_max_filesize ile belirtilen dosya yükleme sınırı aşıldı.");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Yüklenecek dosyanın boyutu HTML formunda belirtilen MAX_FILE_SIZE limitini aşıyor");
    translations.insert("The uploaded file was only partially uploaded", "Dosya kısmen karşıya yüklenebildi");
    translations.insert("No file was uploaded", "Hiç dosya gönderilmedi");
    translations.insert("Missing a temporary folder", "Geçici dizin eksik");
    translations.insert("Failed to write to disk", "Diske yazılamadı");
    translations.insert("Not enough storage available", "Yeterli disk alanı yok");
    translations.insert("Upload failed. Could not get file info.", "Yükleme başarısız. Dosya bilgisi alınamadı.");
    translations.insert("Upload failed. Could not find uploaded file", "Yükleme başarısız. Yüklenen dosya bulunamadı");
    translations.insert("Invalid directory.", "Geçersiz dizin.");
    translations.insert("Files", "Dosyalar");
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Bir dizin veya 0 bayt olduğundan {filename} yüklenemedi");
    translations.insert("Not enough space available", "Yeterli disk alanı yok");
    translations.insert("Upload cancelled.", "Yükleme iptal edildi.");
    translations.insert("Could not get result from server.", "Sunucudan sonuç alınamadı.");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Dosya yükleme işlemi sürüyor. Şimdi sayfadan ayrılırsanız işleminiz iptal olur.");
    translations.insert("URL cannot be empty", "URL boş olamaz");
    translations.insert("In the home folder 'Shared' is a reserved filename", "Ev klasöründeki 'Paylaşılan', ayrılmış bir dosya adıdır");
    translations.insert("{new_name} already exists", "{new_name} zaten mevcut");
    translations.insert("Could not create file", "Dosya oluşturulamadı");
    translations.insert("Could not create folder", "Klasör oluşturulamadı");
    translations.insert("Share", "Paylaş");
    translations.insert("Delete permanently", "Kalıcı olarak sil");
    translations.insert("Rename", "İsim değiştir.");
    translations.insert("Pending", "Bekliyor");
    translations.insert("Could not rename file", "Dosya adlandırılamadı");
    translations.insert("replaced {new_name} with {old_name}", "{new_name} ismi {old_name} ile değiştirildi");
    translations.insert("undo", "geri al");
    translations.insert("_%n folder_::_%n folders_", "%n dizin");
    translations.insert("_%n file_::_%n files_", "%n dosya");
    translations.insert("{dirs} and {files}", "{dirs} ve {files}");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "%n dosya yükleniyor");
    translations.insert("'.' is an invalid file name.", "'.' geçersiz dosya adı.");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Geçersiz isim, '\\', '/', '<', '>', ':', '\"', '|', '?' ve '*' karakterlerine izin verilmemektedir.");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "Depolama alanınız dolu, artık dosyalar güncellenmeyecek yada senkronizasyon edilmeyecek.");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "Depolama alanınız neredeyse dolu ({usedSpacePercent}%)");
    translations.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "Şifreleme Uygulaması etkin ancak anahtarlarınız başlatılmamış. Lütfen oturumu kapatıp yeniden açın");
    translations.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Şifreleme Uygulaması için geçersiz özel anahtar. Lütfen şifreli dosyalarınıza erişimi tekrar kazanabilmek için kişisel ayarlarınızdan özel anahtar parolanızı güncelleyin.");
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Şifreleme işlemi durduruldu ancak dosyalarınız şifreli. Dosyalarınızın şifresini kaldırmak için lütfen kişisel ayarlar kısmına geçiniz.");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "İndirmeniz hazırlanıyor. Dosya büyük ise biraz zaman alabilir.");
    translations.insert("Error moving file", "Dosya taşıma hatası");
    translations.insert("Error", "Hata");
    translations.insert("Name", "İsim");
    translations.insert("Size", "Boyut");
    translations.insert("Modified", "Değiştirilme");
    translations.insert("Invalid folder name. Usage of 'Shared' is reserved.", "Geçersiz dizin adı. Shared isminin kullanımı Owncloud tarafından rezerve edilmiştir.");
    translations.insert("%s could not be renamed", "%s yeniden adlandırılamadı");
    translations.insert("Upload", "Yükle");
    translations.insert("File handling", "Dosya taşıma");
    translations.insert("Maximum upload size", "Maksimum yükleme boyutu");
    translations.insert("max. possible: ", "mümkün olan en fazla: ");
    translations.insert("Needed for multi-file and folder downloads.", "Çoklu dosya ve dizin indirmesi için gerekli.");
    translations.insert("Enable ZIP-download", "ZIP indirmeyi aktif et");
    translations.insert("0 is unlimited", "0 limitsiz demektir");
    translations.insert("Maximum input size for ZIP files", "ZIP dosyaları için en fazla girdi sayısı");
    translations.insert("Save", "Kaydet");
    translations.insert("New", "Yeni");
    translations.insert("Text file", "Metin dosyası");
    translations.insert("Folder", "Klasör");
    translations.insert("From link", "Bağlantıdan");
    translations.insert("Deleted files", "Silinmiş dosyalar");
    translations.insert("Cancel upload", "Yüklemeyi iptal et");
    translations.insert("You don't have permission to upload or create files here", "Buraya dosya yükleme veya oluşturma izniniz yok");
    translations.insert("Nothing in here. Upload something!", "Burada hiçbir şey yok. Bir şeyler yükleyin!");
    translations.insert("Download", "İndir");
    translations.insert("Unshare", "Paylaşılmayan");
    translations.insert("Delete", "Sil");
    translations.insert("Upload too large", "Yükleme çok büyük");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Yüklemeye çalıştığınız dosyalar bu sunucudaki maksimum yükleme boyutunu aşıyor.");
    translations.insert("Files are being scanned, please wait.", "Dosyalar taranıyor, lütfen bekleyin.");
    translations.insert("Current scanning", "Güncel tarama");
    translations.insert("Upgrading filesystem cache...", "Sistem dosyası önbelleği güncelleniyor");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}

// Registro de traducciones con el sistema i18n
pub fn register_tr_translations() {
    let translations = get_translations();
    for (key, value) in translations {
        rust_i18n::set("tr", key, value);
    }
}