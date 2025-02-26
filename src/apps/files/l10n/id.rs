use std::collections::HashMap;
use rust_gettext::Catalog;

// Definimos la estructura para las traducciones
pub struct IdTranslations {
    catalog: Catalog,
}

impl IdTranslations {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("Could not move %s - File with this name already exists".to_string(), "Tidak dapat memindahkan %s - Berkas dengan nama ini sudah ada".to_string());
        translations.insert("Could not move %s".to_string(), "Tidak dapat memindahkan %s".to_string());
        translations.insert("File name cannot be empty.".to_string(), "Nama berkas tidak boleh kosong.".to_string());
        translations.insert("No file was uploaded. Unknown error".to_string(), "Tidak ada berkas yang diunggah. Galat tidak dikenal.".to_string());
        translations.insert("There is no error, the file uploaded with success".to_string(), "Tidak ada galat, berkas sukses diunggah".to_string());
        translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(), "Berkas yang diunggah melampaui direktif upload_max_filesize pada php.ini".to_string());
        translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(), "Berkas yang diunggah melampaui direktif MAX_FILE_SIZE yang ditentukan dalam formulir HTML.".to_string());
        translations.insert("The uploaded file was only partially uploaded".to_string(), "Berkas hanya diunggah sebagian".to_string());
        translations.insert("No file was uploaded".to_string(), "Tidak ada berkas yang diunggah".to_string());
        translations.insert("Missing a temporary folder".to_string(), "Folder sementara tidak ada".to_string());
        translations.insert("Failed to write to disk".to_string(), "Gagal menulis ke disk".to_string());
        translations.insert("Not enough storage available".to_string(), "Ruang penyimpanan tidak mencukupi".to_string());
        translations.insert("Invalid directory.".to_string(), "Direktori tidak valid.".to_string());
        translations.insert("Files".to_string(), "Berkas".to_string());
        translations.insert("Not enough space available".to_string(), "Ruang penyimpanan tidak mencukupi".to_string());
        translations.insert("Upload cancelled.".to_string(), "Pengunggahan dibatalkan.".to_string());
        translations.insert("File upload is in progress. Leaving the page now will cancel the upload.".to_string(), "Berkas sedang diunggah. Meninggalkan halaman ini akan membatalkan proses.".to_string());
        translations.insert("{new_name} already exists".to_string(), "{new_name} sudah ada".to_string());
        translations.insert("Share".to_string(), "Bagikan".to_string());
        translations.insert("Delete permanently".to_string(), "Hapus secara permanen".to_string());
        translations.insert("Rename".to_string(), "Ubah nama".to_string());
        translations.insert("Pending".to_string(), "Menunggu".to_string());
        translations.insert("replaced {new_name} with {old_name}".to_string(), "mengganti {new_name} dengan {old_name}".to_string());
        translations.insert("undo".to_string(), "urungkan".to_string());
        translations.insert("_%n folder_::_%n folders_".to_string(), "".to_string());
        translations.insert("_%n file_::_%n files_".to_string(), "".to_string());
        translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), "".to_string());
        translations.insert("'.' is an invalid file name.".to_string(), "'.' bukan nama berkas yang valid.".to_string());
        translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(), "Nama tidak valid, karakter '\\', '/', '<', '>', ':', '\"', '|', '?' dan '*' tidak diizinkan.".to_string());
        translations.insert("Your storage is full, files can not be updated or synced anymore!".to_string(), "Ruang penyimpanan Anda penuh, berkas tidak dapat diperbarui atau disinkronkan lagi!".to_string());
        translations.insert("Your storage is almost full ({usedSpacePercent}%)".to_string(), "Ruang penyimpanan hampir penuh ({usedSpacePercent}%)".to_string());
        translations.insert("Your download is being prepared. This might take some time if the files are big.".to_string(), "Unduhan Anda sedang disiapkan. Prosesnya dapat berlangsung agak lama jika ukuran berkasnya besar.".to_string());
        translations.insert("Error".to_string(), "Galat".to_string());
        translations.insert("Name".to_string(), "Nama".to_string());
        translations.insert("Size".to_string(), "Ukuran".to_string());
        translations.insert("Modified".to_string(), "Dimodifikasi".to_string());
        translations.insert("Upload".to_string(), "Unggah".to_string());
        translations.insert("File handling".to_string(), "Penanganan berkas".to_string());
        translations.insert("Maximum upload size".to_string(), "Ukuran pengunggahan maksimum".to_string());
        translations.insert("max. possible: ".to_string(), "Kemungkinan maks.:".to_string());
        translations.insert("Needed for multi-file and folder downloads.".to_string(), "Dibutuhkan untuk pengunduhan multi-berkas dan multi-folder".to_string());
        translations.insert("Enable ZIP-download".to_string(), "Aktifkan unduhan ZIP".to_string());
        translations.insert("0 is unlimited".to_string(), "0 berarti tidak terbatas".to_string());
        translations.insert("Maximum input size for ZIP files".to_string(), "Ukuran masukan maksimum untuk berkas ZIP".to_string());
        translations.insert("Save".to_string(), "Simpan".to_string());
        translations.insert("New".to_string(), "Baru".to_string());
        translations.insert("Text file".to_string(), "Berkas teks".to_string());
        translations.insert("Folder".to_string(), "Folder".to_string());
        translations.insert("From link".to_string(), "Dari tautan".to_string());
        translations.insert("Deleted files".to_string(), "Berkas yang dihapus".to_string());
        translations.insert("Cancel upload".to_string(), "Batal pengunggahan".to_string());
        translations.insert("Nothing in here. Upload something!".to_string(), "Tidak ada apa-apa di sini. Unggah sesuatu!".to_string());
        translations.insert("Download".to_string(), "Unduh".to_string());
        translations.insert("Unshare".to_string(), "Batalkan berbagi".to_string());
        translations.insert("Delete".to_string(), "Hapus".to_string());
        translations.insert("Upload too large".to_string(), "Yang diunggah terlalu besar".to_string());
        translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(), "Berkas yang dicoba untuk diunggah melebihi ukuran maksimum pengunggahan berkas di server ini.".to_string());
        translations.insert("Files are being scanned, please wait.".to_string(), "Berkas sedang dipindai, silakan tunggu.".to_string());
        translations.insert("Current scanning".to_string(), "Yang sedang dipindai".to_string());
        translations.insert("Upgrading filesystem cache...".to_string(), "Meningkatkan tembolok sistem berkas...".to_string());

        let catalog = Catalog {
            messages: translations,
            plural_form: "nplurals=1; plural=0;".to_string(),
            ..Default::default()
        };

        Self { catalog }
    }
    
    pub fn get_catalog(&self) -> &Catalog {
        &self.catalog
    }
}

impl Default for IdTranslations {
    fn default() -> Self {
        Self::new()
    }
}