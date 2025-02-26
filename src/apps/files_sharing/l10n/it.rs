use rust_i18n::i18n;

i18n!("it");

pub fn init_translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    translations.insert("This share is password-protected".to_string(), "Questa condivione è protetta da password".to_string());
    translations.insert("The password is wrong. Try again.".to_string(), "La password è errata. Prova ancora.".to_string());
    translations.insert("Password".to_string(), "Password".to_string());
    translations.insert("Sorry, this link doesn't seem to work anymore.".to_string(), "Spiacenti, questo collegamento sembra non essere più attivo.".to_string());
    translations.insert("Reasons might be:".to_string(), "I motivi potrebbero essere:".to_string());
    translations.insert("the item was removed".to_string(), "l'elemento è stato rimosso".to_string());
    translations.insert("the link expired".to_string(), "il collegamento è scaduto".to_string());
    translations.insert("sharing is disabled".to_string(), "la condivisione è disabilitata".to_string());
    translations.insert("For more info, please ask the person who sent this link.".to_string(), "Per ulteriori informazioni, chiedi alla persona che ti ha inviato il collegamento.".to_string());
    translations.insert("%s shared the folder %s with you".to_string(), "%s ha condiviso la cartella %s con te".to_string());
    translations.insert("%s shared the file %s with you".to_string(), "%s ha condiviso il file %s con te".to_string());
    translations.insert("Download".to_string(), "Scarica".to_string());
    translations.insert("Upload".to_string(), "Carica".to_string());
    translations.insert("Cancel upload".to_string(), "Annulla il caricamento".to_string());
    translations.insert("No preview available for".to_string(), "Nessuna anteprima disponibile per".to_string());
    translations.insert("Direct link".to_string(), "Collegamento diretto".to_string());
    
    translations.set_plural_form("nplurals=2; plural=(n != 1);".to_string());
    
    translations
}