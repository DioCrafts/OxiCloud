use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "Σφάλμα στην φόρτωση της λίστας από το App Store");
        m.insert("Authentication error", "Σφάλμα πιστοποίησης");
        m.insert("Group already exists", "Η ομάδα υπάρχει ήδη");
        m.insert("Unable to add group", "Αδυναμία προσθήκης ομάδας");
        m.insert("Email saved", "Το email αποθηκεύτηκε ");
        m.insert("Invalid email", "Μη έγκυρο email");
        m.insert("Unable to delete group", "Αδυναμία διαγραφής ομάδας");
        m.insert("Unable to delete user", "Αδυναμία διαγραφής χρήστη");
        m.insert("Language changed", "Η γλώσσα άλλαξε");
        m.insert("Invalid request", "Μη έγκυρο αίτημα");
        m.insert("Admins can't remove themself from the admin group", "Οι διαχειριστές δεν μπορούν να αφαιρέσουν τους εαυτούς τους από την ομάδα των διαχειριστών");
        m.insert("Unable to add user to group %s", "Αδυναμία προσθήκη χρήστη στην ομάδα %s");
        m.insert("Unable to remove user from group %s", "Αδυναμία αφαίρεσης χρήστη από την ομάδα %s");
        m.insert("Couldn't update app.", "Αδυναμία ενημέρωσης εφαρμογής");
        m.insert("Wrong password", "Εσφαλμένο συνθηματικό");
        m.insert("Unable to change password", "Αδυναμία αλλαγής συνθηματικού");
        m.insert("Update to {appversion}", "Ενημέρωση σε {appversion}");
        m.insert("Disable", "Απενεργοποίηση");
        m.insert("Enable", "Ενεργοποίηση");
        m.insert("Please wait....", "Παρακαλώ περιμένετε...");
        m.insert("Error while disabling app", "Σφάλμα κατά την απενεργοποίηση εισόδου");
        m.insert("Error while enabling app", "Σφάλμα κατά την ενεργοποίηση της εφαρμογής");
        m.insert("Updating....", "Ενημέρωση...");
        m.insert("Error while updating app", "Σφάλμα κατά την ενημέρωση της εφαρμογής");
        m.insert("Error", "Σφάλμα");
        m.insert("Update", "Ενημέρωση");
        m.insert("Updated", "Ενημερώθηκε");
        m.insert("Select a profile picture", "Επιλογή εικόνας προφίλ");
        m.insert("Saving...", "Γίνεται αποθήκευση...");
        m.insert("deleted", "διαγράφηκε");
        m.insert("undo", "αναίρεση");
        m.insert("Unable to remove user", "Αδυναμία αφαίρεση χρήστη");
        m.insert("Groups", "Ομάδες");
        m.insert("Group Admin", "Ομάδα Διαχειριστών");
        m.insert("Delete", "Διαγραφή");
        m.insert("add group", "προσθήκη ομάδας");
        m.insert("A valid username must be provided", "Πρέπει να δοθεί έγκυρο όνομα χρήστη");
        m.insert("Error creating user", "Σφάλμα δημιουργίας χρήστη");
        m.insert("A valid password must be provided", "Πρέπει να δοθεί έγκυρο συνθηματικό");
        m.insert("__language_name__", "__όνομα_γλώσσας__");
        m.insert("Security Warning", "Προειδοποίηση Ασφαλείας");
        m.insert("Setup Warning", "Ρύθμιση Προειδοποίησης");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Ο διακομιστής σας δεν έχει ρυθμιστεί κατάλληλα ώστε να επιτρέπει τον συγχρονισμό αρχείων γιατί η διεπαφή WebDAV πιθανόν να είναι κατεστραμμένη.");
        m.insert("Please double check the <a href=\"%s\">installation guides</a>.", "Ελέγξτε ξανά τις <a href=\"%s\">οδηγίες εγκατάστασης</a>.");
        m.insert("Module 'fileinfo' missing", "Η ενοτητα 'fileinfo' λειπει");
        m.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "Η PHP ενοτητα 'fileinfo' λειπει. Σας συνιστούμε να ενεργοποιήσετε αυτή την ενότητα για να έχετε καλύτερα αποτελέσματα με τον εντοπισμό τύπου MIME. ");
        m.insert("Locale not working", "Η μετάφραση δεν δουλεύει");
        m.insert("Internet connection not working", "Η σύνδεση στο διαδίκτυο δεν δουλεύει");
        m.insert("Cron", "Cron");
        m.insert("Execute one task with each page loaded", "Εκτέλεση μιας διεργασίας με κάθε σελίδα που φορτώνεται");
        m.insert("Sharing", "Διαμοιρασμός");
        m.insert("Enable Share API", "Ενεργοποίηση API Διαμοιρασμού");
        m.insert("Allow apps to use the Share API", "Να επιτρέπεται στις εφαρμογές να χρησιμοποιούν το API Διαμοιρασμού");
        m.insert("Allow links", "Να επιτρέπονται σύνδεσμοι");
        m.insert("Allow users to share items to the public with links", "Να επιτρέπεται στους χρήστες να διαμοιράζουν δημόσια με συνδέσμους");
        m.insert("Allow resharing", "Να επιτρέπεται ο επαναδιαμοιρασμός");
        m.insert("Allow users to share items shared with them again", "Να επιτρέπεται στους χρήστες να διαμοιράζουν ότι τους έχει διαμοιραστεί");
        m.insert("Allow users to share with anyone", "Να επιτρέπεται ο διαμοιρασμός με οποιονδήποτε");
        m.insert("Allow users to only share with users in their groups", "Να επιτρέπεται στους χρήστες ο διαμοιρασμός μόνο με χρήστες της ίδιας ομάδας");
        m.insert("Security", "Ασφάλεια");
        m.insert("Enforce HTTPS", "Επιβολή χρήσης HTTPS");
        m.insert("Log", "Καταγραφές");
        m.insert("Log level", "Επίπεδο καταγραφής");
        m.insert("More", "Περισσότερα");
        m.insert("Less", "Λιγότερα");
        m.insert("Version", "Έκδοση");
        m.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Αναπτύχθηκε από την <a href=\"http://ownCloud.org/contact\" target=\"_blank\">κοινότητα ownCloud</a>, ο <a href=\"https://github.com/owncloud\" target=\"_blank\">πηγαίος κώδικας</a> είναι υπό άδεια χρήσης <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
        m.insert("Add your App", "Πρόσθεστε τη Δικιά σας Εφαρμογή");
        m.insert("More Apps", "Περισσότερες Εφαρμογές");
        m.insert("Select an App", "Επιλέξτε μια Εφαρμογή");
        m.insert("See application page at apps.owncloud.com", "Δείτε την σελίδα εφαρμογών στο apps.owncloud.com");
        m.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-άδεια από <span class=\"author\"></span>");
        m.insert("User Documentation", "Τεκμηρίωση Χρήστη");
        m.insert("Administrator Documentation", "Τεκμηρίωση Διαχειριστή");
        m.insert("Online Documentation", "Τεκμηρίωση στο Διαδίκτυο");
        m.insert("Forum", "Φόρουμ");
        m.insert("Bugtracker", "Bugtracker");
        m.insert("Commercial Support", "Εμπορική Υποστήριξη");
        m.insert("Get the apps to sync your files", "Λήψη της εφαρμογής για συγχρονισμό των αρχείων σας");
        m.insert("Show First Run Wizard again", "Προβολή Πρώτης Εκτέλεσης Οδηγού πάλι");
        m.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Χρησιμοποιήσατε <strong>%s</strong> από διαθέσιμα <strong>%s</strong>");
        m.insert("Password", "Συνθηματικό");
        m.insert("Your password was changed", "Το συνθηματικό σας έχει αλλάξει");
        m.insert("Unable to change your password", "Δεν ήταν δυνατή η αλλαγή του κωδικού πρόσβασης");
        m.insert("Current password", "Τρέχων συνθηματικό");
        m.insert("New password", "Νέο συνθηματικό");
        m.insert("Change password", "Αλλαγή συνθηματικού");
        m.insert("Email", "Ηλ. ταχυδρομείο");
        m.insert("Your email address", "Η διεύθυνση ηλεκτρονικού ταχυδρομείου σας");
        m.insert("Fill in an email address to enable password recovery", "Συμπληρώστε μια διεύθυνση ηλεκτρονικού ταχυδρομείου για να ενεργοποιηθεί η ανάκτηση συνθηματικού");
        m.insert("Profile picture", "Φωτογραφία προφίλ");
        m.insert("Select new from Files", "Επιλογή νέου από τα Αρχεία");
        m.insert("Remove image", "Αφαίρεση εικόνας");
        m.insert("Abort", "Ματαίωση");
        m.insert("Choose as profile image", "Επιλογή εικόνας προφίλ");
        m.insert("Language", "Γλώσσα");
        m.insert("Help translate", "Βοηθήστε στη μετάφραση");
        m.insert("WebDAV", "WebDAV");
        m.insert("Encryption", "Κρυπτογράφηση");
        m.insert("Log-in password", "Συνθηματικό εισόδου");
        m.insert("Login Name", "Όνομα Σύνδεσης");
        m.insert("Create", "Δημιουργία");
        m.insert("Admin Recovery Password", "Κωδικός Επαναφοράς Διαχειριστή ");
        m.insert("Enter the recovery password in order to recover the users files during password change", "Εισάγετε το συνθηματικό ανάκτησης ώστε να ανακτήσετε τα αρχεία χρηστών κατά την αλλαγή συνθηματικού");
        m.insert("Default Storage", "Προκαθορισμένη Αποθήκευση ");
        m.insert("Unlimited", "Απεριόριστο");
        m.insert("Other", "Άλλο");
        m.insert("Username", "Όνομα χρήστη");
        m.insert("Storage", "Αποθήκευση");
        m.insert("set new password", "επιλογή νέου κωδικού");
        m.insert("Default", "Προκαθορισμένο");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}