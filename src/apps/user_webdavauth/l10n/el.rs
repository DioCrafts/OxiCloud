use once_cell::sync::Lazy;
use std::collections::HashMap;

// Greek translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert(
        "WebDAV Authentication",
        "Αυθεντικοποίηση μέσω WebDAV ",
    );
    translations.insert("Address: ", "Διεύθυνση:");
    translations.insert(
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
        "Τα διαπιστευτήρια του χρήστη θα σταλούν σε αυτή την διεύθυνση. Αυτό το πρόσθετο ελέγχει την απόκριση και θα ερμηνεύσει τους κωδικούς κατάστασης HTTP 401 και 402 ως μη έγκυρα διαπιστευτήρια και όλες τις άλλες αποκρίσεις ως έγκυρα διαπιστευτήρια.",
    );
    translations
});

pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn get_translation(text: &str) -> &'static str {
    TRANSLATIONS.get(text).copied().unwrap_or(text)
}