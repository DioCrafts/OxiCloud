use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Unable to load list from App Store", "Tidak dapat memuat daftar dari App Store");
    map.insert("Authentication error", "Galat saat autentikasi");
    map.insert("Group already exists", "Grup sudah ada");
    map.insert("Unable to add group", "Tidak dapat menambah grup");
    map.insert("Email saved", "Email disimpan");
    map.insert("Invalid email", "Email tidak valid");
    map.insert("Unable to delete group", "Tidak dapat menghapus grup");
    map.insert("Unable to delete user", "Tidak dapat menghapus pengguna");
    map.insert("Language changed", "Bahasa telah diubah");
    map.insert("Invalid request", "Permintaan tidak valid");
    map.insert("Admins can't remove themself from the admin group", "Admin tidak dapat menghapus dirinya sendiri dari grup admin");
    map.insert("Unable to add user to group %s", "Tidak dapat menambahkan pengguna ke grup %s");
    map.insert("Unable to remove user from group %s", "Tidak dapat menghapus pengguna dari grup %s");
    map.insert("Couldn't update app.", "Tidak dapat memperbarui aplikasi.");
    map.insert("Update to {appversion}", "Perbarui ke {appversion}");
    map.insert("Disable", "Nonaktifkan");
    map.insert("Enable", "aktifkan");
    map.insert("Please wait....", "Mohon tunggu....");
    map.insert("Updating....", "Memperbarui....");
    map.insert("Error while updating app", "Gagal ketika memperbarui aplikasi");
    map.insert("Error", "Galat");
    map.insert("Update", "Perbarui");
    map.insert("Updated", "Diperbarui");
    map.insert("Saving...", "Menyimpan...");
    map.insert("deleted", "dihapus");
    map.insert("undo", "urungkan");
    map.insert("Unable to remove user", "Tidak dapat menghapus pengguna");
    map.insert("Groups", "Grup");
    map.insert("Group Admin", "Admin Grup");
    map.insert("Delete", "Hapus");
    map.insert("add group", "tambah grup");
    map.insert("A valid username must be provided", "Tuliskan nama pengguna yang valid");
    map.insert("Error creating user", "Gagal membuat pengguna");
    map.insert("A valid password must be provided", "Tuliskan sandi yang valid");
    map.insert("__language_name__", "__language_name__");
    map.insert("Security Warning", "Peringatan Keamanan");
    map.insert("Setup Warning", "Peringatan Persiapan");
    map.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Web server Anda belum dikonfigurasikan dengan baik untuk mengizinkan sinkronisasi berkas karena tampaknya antarmuka WebDAV rusak.");
    map.insert("Module 'fileinfo' missing", "Module 'fileinfo' tidak ada");
    map.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "Module 'fileinfo' pada PHP tidak ada. Kami sangat menyarankan untuk mengaktifkan modul ini untuk mendapatkan hasil terbaik pada proses pendeteksian mime-type.");
    map.insert("Locale not working", "Kode pelokalan tidak berfungsi");
    map.insert("Internet connection not working", "Koneksi internet tidak berfungsi");
    map.insert("Cron", "Cron");
    map.insert("Execute one task with each page loaded", "Jalankan tugas setiap kali halaman dimuat");
    map.insert("Sharing", "Berbagi");
    map.insert("Enable Share API", "Aktifkan API Pembagian");
    map.insert("Allow apps to use the Share API", "Izinkan aplikasi untuk menggunakan API Pembagian");
    map.insert("Allow links", "Izinkan tautan");
    map.insert("Allow users to share items to the public with links", "Izinkan pengguna untuk berbagi item kepada publik lewat tautan");
    map.insert("Allow resharing", "Izinkan pembagian ulang");
    map.insert("Allow users to share items shared with them again", "Izinkan pengguna untuk berbagi kembali item yang dibagikan kepada mereka.");
    map.insert("Allow users to share with anyone", "Izinkan pengguna untuk berbagi kepada siapa saja");
    map.insert("Allow users to only share with users in their groups", "Hanya izinkan pengguna untuk berbagi dengan pengguna pada grup mereka sendiri");
    map.insert("Security", "Keamanan");
    map.insert("Enforce HTTPS", "Selalu Gunakan HTTPS");
    map.insert("Log", "Catat");
    map.insert("Log level", "Level pencatatan");
    map.insert("More", "Lainnya");
    map.insert("Less", "Ciutkan");
    map.insert("Version", "Versi");
    map.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Dikembangkan oleh <a href=\"http://ownCloud.org/contact\" target=\"_blank\">komunitas ownCloud</a>, <a href=\"https://github.com/owncloud\" target=\"_blank\">kode sumber</a> dilisensikan di bawah <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
    map.insert("Add your App", "Tambahkan Aplikasi Anda");
    map.insert("More Apps", "Aplikasi Lainnya");
    map.insert("Select an App", "Pilih Aplikasi");
    map.insert("See application page at apps.owncloud.com", "Lihat halaman aplikasi di apps.owncloud.com");
    map.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-dilisensikan oleh <span class=\"author\"></span>");
    map.insert("User Documentation", "Dokumentasi Pengguna");
    map.insert("Administrator Documentation", "Dokumentasi Administrator");
    map.insert("Online Documentation", "Dokumentasi Online");
    map.insert("Forum", "Forum");
    map.insert("Bugtracker", "Bugtracker");
    map.insert("Commercial Support", "Dukungan Komersial");
    map.insert("Get the apps to sync your files", "Dapatkan aplikasi untuk sinkronisasi berkas Anda");
    map.insert("Show First Run Wizard again", "Tampilkan Penuntun Konfigurasi Awal");
    map.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Anda telah menggunakan <strong>%s</strong> dari total <strong>%s</strong>");
    map.insert("Password", "Sandi");
    map.insert("Your password was changed", "Sandi Anda telah diubah");
    map.insert("Unable to change your password", "Gagal mengubah sandi Anda");
    map.insert("Current password", "Sandi saat ini");
    map.insert("New password", "Sandi baru");
    map.insert("Change password", "Ubah sandi");
    map.insert("Email", "Email");
    map.insert("Your email address", "Alamat email Anda");
    map.insert("Fill in an email address to enable password recovery", "Masukkan alamat email untuk mengaktifkan pemulihan sandi");
    map.insert("Language", "Bahasa");
    map.insert("Help translate", "Bantu menerjemahkan");
    map.insert("WebDAV", "WebDAV");
    map.insert("Encryption", "Enkripsi");
    map.insert("Login Name", "Nama Masuk");
    map.insert("Create", "Buat");
    map.insert("Default Storage", "Penyimpanan Baku");
    map.insert("Unlimited", "Tak terbatas");
    map.insert("Other", "Lainnya");
    map.insert("Username", "Nama pengguna");
    map.insert("Storage", "Penyimpanan");
    map.insert("set new password", "setel sandi baru");
    map.insert("Default", "Baku");
    map
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";