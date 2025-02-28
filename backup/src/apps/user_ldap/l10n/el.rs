use rust_i18n::t;

pub fn register_translations() {
    rust_i18n::set_locale("el");

    // Regular translations
    t!("Failed to delete the server configuration", "Αποτυχία διαγραφής ρυθμίσεων διακομιστή");
    t!("The configuration is valid and the connection could be established!", "Οι ρυθμίσεις είναι έγκυρες και η σύνδεση μπορεί να πραγματοποιηθεί!");
    t!("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Οι ρυθμίσεις είναι έγκυρες, αλλά απέτυχε η σύνδεση. Παρακαλώ ελέγξτε τις ρυθμίσεις του διακομιστή και τα διαπιστευτήρια.");
    t!("Deletion failed", "Η διαγραφή απέτυχε");
    t!("Take over settings from recent server configuration?", "Πάρτε πάνω από τις πρόσφατες ρυθμίσεις διαμόρφωσης του διακομιστή?");
    t!("Keep settings?", "Διατήρηση ρυθμίσεων;");
    t!("Cannot add server configuration", "Αδυναμία προσθήκης ρυθμίσεων διακομιστή");
    t!("Success", "Επιτυχία");
    t!("Error", "Σφάλμα");
    t!("Select groups", "Επιλέξτε ομάδες");
    t!("Connection test succeeded", "Επιτυχημένη δοκιμαστική σύνδεση");
    t!("Connection test failed", "Αποτυχημένη δοκιμαστική σύνδεσης.");
    t!("Do you really want to delete the current Server Configuration?", "Θέλετε να διαγράψετε τις τρέχουσες ρυθμίσεις του διακομιστή;");
    t!("Confirm Deletion", "Επιβεβαίωση Διαγραφής");
    t!("Save", "Αποθήκευση");
    t!("Test Configuration", "Δοκιμαστικες ρυθμισεις");
    t!("Help", "Βοήθεια");
    t!("Add Server Configuration", "Προσθήκη Ρυθμίσεων Διακομιστή");
    t!("Host", "Διακομιστής");
    t!("You can omit the protocol, except you require SSL. Then start with ldaps://", "Μπορείτε να παραλείψετε το πρωτόκολλο, εκτός αν απαιτείται SSL. Σε αυτή την περίπτωση ξεκινήστε με ldaps://");
    t!("Port", "Θύρα");
    t!("User DN", "User DN");
    t!("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "Το DN του χρήστη πελάτη με το οποίο θα πρέπει να γίνει η σύνδεση, π.χ. uid=agent,dc=example,dc=com. Για χρήση χωρίς πιστοποίηση, αφήστε το DN και τον Κωδικό κενά.");
    t!("Password", "Συνθηματικό");
    t!("For anonymous access, leave DN and Password empty.", "Για ανώνυμη πρόσβαση, αφήστε κενά τα πεδία DN και Pasword.");
    t!("One Base DN per line", "Ένα DN Βάσης ανά γραμμή ");
    t!("You can specify Base DN for users and groups in the Advanced tab", "Μπορείτε να καθορίσετε το Base DN για χρήστες και ομάδες από την καρτέλα Προηγμένες ρυθμίσεις");
    t!("Back", "Επιστροφή");
    t!("Continue", "Συνέχεια");
    t!("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Προσοχή:</b> Το άρθρωμα PHP LDAP δεν είναι εγκατεστημένο και το σύστημα υποστήριξης δεν θα δουλέψει. Παρακαλώ ζητήστε από τον διαχειριστή συστήματος να το εγκαταστήσει.");
    t!("Connection Settings", "Ρυθμίσεις Σύνδεσης");
    t!("Configuration Active", "Ενεργοποιηση ρυθμισεων");
    t!("When unchecked, this configuration will be skipped.", "Όταν δεν είναι επιλεγμένο, αυτή η ρύθμιση θα πρέπει να παραλειφθεί. ");
    t!("User Login Filter", "User Login Filter");
    t!("Backup (Replica) Host", "Δημιουργία αντιγράφων ασφαλείας (Replica) Host ");
    t!("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Δώστε μια προαιρετική εφεδρική υποδοχή. Πρέπει να είναι ένα αντίγραφο του κύριου LDAP / AD διακομιστη.");
    t!("Backup (Replica) Port", "Δημιουργία αντιγράφων ασφαλείας (Replica) Υποδοχη");
    t!("Disable Main Server", "Απενεργοποιηση του κεντρικου διακομιστη");
    t!("Case insensitve LDAP server (Windows)", "LDAP server (Windows) με διάκριση πεζών-ΚΕΦΑΛΑΙΩΝ");
    t!("Turn off SSL certificate validation.", "Απενεργοποίηση επικύρωσης πιστοποιητικού SSL.");
    t!("Cache Time-To-Live", "Cache Time-To-Live");
    t!("in seconds. A change empties the cache.", "σε δευτερόλεπτα. Μια αλλαγή αδειάζει την μνήμη cache.");
    t!("Directory Settings", "Ρυθμίσεις Καταλόγου");
    t!("User Display Name Field", "Πεδίο Ονόματος Χρήστη");
    t!("Base User Tree", "Base User Tree");
    t!("One User Base DN per line", "Ένα DN βάσης χρηστών ανά γραμμή");
    t!("User Search Attributes", "Χαρακτηριστικά αναζήτησης των χρηστών ");
    t!("Optional; one attribute per line", "Προαιρετικά? Ένα χαρακτηριστικό ανά γραμμή ");
    t!("Group Display Name Field", "Group Display Name Field");
    t!("Base Group Tree", "Base Group Tree");
    t!("One Group Base DN per line", "Μια ομαδικη Βάση DN ανά γραμμή");
    t!("Group Search Attributes", "Ομάδα Χαρακτηριστικων Αναζήτηση");
    t!("Group-Member association", "Group-Member association");
    t!("Special Attributes", "Ειδικά Χαρακτηριστικά ");
    t!("Quota Field", "Ποσοσταση πεδιου");
    t!("Quota Default", "Προκαθισμενο πεδιο");
    t!("in bytes", "σε bytes");
    t!("Email Field", "Email τυπος");
    t!("User Home Folder Naming Rule", "Χρήστης Προσωπικόςφάκελος Ονομασία Κανόνας ");
    t!("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Αφήστε το κενό για το όνομα χρήστη (προεπιλογή). Διαφορετικά, συμπληρώστε μία ιδιότητα LDAP/AD.");

    // Plural forms
    rust_i18n::set_plural_rule("el", |n| if n != 1 { 1 } else { 0 });
    
    t!("_{s} group found_::_{s} groups found_", |n| match n {
        1 => "", // singular form
        _ => "", // plural form
    });
    
    t!("_{s} user found_::_{s} users found_", |n| match n {
        1 => "", // singular form
        _ => "", // plural form
    });
}