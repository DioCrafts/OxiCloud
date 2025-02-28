use rust_i18n::i18n;

i18n!("ms_MY");

pub fn ms_my_translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    translations.insert("No file was uploaded. Unknown error", "Tiada fail dimuatnaik. Ralat tidak diketahui.");
    translations.insert("There is no error, the file uploaded with success", "Tiada ralat berlaku, fail berjaya dimuatnaik");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Saiz fail yang dimuatnaik melebihi MAX_FILE_SIZE yang ditetapkan dalam borang HTML");
    translations.insert("The uploaded file was only partially uploaded", "Fail yang dimuatnaik tidak lengkap");
    translations.insert("No file was uploaded", "Tiada fail dimuatnaik");
    translations.insert("Missing a temporary folder", "Direktori sementara hilang");
    translations.insert("Failed to write to disk", "Gagal untuk disimpan");
    translations.insert("Files", "Fail-fail");
    translations.insert("Upload cancelled.", "Muatnaik dibatalkan.");
    translations.insert("Share", "Kongsi");
    translations.insert("Rename", "Namakan");
    translations.insert("Pending", "Dalam proses");
    translations.insert("_%n folder_::_%n folders_", "");
    translations.insert("_%n file_::_%n files_", "");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "");
    translations.insert("Error", "Ralat");
    translations.insert("Name", "Nama");
    translations.insert("Size", "Saiz");
    translations.insert("Modified", "Dimodifikasi");
    translations.insert("Upload", "Muat naik");
    translations.insert("File handling", "Pengendalian fail");
    translations.insert("Maximum upload size", "Saiz maksimum muat naik");
    translations.insert("max. possible: ", "maksimum:");
    translations.insert("Needed for multi-file and folder downloads.", "Diperlukan untuk muatturun fail pelbagai ");
    translations.insert("Enable ZIP-download", "Aktifkan muatturun ZIP");
    translations.insert("0 is unlimited", "0 adalah tanpa had");
    translations.insert("Maximum input size for ZIP files", "Saiz maksimum input untuk fail ZIP");
    translations.insert("Save", "Simpan");
    translations.insert("New", "Baru");
    translations.insert("Text file", "Fail teks");
    translations.insert("Folder", "Folder");
    translations.insert("Cancel upload", "Batal muat naik");
    translations.insert("Nothing in here. Upload something!", "Tiada apa-apa di sini. Muat naik sesuatu!");
    translations.insert("Download", "Muat turun");
    translations.insert("Delete", "Padam");
    translations.insert("Upload too large", "Muatnaik terlalu besar");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Fail yang cuba dimuat naik melebihi saiz maksimum fail upload server");
    translations.insert("Files are being scanned, please wait.", "Fail sedang diimbas, harap bersabar.");
    translations.insert("Current scanning", "Imbasan semasa");
    
    // Set plural rules
    translations.set_plural_rule("nplurals=1; plural=0;");
    
    translations
}