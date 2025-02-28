use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Αδυναμία μετακίνησης του %s - υπάρχει ήδη αρχείο με αυτό το όνομα");
        m.insert("Could not move %s", "Αδυναμία μετακίνησης του %s");
        m.insert("File name cannot be empty.", "Το όνομα αρχείου δεν μπορεί να είναι κενό.");
        m.insert("Unable to set upload directory.", "Αδυναμία ορισμού καταλόγου αποστολής.");
        m.insert("Invalid Token", "Μη έγκυρο Token");
        m.insert("No file was uploaded. Unknown error", "Δεν ανέβηκε κάποιο αρχείο. Άγνωστο σφάλμα");
        m.insert("There is no error, the file uploaded with success", "Δεν υπάρχει σφάλμα, το αρχείο εστάλει επιτυχώς");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Το αρχείο που εστάλει υπερβαίνει την οδηγία μέγιστου επιτρεπτού μεγέθους \"upload_max_filesize\" του php.ini");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Το ανεβασμένο αρχείο υπερβαίνει το MAX_FILE_SIZE που ορίζεται στην  HTML φόρμα");
        m.insert("The uploaded file was only partially uploaded", "Το αρχείο εστάλει μόνο εν μέρει");
        m.insert("No file was uploaded", "Κανένα αρχείο δεν στάλθηκε");
        m.insert("Missing a temporary folder", "Λείπει ο προσωρινός φάκελος");
        m.insert("Failed to write to disk", "Αποτυχία εγγραφής στο δίσκο");
        m.insert("Not enough storage available", "Μη επαρκής διαθέσιμος αποθηκευτικός χώρος");
        m.insert("Invalid directory.", "Μη έγκυρος φάκελος.");
        m.insert("Files", "Αρχεία");
        m.insert("Not enough space available", "Δεν υπάρχει αρκετός διαθέσιμος χώρος");
        m.insert("Upload cancelled.", "Η αποστολή ακυρώθηκε.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Η αποστολή του αρχείου βρίσκεται σε εξέλιξη. Το κλείσιμο της σελίδας θα ακυρώσει την αποστολή.");
        m.insert("{new_name} already exists", "{new_name} υπάρχει ήδη");
        m.insert("Share", "Διαμοιρασμός");
        m.insert("Delete permanently", "Μόνιμη διαγραφή");
        m.insert("Rename", "Μετονομασία");
        m.insert("Pending", "Εκκρεμεί");
        m.insert("replaced {new_name} with {old_name}", "αντικαταστάθηκε το {new_name} με {old_name}");
        m.insert("undo", "αναίρεση");
        m.insert("'.' is an invalid file name.", "'.' είναι μη έγκυρο όνομα αρχείου.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Μη έγκυρο όνομα, '\\', '/', '<', '>', ':', '\"', '|', '?' και '*' δεν επιτρέπονται.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Ο αποθηκευτικός σας χώρος είναι γεμάτος, τα αρχεία δεν μπορούν να ενημερωθούν ή να συγχρονιστούν πια!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Ο αποθηκευτικός χώρος είναι σχεδόν γεμάτος ({usedSpacePercent}%)");
        m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Η κρυπτογράφηση απενεργοποιήθηκε, αλλά τα αρχεία σας είναι ακόμα κρυπτογραφημένα. Παρακαλούμε απενεργοποιήσετε την κρυπτογράφηση αρχείων από τις προσωπικές σας ρυθμίσεις");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Η λήψη προετοιμάζεται. Αυτό μπορεί να πάρει ώρα εάν τα αρχεία έχουν μεγάλο μέγεθος.");
        m.insert("Error moving file", "Σφάλμα κατά τη μετακίνηση του αρχείου");
        m.insert("Error", "Σφάλμα");
        m.insert("Name", "Όνομα");
        m.insert("Size", "Μέγεθος");
        m.insert("Modified", "Τροποποιήθηκε");
        m.insert("%s could not be renamed", "Αδυναμία μετονομασίας του %s");
        m.insert("Upload", "Μεταφόρτωση");
        m.insert("File handling", "Διαχείριση αρχείων");
        m.insert("Maximum upload size", "Μέγιστο μέγεθος αποστολής");
        m.insert("max. possible: ", "μέγιστο δυνατό:");
        m.insert("Needed for multi-file and folder downloads.", "Απαραίτητο για κατέβασμα πολλαπλών αρχείων και φακέλων");
        m.insert("Enable ZIP-download", "Ενεργοποίηση κατεβάσματος ZIP");
        m.insert("0 is unlimited", "0 για απεριόριστο");
        m.insert("Maximum input size for ZIP files", "Μέγιστο μέγεθος για αρχεία ZIP");
        m.insert("Save", "Αποθήκευση");
        m.insert("New", "Νέο");
        m.insert("Text file", "Αρχείο κειμένου");
        m.insert("Folder", "Φάκελος");
        m.insert("From link", "Από σύνδεσμο");
        m.insert("Deleted files", "Διαγραμμένα αρχεία");
        m.insert("Cancel upload", "Ακύρωση αποστολής");
        m.insert("Nothing in here. Upload something!", "Δεν υπάρχει τίποτα εδώ. Ανεβάστε κάτι!");
        m.insert("Download", "Λήψη");
        m.insert("Unshare", "Σταμάτημα διαμοιρασμού");
        m.insert("Delete", "Διαγραφή");
        m.insert("Upload too large", "Πολύ μεγάλο αρχείο προς αποστολή");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Τα αρχεία που προσπαθείτε να ανεβάσετε υπερβαίνουν το μέγιστο μέγεθος αποστολής αρχείων σε αυτόν τον διακομιστή.");
        m.insert("Files are being scanned, please wait.", "Τα αρχεία σαρώνονται, παρακαλώ περιμένετε.");
        m.insert("Current scanning", "Τρέχουσα ανίχνευση");
        m.insert("Upgrading filesystem cache...", "Ενημέρωση της μνήμης cache του συστήματος αρχείων...");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec!["%n φάκελος", "%n φάκελοι"]);
        m.insert("_%n file_::_%n files_", vec!["%n αρχείο", "%n αρχεία"]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec!["Ανέβασμα %n αρχείου", "Ανέβασμα %n αρχείων"]);
        m
    };
}

pub fn get_plural_form(n: i64) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn translate(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn translate_plural(key: &str, count: i64) -> Option<&'static str> {
    PLURAL_FORMS.get(key).and_then(|forms| {
        let index = get_plural_form(count);
        forms.get(index).copied()
    })
}