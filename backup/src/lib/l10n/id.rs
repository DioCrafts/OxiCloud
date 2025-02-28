use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "Bantuan");
        m.insert("Personal", "Pribadi");
        m.insert("Settings", "Setelan");
        m.insert("Users", "Pengguna");
        m.insert("Admin", "Admin");
        m.insert("web services under your control", "layanan web dalam kontrol Anda");
        m.insert("ZIP download is turned off.", "Pengunduhan ZIP dimatikan.");
        m.insert("Files need to be downloaded one by one.", "Berkas harus diunduh satu persatu.");
        m.insert("Back to Files", "Kembali ke Daftar Berkas");
        m.insert("Selected files too large to generate zip file.", "Berkas yang dipilih terlalu besar untuk dibuat berkas zip-nya.");
        m.insert("Application is not enabled", "Aplikasi tidak diaktifkan");
        m.insert("Authentication error", "Galat saat autentikasi");
        m.insert("Token expired. Please reload page.", "Token kedaluwarsa. Silakan muat ulang halaman.");
        m.insert("Files", "Berkas");
        m.insert("Text", "Teks");
        m.insert("Images", "Gambar");
        m.insert("%s enter the database username.", "%s masukkan nama pengguna basis data.");
        m.insert("%s enter the database name.", "%s masukkan nama basis data.");
        m.insert("%s you may not use dots in the database name", "%sAnda tidak boleh menggunakan karakter titik pada nama basis data");
        m.insert("MS SQL username and/or password not valid: %s", "Nama pengguna dan/atau sandi MySQL tidak valid: %s");
        m.insert("You need to enter either an existing account or the administrator.", "Anda harus memasukkan akun yang sudah ada atau administrator.");
        m.insert("MySQL username and/or password not valid", "Nama pengguna dan/atau sandi MySQL tidak valid");
        m.insert("DB Error: \"%s\"", "Galat Basis Data: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Perintah yang bermasalah: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "Pengguna MySQL '%s'@'localhost' sudah ada.");
        m.insert("Drop this user from MySQL", "Hapus pengguna ini dari MySQL");
        m.insert("MySQL user '%s'@'%%' already exists", "Pengguna MySQL '%s'@'%%' sudah ada.");
        m.insert("Drop this user from MySQL.", "Hapus pengguna ini dari MySQL.");
        m.insert("Oracle username and/or password not valid", "Nama pengguna dan/atau sandi Oracle tidak valid");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Perintah yang bermasalah: \"%s\", nama pengguna: %s, sandi: %s");
        m.insert("PostgreSQL username and/or password not valid", "Nama pengguna dan/atau sandi PostgreSQL tidak valid");
        m.insert("Set an admin username.", "Setel nama pengguna admin.");
        m.insert("Set an admin password.", "Setel sandi admin.");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Web server Anda belum dikonfigurasikan dengan baik untuk mengizinkan sinkronisasi berkas karena tampaknya antarmuka WebDAV rusak.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Silakan periksa ulang <a href='%s'>panduan instalasi</a>.");
        m.insert("Could not find category \"%s\"", "Tidak dapat menemukan kategori \"%s\"");
        m.insert("seconds ago", "beberapa detik yang lalu");
        m.insert("_%n minute ago_::_%n minutes ago_", "");
        m.insert("_%n hour ago_::_%n hours ago_", "");
        m.insert("today", "hari ini");
        m.insert("yesterday", "kemarin");
        m.insert("_%n day go_::_%n days ago_", "");
        m.insert("last month", "bulan kemarin");
        m.insert("_%n month ago_::_%n months ago_", "");
        m.insert("last year", "tahun kemarin");
        m.insert("years ago", "beberapa tahun lalu");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}