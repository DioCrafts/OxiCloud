use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Acces permis");
        m.insert("Error configuring Dropbox storage", "Eroare la configurarea mediului de stocare Dropbox");
        m.insert("Grant access", "Permite accesul");
        m.insert("Please provide a valid Dropbox app key and secret.", "Prezintă te rog o cheie de Dropbox validă și parola");
        m.insert("Error configuring Google Drive storage", "Eroare la configurarea mediului de stocare Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Atenție:</b> \"smbclient\" nu este instalat. Montarea mediilor CIFS/SMB partajate nu este posibilă. Solicită administratorului sistemului tău să îl instaleaze.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Atenție:</b> suportul pentru FTP în PHP nu este activat sau instalat. Montarea mediilor FPT partajate nu este posibilă. Solicită administratorului sistemului tău să îl instaleze.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Atentie:</b> Suportul Curl nu este pornit / instalat in configuratia PHP! Montarea ownCloud / WebDAV / GoogleDrive nu este posibila! Intrebati administratorul sistemului despre aceasta problema!");
        m.insert("External Storage", "Stocare externă");
        m.insert("Folder name", "Denumire director");
        m.insert("External storage", "Stocare externă");
        m.insert("Configuration", "Configurație");
        m.insert("Options", "Opțiuni");
        m.insert("Applicable", "Aplicabil");
        m.insert("Add storage", "Adauga stocare");
        m.insert("None set", "Niciunul");
        m.insert("All Users", "Toți utilizatorii");
        m.insert("Groups", "Grupuri");
        m.insert("Users", "Utilizatori");
        m.insert("Delete", "Șterge");
        m.insert("Enable User External Storage", "Permite stocare externă pentru utilizatori");
        m.insert("Allow users to mount their own external storage", "Permite utilizatorilor să monteze stocare externă proprie");
        m.insert("SSL root certificates", "Certificate SSL root");
        m.insert("Import Root Certificate", "Importă certificat root");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1?0:(((n%100>19)||((n%100==0)&&(n!=0)))?2:1));";
}