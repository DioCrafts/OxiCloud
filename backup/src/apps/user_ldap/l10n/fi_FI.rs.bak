use std::collections::HashMap;
use once_cell::sync::Lazy;

pub type Translations = HashMap<&'static str, &'static str>;
pub type PluralForms = &'static str;

pub struct L10n {
    pub translations: Translations,
    pub plural_forms: PluralForms,
    pub plural_translations: HashMap<&'static str, Vec<&'static str>>,
}

pub static FI_FI: Lazy<L10n> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Deletion failed", "Poisto epäonnistui");
    translations.insert("Keep settings?", "Säilytetäänkö asetukset?");
    translations.insert("Cannot add server configuration", "Palvelinasetusten lisäys epäonnistui");
    translations.insert("Success", "Onnistui!");
    translations.insert("Error", "Virhe");
    translations.insert("Select groups", "Valitse ryhmät");
    translations.insert("Connection test succeeded", "Yhteystesti onnistui");
    translations.insert("Connection test failed", "Yhteystesti epäonnistui");
    translations.insert("Confirm Deletion", "Vahvista poisto");
    translations.insert("Save", "Tallenna");
    translations.insert("Help", "Ohje");
    translations.insert("Host", "Isäntä");
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Voit jättää protokollan määrittämättä, paitsi kun vaadit SSL:ää. Aloita silloin ldaps://");
    translations.insert("Port", "Portti");
    translations.insert("User DN", "Käyttäjän DN");
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "Asiakasohjelman DN, jolla yhdistäminen tehdään, ts. uid=agent,dc=example,dc=com. Mahdollistaaksesi anonyymin yhteyden, jätä DN ja salasana tyhjäksi.");
    translations.insert("Password", "Salasana");
    translations.insert("For anonymous access, leave DN and Password empty.", "Jos haluat mahdollistaa anonyymin pääsyn, jätä DN ja Salasana tyhjäksi ");
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", "Voit määrittää käyttäjien ja ryhmien oletus DN:n (distinguished name) 'tarkemmat asetukset'-välilehdeltä  ");
    translations.insert("Back", "Takaisin");
    translations.insert("Continue", "Jatka");
    translations.insert("Connection Settings", "Yhteysasetukset");
    translations.insert("User Login Filter", "Login suodatus");
    translations.insert("Disable Main Server", "Poista pääpalvelin käytöstä");
    translations.insert("Case insensitve LDAP server (Windows)", "Kirjainkoosta piittamaton LDAP-palvelin (Windows)");
    translations.insert("Turn off SSL certificate validation.", "Poista käytöstä SSL-varmenteen vahvistus");
    translations.insert("in seconds. A change empties the cache.", "sekunneissa. Muutos tyhjentää välimuistin.");
    translations.insert("Directory Settings", "Hakemistoasetukset");
    translations.insert("User Display Name Field", "Käyttäjän näytettävän nimen kenttä");
    translations.insert("Base User Tree", "Oletuskäyttäjäpuu");
    translations.insert("Group Display Name Field", "Ryhmän \"näytettävä nimi\"-kenttä");
    translations.insert("Base Group Tree", "Ryhmien juuri");
    translations.insert("Group-Member association", "Ryhmän ja jäsenen assosiaatio (yhteys)");
    translations.insert("in bytes", "tavuissa");
    translations.insert("Email Field", "Sähköpostikenttä");
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Jätä tyhjäksi käyttäjänimi (oletusasetus). Muutoin anna LDAP/AD-atribuutti.");
    
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%s group found_::_%s groups found_", vec!["", ""]);
    plural_translations.insert("_%s user found_::_%s users found_", vec!["", ""]);
    
    L10n {
        translations,
        plural_forms: "nplurals=2; plural=(n != 1);",
        plural_translations,
    }
});