// L10n translation definitions for French

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("This share is password-protected", "Ce partage est protégé par un mot de passe");
        m.insert("The password is wrong. Try again.", "Le mot de passe est incorrect. Veuillez réessayer.");
        m.insert("Password", "Mot de passe");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Désolé, mais le lien semble ne plus fonctionner.");
        m.insert("Reasons might be:", "Les raisons peuvent être :");
        m.insert("the item was removed", "l'item a été supprimé");
        m.insert("the link expired", "le lien a expiré");
        m.insert("sharing is disabled", "le partage est désactivé");
        m.insert("For more info, please ask the person who sent this link.", "Pour plus d'informations, veuillez contacter la personne qui a envoyé ce lien.");
        m.insert("%s shared the folder %s with you", "%s a partagé le répertoire %s avec vous");
        m.insert("%s shared the file %s with you", "%s a partagé le fichier %s avec vous");
        m.insert("Download", "Télécharger");
        m.insert("Upload", "Envoyer");
        m.insert("Cancel upload", "Annuler l'envoi");
        m.insert("No preview available for", "Pas d'aperçu disponible pour");
        m.insert("Direct link", "Lien direct");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}