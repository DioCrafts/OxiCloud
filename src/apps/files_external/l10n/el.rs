use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "Προσβαση παρασχέθηκε");
    m.insert("Error configuring Dropbox storage", "Σφάλμα ρυθμίζωντας αποθήκευση Dropbox ");
    m.insert("Grant access", "Παροχή πρόσβασης");
    m.insert("Please provide a valid Dropbox app key and secret.", "Παρακαλούμε δώστε έγκυρο κλειδί Dropbox και μυστικό.");
    m.insert("Error configuring Google Drive storage", "Σφάλμα ρυθμίζωντας αποθήκευση Google Drive ");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Προσοχή:</b> Ο \"smbclient\" δεν εγκαταστάθηκε. Δεν είναι δυνατή η προσάρτηση CIFS/SMB. Παρακαλώ ενημερώστε τον διαχειριστή συστήματος να το εγκαταστήσει.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Προσοχή:</b> Η υποστήριξη FTP στην PHP δεν ενεργοποιήθηκε ή εγκαταστάθηκε. Δεν είναι δυνατή η προσάρτηση FTP. Παρακαλώ ενημερώστε τον διαχειριστή συστήματος να το εγκαταστήσει.");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<Προειδοποίηση </b> Η υποστήριξη του συστήματος Curl στο PHP δεν είναι ενεργοποιημένη ή εγκαταστημένη. Η αναπαραγωγή του ownCloud/WebDAV ή GoogleDrive δεν είναι δυνατή. Παρακαλώ ρωτήστε τον διαχειριστλη του συστήματος για την εγκατάσταση. ");
    m.insert("External Storage", "Εξωτερικό Αποθηκευτικό Μέσο");
    m.insert("Folder name", "Όνομα φακέλου");
    m.insert("External storage", "Εξωτερική αποθήκευση");
    m.insert("Configuration", "Ρυθμίσεις");
    m.insert("Options", "Επιλογές");
    m.insert("Applicable", "Εφαρμόσιμο");
    m.insert("Add storage", "Προσθηκη αποθηκευσης");
    m.insert("None set", "Κανένα επιλεγμένο");
    m.insert("All Users", "Όλοι οι Χρήστες");
    m.insert("Groups", "Ομάδες");
    m.insert("Users", "Χρήστες");
    m.insert("Delete", "Διαγραφή");
    m.insert("Enable User External Storage", "Ενεργοποίηση Εξωτερικού Αποθηκευτικού Χώρου Χρήστη");
    m.insert("Allow users to mount their own external storage", "Να επιτρέπεται στους χρήστες να προσαρτούν δικό τους εξωτερικό αποθηκευτικό χώρο");
    m.insert("SSL root certificates", "Πιστοποιητικά SSL root");
    m.insert("Import Root Certificate", "Εισαγωγή Πιστοποιητικού Root");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";