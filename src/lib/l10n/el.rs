use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "Βοήθεια");
        m.insert("Personal", "Προσωπικά");
        m.insert("Settings", "Ρυθμίσεις");
        m.insert("Users", "Χρήστες");
        m.insert("Admin", "Διαχειριστής");
        m.insert("Failed to upgrade \"%s\".", "Αποτυχία αναβάθμισης του \"%s\".");
        m.insert("Unknown filetype", "Άγνωστος τύπος αρχείου");
        m.insert("Invalid image", "Μη έγκυρη εικόνα");
        m.insert("web services under your control", "υπηρεσίες δικτύου υπό τον έλεγχό σας");
        m.insert("cannot open \"%s\"", "αδυναμία ανοίγματος \"%s\"");
        m.insert("ZIP download is turned off.", "Η λήψη ZIP απενεργοποιήθηκε.");
        m.insert("Files need to be downloaded one by one.", "Τα αρχεία πρέπει να ληφθούν ένα-ένα.");
        m.insert("Back to Files", "Πίσω στα Αρχεία");
        m.insert("Selected files too large to generate zip file.", "Τα επιλεγμένα αρχεία είναι μεγάλα ώστε να δημιουργηθεί αρχείο zip.");
        m.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Λήψη των αρχείων σε μικρότερα κομμάτια, χωριστά ή ρωτήστε τον διαχειριστή σας.");
        m.insert("Application is not enabled", "Δεν ενεργοποιήθηκε η εφαρμογή");
        m.insert("Authentication error", "Σφάλμα πιστοποίησης");
        m.insert("Token expired. Please reload page.", "Το αναγνωριστικό έληξε. Παρακαλώ φορτώστε ξανά την σελίδα.");
        m.insert("Files", "Αρχεία");
        m.insert("Text", "Κείμενο");
        m.insert("Images", "Εικόνες");
        m.insert("%s enter the database username.", "%s εισάγετε το όνομα χρήστη της βάσης δεδομένων.");
        m.insert("%s enter the database name.", "%s εισάγετε το όνομα της βάσης δεδομένων.");
        m.insert("%s you may not use dots in the database name", "%s μάλλον δεν χρησιμοποιείτε τελείες στο όνομα της βάσης δεδομένων");
        m.insert("MS SQL username and/or password not valid: %s", "Το όνομα χρήστη και/ή ο κωδικός της MS SQL δεν είναι έγκυρα: %s");
        m.insert("You need to enter either an existing account or the administrator.", "Χρειάζεται να εισάγετε είτε έναν υπάρχον λογαριασμό ή του διαχειριστή.");
        m.insert("MySQL username and/or password not valid", "Μη έγκυρος χρήστης και/ή συνθηματικό της MySQL");
        m.insert("DB Error: \"%s\"", "Σφάλμα Βάσης Δεδομένων: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Η εντολη παραβατικοτητας ηταν: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "Υπάρχει ήδη ο χρήστης '%s'@'localhost' της MySQL.");
        m.insert("Drop this user from MySQL", "Απόρριψη αυτού του χρήστη από την MySQL");
        m.insert("MySQL user '%s'@'%%' already exists", "Ο χρήστης '%s'@'%%' της MySQL υπάρχει ήδη");
        m.insert("Drop this user from MySQL.", "Απόρριψη αυτού του χρήστη από την MySQL");
        m.insert("Oracle connection could not be established", "Αδυναμία σύνδεσης Oracle");
        m.insert("Oracle username and/or password not valid", "Μη έγκυρος χρήστης και/ή συνθηματικό της Oracle");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Η εντολη παραβατικοτητας ηταν: \"%s\", ονομα: %s, κωδικος: %s");
        m.insert("PostgreSQL username and/or password not valid", "Μη έγκυρος χρήστης και/ή συνθηματικό της PostgreSQL");
        m.insert("Set an admin username.", "Εισάγετε όνομα χρήστη διαχειριστή.");
        m.insert("Set an admin password.", "Εισάγετε συνθηματικό διαχειριστή.");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Ο διακομιστής σας δεν έχει ρυθμιστεί κατάλληλα ώστε να επιτρέπει τον συγχρονισμό αρχείων γιατί η διεπαφή WebDAV πιθανόν να είναι κατεστραμμένη.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Ελέγξτε ξανά τις <a href='%s'>οδηγίες εγκατάστασης</a>.");
        m.insert("Could not find category \"%s\"", "Αδυναμία εύρεσης κατηγορίας \"%s\"");
        m.insert("seconds ago", "δευτερόλεπτα πριν");
        m.insert("today", "σήμερα");
        m.insert("yesterday", "χτες");
        m.insert("last month", "τελευταίο μήνα");
        m.insert("last year", "τελευταίο χρόνο");
        m.insert("years ago", "χρόνια πριν");
        m.insert("Caused by:", "Προκλήθηκε από:");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
        m.insert("_%n day go_::_%n days ago_", vec!["", ""]);
        m.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
        m
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, n: usize) -> Option<&'static str> {
    let plural_index = if n != 1 { 1 } else { 0 };
    PLURAL_TRANSLATIONS.get(key).and_then(|forms| forms.get(plural_index).copied())
}