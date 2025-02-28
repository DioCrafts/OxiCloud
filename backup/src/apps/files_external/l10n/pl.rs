use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Dostęp do");
        m.insert("Error configuring Dropbox storage", "Wystąpił błąd podczas konfigurowania zasobu Dropbox");
        m.insert("Grant access", "Udziel dostępu");
        m.insert("Please provide a valid Dropbox app key and secret.", "Proszę podać prawidłowy klucz aplikacji Dropbox i klucz sekretny.");
        m.insert("Error configuring Google Drive storage", "Wystąpił błąd podczas konfigurowania zasobu Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Ostrzeżenie:</b> \"smbclient\" nie jest zainstalowany. Zamontowanie katalogów CIFS/SMB nie jest możliwe. Skontaktuj sie z administratorem w celu zainstalowania.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Ostrzeżenie:</b> Wsparcie dla FTP w PHP nie jest zainstalowane lub włączone.  Skontaktuj sie z administratorem w celu zainstalowania lub włączenia go.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Ostrzeżenie:</b> Wsparcie dla  Curl w PHP nie jest zainstalowane lub włączone. Montowanie WebDAV lub GoogleDrive nie będzie możliwe. Skontaktuj się z administratorem w celu zainstalowania lub włączenia tej opcji.");
        m.insert("External Storage", "Zewnętrzna zasoby dyskowe");
        m.insert("Folder name", "Nazwa folderu");
        m.insert("External storage", "Zewnętrzne zasoby dyskowe");
        m.insert("Configuration", "Konfiguracja");
        m.insert("Options", "Opcje");
        m.insert("Applicable", "Zastosowanie");
        m.insert("Add storage", "Dodaj zasoby dyskowe");
        m.insert("None set", "Nie ustawione");
        m.insert("All Users", "Wszyscy uzytkownicy");
        m.insert("Groups", "Grupy");
        m.insert("Users", "Użytkownicy");
        m.insert("Delete", "Usuń");
        m.insert("Enable User External Storage", "Włącz zewnętrzne zasoby dyskowe użytkownika");
        m.insert("Allow users to mount their own external storage", "Zezwalaj użytkownikom na montowanie  ich własnych zewnętrznych zasobów dyskowych");
        m.insert("SSL root certificates", "Główny certyfikat SSL");
        m.insert("Import Root Certificate", "Importuj główny certyfikat");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}