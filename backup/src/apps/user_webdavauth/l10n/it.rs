use rust_i18n::t;

pub fn register_translations() {
    rust_i18n::set_locale("it");

    rust_i18n::define_translation!("it", {
        "WebDAV Authentication": "Autenticazione WebDAV",
        "Address: ": "Indirizzo:",
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.": "Le credenziali dell'utente saranno inviate a questo indirizzo. Questa estensione controlla la risposta e interpreterà i codici di stato HTTP 401 e 403 come credenziali non valide, e tutte le altre risposte come credenziali valide."
    });

    rust_i18n::set_plural_rule("it", |n| if n != 1 { 1 } else { 0 });
}