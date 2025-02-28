use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The password is wrong. Try again.", "Εσφαλμένο συνθηματικό. Προσπαθήστε ξανά.");
        m.insert("Password", "Συνθηματικό");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Συγγνώμη, αυτός ο σύνδεσμος μοιάζει να μην ισχύει πια.");
        m.insert("Reasons might be:", "Οι λόγοι μπορεί να είναι:");
        m.insert("the item was removed", "το αντικείμενο απομακρύνθηκε");
        m.insert("the link expired", "ο σύνδεσμος έληξε");
        m.insert("sharing is disabled", "ο διαμοιρασμός απενεργοποιήθηκε");
        m.insert("For more info, please ask the person who sent this link.", "Για περισσότερες πληροφορίες, παρακαλώ ρωτήστε το άτομο που σας έστειλε αυτόν τον σύνδεσμο.");
        m.insert("%s shared the folder %s with you", "%s μοιράστηκε τον φάκελο %s μαζί σας");
        m.insert("%s shared the file %s with you", "%s μοιράστηκε το αρχείο %s μαζί σας");
        m.insert("Download", "Λήψη");
        m.insert("Upload", "Μεταφόρτωση");
        m.insert("Cancel upload", "Ακύρωση αποστολής");
        m.insert("No preview available for", "Δεν υπάρχει διαθέσιμη προεπισκόπηση για");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}