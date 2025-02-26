//! Localization for Luxembourgish (lb)

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "Konnt Lescht net vum App Store lueden");
        m.insert("Authentication error", "Authentifikatioun's Fehler");
        m.insert("Group already exists", "Group existeiert schon.");
        m.insert("Unable to add group", "Onmeiglech Grupp beizefügen.");
        m.insert("Email saved", "E-mail gespäichert");
        m.insert("Invalid email", "Ongülteg e-mail");
        m.insert("Unable to delete group", "Onmeiglech d'Grup ze läschen.");
        m.insert("Unable to delete user", "Onmeiglech User zu läschen.");
        m.insert("Language changed", "Sprooch huet geännert");
        m.insert("Invalid request", "Ongülteg Requête");
        m.insert("Admins can't remove themself from the admin group", "Admins kennen sech selwer net aus enger Admin Group läschen.");
        m.insert("Unable to add user to group %s", "Onmeiglech User an Grupp ze sätzen %s");
        m.insert("Disable", "Ofschalten");
        m.insert("Enable", "Aschalten");
        m.insert("Error", "Fehler");
        m.insert("Update", "Update");
        m.insert("Saving...", "Speicheren...");
        m.insert("deleted", "geläscht");
        m.insert("undo", "réckgängeg man");
        m.insert("Groups", "Gruppen");
        m.insert("Group Admin", "Gruppen Admin");
        m.insert("Delete", "Läschen");
        m.insert("__language_name__", "__language_name__");
        m.insert("Security Warning", "Sécherheets Warnung");
        m.insert("Cron", "Cron");
        m.insert("Enable Share API", "Share API aschalten");
        m.insert("Allow apps to use the Share API", "Erlab Apps d'Share API ze benotzen");
        m.insert("Allow links", "Links erlaben");
        m.insert("Allow resharing", "Resharing erlaben");
        m.insert("Allow users to share with anyone", "Useren erlaben mat egal wiem ze sharen");
        m.insert("Allow users to only share with users in their groups", "Useren nëmmen erlaben mat Useren aus hirer Grupp ze sharen");
        m.insert("Log", "Log");
        m.insert("More", "Méi");
        m.insert("Less", "Manner");
        m.insert("Add your App", "Setz deng App bei");
        m.insert("Select an App", "Wiel eng Applikatioun aus");
        m.insert("See application page at apps.owncloud.com", "Kuck dir d'Applicatioun's Säit op apps.owncloud.com un");
        m.insert("Password", "Passwuert");
        m.insert("Unable to change your password", "Konnt däin Passwuert net änneren");
        m.insert("Current password", "Momentan 't Passwuert");
        m.insert("New password", "Neit Passwuert");
        m.insert("Change password", "Passwuert änneren");
        m.insert("Email", "Email");
        m.insert("Your email address", "Deng Email Adress");
        m.insert("Fill in an email address to enable password recovery", "Gëff eng Email Adress an fir d'Passwuert recovery ze erlaben");
        m.insert("Language", "Sprooch");
        m.insert("Help translate", "Hëllef iwwersetzen");
        m.insert("Create", "Erstellen");
        m.insert("Other", "Aner");
        m.insert("Username", "Benotzernumm");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

/// Gets a translation for the given key
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

/// Gets the plural form rule
pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}