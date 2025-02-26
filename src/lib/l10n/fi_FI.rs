use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("fi_FI");

fn create_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "Sovellusta \"%s\" ei voi asentaa, koska se ei ole yhteensopiva käytössä olevan ownCloud-version kanssa.");
    translations.insert("No app name specified", "Sovelluksen nimeä ei määritelty");
    translations.insert("Help", "Ohje");
    translations.insert("Personal", "Henkilökohtainen");
    translations.insert("Settings", "Asetukset");
    translations.insert("Users", "Käyttäjät");
    translations.insert("Admin", "Ylläpitäjä");
    translations.insert("Failed to upgrade \"%s\".", "Kohteen \"%s\" päivitys epäonnistui.");
    translations.insert("Unknown filetype", "Tuntematon tiedostotyyppi");
    translations.insert("Invalid image", "Virheellinen kuva");
    translations.insert("web services under your control", "verkkopalvelut hallinnassasi");
    translations.insert("ZIP download is turned off.", "ZIP-lataus on poistettu käytöstä.");
    translations.insert("Files need to be downloaded one by one.", "Tiedostot on ladattava yksittäin.");
    translations.insert("Back to Files", "Takaisin tiedostoihin");
    translations.insert("Selected files too large to generate zip file.", "Valitut tiedostot ovat liian suurikokoisia mahtuakseen zip-tiedostoon.");
    translations.insert("No source specified when installing app", "Lähdettä ei määritelty sovellusta asennettaessa");
    translations.insert("No path specified when installing app from local file", "Polkua ei määritelty sovellusta asennettaessa paikallisesta tiedostosta");
    translations.insert("Archives of type %s are not supported", "Tyypin %s arkistot eivät ole tuettuja");
    translations.insert("App does not provide an info.xml file", "Sovellus ei sisällä info.xml-tiedostoa");
    translations.insert("App can't be installed because of not allowed code in the App", "Sovellusta ei voi asentaa, koska sovellus sisältää kiellettyä koodia");
    translations.insert("App can't be installed because it is not compatible with this version of ownCloud", "Sovellusta ei voi asentaa, koska se ei ole yhteensopiva käytössä olevan ownCloud-version kanssa");
    translations.insert("App directory already exists", "Sovelluskansio on jo olemassa");
    translations.insert("Can't create app folder. Please fix permissions. %s", "Sovelluskansion luominen ei onnistu. Korjaa käyttöoikeudet. %s");
    translations.insert("Application is not enabled", "Sovellusta ei ole otettu käyttöön");
    translations.insert("Authentication error", "Tunnistautumisvirhe");
    translations.insert("Token expired. Please reload page.", "Valtuutus vanheni. Lataa sivu uudelleen.");
    translations.insert("Files", "Tiedostot");
    translations.insert("Text", "Teksti");
    translations.insert("Images", "Kuvat");
    translations.insert("%s enter the database username.", "%s anna tietokannan käyttäjätunnus.");
    translations.insert("%s enter the database name.", "%s anna tietokannan nimi.");
    translations.insert("%s you may not use dots in the database name", "%s et voi käyttää pisteitä tietokannan nimessä");
    translations.insert("MS SQL username and/or password not valid: %s", "MS SQL -käyttäjätunnus ja/tai -salasana on väärin: %s");
    translations.insert("MySQL username and/or password not valid", "MySQL:n käyttäjätunnus ja/tai salasana on väärin");
    translations.insert("DB Error: \"%s\"", "Tietokantavirhe: \"%s\"");
    translations.insert("MySQL user '%s'@'localhost' exists already.", "MySQL-käyttäjä '%s'@'localhost' on jo olemassa.");
    translations.insert("Drop this user from MySQL", "Pudota tämä käyttäjä MySQL:stä");
    translations.insert("MySQL user '%s'@'%%' already exists", "MySQL-käyttäjä '%s'@'%%' on jo olemassa");
    translations.insert("Drop this user from MySQL.", "Pudota tämä käyttäjä MySQL:stä.");
    translations.insert("Oracle connection could not be established", "Oracle-yhteyttä ei voitu muodostaa");
    translations.insert("Oracle username and/or password not valid", "Oraclen käyttäjätunnus ja/tai salasana on väärin");
    translations.insert("PostgreSQL username and/or password not valid", "PostgreSQL:n käyttäjätunnus ja/tai salasana on väärin");
    translations.insert("Set an admin username.", "Aseta ylläpitäjän käyttäjätunnus.");
    translations.insert("Set an admin password.", "Aseta ylläpitäjän salasana.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Lue tarkasti <a href='%s'>asennusohjeet</a>.");
    translations.insert("Could not find category \"%s\"", "Luokkaa \"%s\" ei löytynyt");
    translations.insert("seconds ago", "sekuntia sitten");
    translations.insert("today", "tänään");
    translations.insert("yesterday", "eilen");
    translations.insert("last month", "viime kuussa");
    translations.insert("last year", "viime vuonna");
    translations.insert("years ago", "vuotta sitten");
    translations.insert("Caused by:", "Aiheuttaja:");
    
    translations
}

fn create_plural_forms() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plural_forms = HashMap::new();
    
    plural_forms.insert("_%n minute ago_::_%n minutes ago_", vec!["%n minuutti sitten", "%n minuuttia sitten"]);
    plural_forms.insert("_%n hour ago_::_%n hours ago_", vec!["%n tunti sitten", "%n tuntia sitten"]);
    plural_forms.insert("_%n day go_::_%n days ago_", vec!["%n päivä sitten", "%n päivää sitten"]);
    plural_forms.insert("_%n month ago_::_%n months ago_", vec!["%n kuukausi sitten", "%n kuukautta sitten"]);
    
    plural_forms
}

pub struct FiFI {
    translations: HashMap<&'static str, &'static str>,
    plural_forms: HashMap<&'static str, Vec<&'static str>>,
}

impl FiFI {
    pub fn new() -> Self {
        FiFI {
            translations: create_translations(),
            plural_forms: create_plural_forms(),
        }
    }

    pub fn get_plural_form(&self, n: usize) -> usize {
        if n != 1 { 1 } else { 0 }
    }
    
    pub fn translate(&self, key: &str) -> Option<&str> {
        self.translations.get(key).copied()
    }
    
    pub fn translate_plural(&self, key: &str, count: usize) -> Option<&str> {
        let form = self.get_plural_form(count);
        self.plural_forms.get(key).and_then(|forms| forms.get(form).copied())
    }
}

impl Default for FiFI {
    fn default() -> Self {
        Self::new()
    }
}