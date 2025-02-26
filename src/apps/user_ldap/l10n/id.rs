use rust_i18n::t;

pub fn register_translations() {
    rust_i18n::set_translations(
        "id",
        &[
            ("Failed to delete the server configuration", "Gagal menghapus konfigurasi server"),
            ("The configuration is valid and the connection could be established!", "Konfigurasi valid dan koneksi dapat dilakukan!"),
            ("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Konfigurasi valid, tetapi Bind gagal. Silakan cek pengaturan server dan keamanan."),
            ("Deletion failed", "Penghapusan gagal"),
            ("Take over settings from recent server configuration?", "Ambil alih pengaturan dari konfigurasi server saat ini?"),
            ("Keep settings?", "Biarkan pengaturan?"),
            ("Cannot add server configuration", "Gagal menambah konfigurasi server"),
            ("Success", "Sukses"),
            ("Error", "Galat"),
            ("Select groups", "Pilih grup"),
            ("Connection test succeeded", "Tes koneksi sukses"),
            ("Connection test failed", "Tes koneksi gagal"),
            ("Do you really want to delete the current Server Configuration?", "Anda ingin menghapus Konfigurasi Server saat ini?"),
            ("Confirm Deletion", "Konfirmasi Penghapusan"),
            ("_%s group found_::_%s groups found_", ""),
            ("_%s user found_::_%s users found_", ""),
            ("Save", "Simpan"),
            ("Test Configuration", "Uji Konfigurasi"),
            ("Help", "Bantuan"),
            ("Add Server Configuration", "Tambah Konfigurasi Server"),
            ("Host", "Host"),
            ("You can omit the protocol, except you require SSL. Then start with ldaps://", "Protokol dapat tidak ditulis, kecuali anda menggunakan SSL. Lalu jalankan dengan ldaps://"),
            ("Port", "port"),
            ("User DN", "User DN"),
            ("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN dari klien pengguna yang dengannya tautan akan diterapkan, mis. uid=agen,dc=contoh,dc=com. Untuk akses anonim, biarkan DN dan kata sandi kosong."),
            ("Password", "Sandi"),
            ("For anonymous access, leave DN and Password empty.", "Untuk akses anonim, biarkan DN dan Kata sandi kosong."),
            ("One Base DN per line", "Satu Base DN per baris"),
            ("You can specify Base DN for users and groups in the Advanced tab", "Anda dapat menetapkan Base DN untuk pengguna dan grup dalam tab Lanjutan"),
            ("Back", "Kembali"),
            ("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Peringatan:</b> Modul LDAP PHP tidak terpasang, perangkat tidak akan bekerja. Silakan minta administrator sistem untuk memasangnya."),
            ("Connection Settings", "Pengaturan Koneksi"),
            ("Configuration Active", "Konfigurasi Aktif"),
            ("When unchecked, this configuration will be skipped.", "Jika tidak dicentang, konfigurasi ini dilewati."),
            ("User Login Filter", "gunakan saringan login"),
            ("Backup (Replica) Host", "Host Cadangan (Replika)"),
            ("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Berikan pilihan host cadangan. Harus merupakan replika dari server LDAP/AD utama."),
            ("Backup (Replica) Port", "Port Cadangan (Replika)"),
            ("Disable Main Server", "Nonaktifkan Server Utama"),
            ("Case insensitve LDAP server (Windows)", "Server LDAP dengan kapitalisasi tidak sensitif (Windows)"),
            ("Turn off SSL certificate validation.", "matikan validasi sertivikat SSL"),
            ("Cache Time-To-Live", "Gunakan Tembolok untuk Time-To-Live"),
            ("in seconds. A change empties the cache.", "dalam detik. perubahan mengosongkan cache"),
            ("Directory Settings", "Pengaturan Direktori"),
            ("User Display Name Field", "Bidang Tampilan Nama Pengguna"),
            ("Base User Tree", "Pohon Pengguna Dasar"),
            ("One User Base DN per line", "Satu Pengguna Base DN per baris"),
            ("User Search Attributes", "Atribut Pencarian Pengguna"),
            ("Optional; one attribute per line", "Pilihan; satu atribut per baris"),
            ("Group Display Name Field", "Bidang Tampilan Nama Grup"),
            ("Base Group Tree", "Pohon Grup Dasar"),
            ("One Group Base DN per line", "Satu Grup Base DN per baris"),
            ("Group Search Attributes", "Atribut Pencarian Grup"),
            ("Group-Member association", "asosiasi Anggota-Grup"),
            ("Special Attributes", "Atribut Khusus"),
            ("Quota Field", "Bidang Kuota"),
            ("Quota Default", "Kuota Baku"),
            ("in bytes", "dalam bytes"),
            ("Email Field", "Bidang Email"),
            ("User Home Folder Naming Rule", "Aturan Penamaan Folder Home Pengguna"),
            ("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Biarkan nama pengguna kosong (default). Atau tetapkan atribut LDAP/AD."),
        ],
    );

    // Establecer información de pluralización para indonesio
    rust_i18n::set_plural_rule("id", |n| if *n == 1 { 0 } else { 0 });
}