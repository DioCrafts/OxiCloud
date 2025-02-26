use std::collections::HashMap;
use rust_i18n::i18n;

/// Estonian translations for ownCloud
pub fn et_ee_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "App \"%s\" can't be installed because it is not compatible with this version of ownCloud.".to_string(),
        "Rakendit \"%s\" ei saa paigaldada, kuna see pole ühilduv selle ownCloud versiooniga.".to_string(),
    );
    translations.insert(
        "No app name specified".to_string(),
        "Ühegi rakendi nime pole määratletud".to_string(),
    );
    translations.insert("Help".to_string(), "Abiinfo".to_string());
    translations.insert("Personal".to_string(), "Isiklik".to_string());
    translations.insert("Settings".to_string(), "Seaded".to_string());
    translations.insert("Users".to_string(), "Kasutajad".to_string());
    translations.insert("Admin".to_string(), "Admin".to_string());
    translations.insert(
        "Failed to upgrade \"%s\".".to_string(),
        "Ebaõnnestunud uuendus \"%s\".".to_string(),
    );
    translations.insert(
        "Unknown filetype".to_string(),
        "Tundmatu failitüüp".to_string(),
    );
    translations.insert("Invalid image".to_string(), "Vigane pilt".to_string());
    translations.insert(
        "web services under your control".to_string(),
        "veebitenused sinu kontrolli all".to_string(),
    );
    translations.insert(
        "cannot open \"%s\"".to_string(),
        "ei suuda avada \"%s\"".to_string(),
    );
    translations.insert(
        "ZIP download is turned off.".to_string(),
        "ZIP-ina allalaadimine on välja lülitatud.".to_string(),
    );
    translations.insert(
        "Files need to be downloaded one by one.".to_string(),
        "Failid tuleb alla laadida ükshaaval.".to_string(),
    );
    translations.insert(
        "Back to Files".to_string(),
        "Tagasi failide juurde".to_string(),
    );
    translations.insert(
        "Selected files too large to generate zip file.".to_string(),
        "Valitud failid on ZIP-faili loomiseks liiga suured.".to_string(),
    );
    translations.insert(
        "Download the files in smaller chunks, seperately or kindly ask your administrator.".to_string(),
        "Laadi failid alla eraldi väiksemate osadena või küsi nõu oma süsteemiadminstraatorilt.".to_string(),
    );
    translations.insert(
        "No source specified when installing app".to_string(),
        "Ühegi lähteallikat pole rakendi paigalduseks määratletud".to_string(),
    );
    translations.insert(
        "No href specified when installing app from http".to_string(),
        "Ühtegi aadressi pole määratletud rakendi paigalduseks veebist".to_string(),
    );
    translations.insert(
        "No path specified when installing app from local file".to_string(),
        "Ühtegi teed pole määratletud paigaldamaks rakendit kohalikust failist".to_string(),
    );
    translations.insert(
        "Archives of type %s are not supported".to_string(),
        "%s tüüpi arhiivid pole toetatud".to_string(),
    );
    translations.insert(
        "Failed to open archive when installing app".to_string(),
        "Arhiivi avamine ebaõnnestus rakendi paigalduse käigus".to_string(),
    );
    translations.insert(
        "App does not provide an info.xml file".to_string(),
        "Rakend ei paku ühtegi info.xml faili".to_string(),
    );
    translations.insert(
        "App can't be installed because of not allowed code in the App".to_string(),
        "Rakendit ei saa paigaldada, kuna sisaldab lubamatud koodi".to_string(),
    );
    translations.insert(
        "App can't be installed because it is not compatible with this version of ownCloud".to_string(),
        "Rakendit ei saa paigaldada, kuna see pole ühilduv selle ownCloud versiooniga.".to_string(),
    );
    translations.insert(
        "App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps".to_string(),
        "Rakendit ei saa paigaldada, kuna see sisaldab \n<shipped>\n\ntrue\n</shipped>\nmärgendit, mis pole lubatud mitte veetud (non shipped) rakendites".to_string(),
    );
    translations.insert(
        "App can't be installed because the version in info.xml/version is not the same as the version reported from the app store".to_string(),
        "Rakendit ei saa paigaldada, kuna selle versioon info.xml/version pole sama, mis on märgitud rakendite laos.".to_string(),
    );
    translations.insert(
        "App directory already exists".to_string(),
        "Rakendi kataloog on juba olemas".to_string(),
    );
    translations.insert(
        "Can't create app folder. Please fix permissions. %s".to_string(),
        "Ei saa luua rakendi kataloogi. Palun korrigeeri õigusi. %s".to_string(),
    );
    translations.insert(
        "Application is not enabled".to_string(),
        "Rakendus pole sisse lülitatud".to_string(),
    );
    translations.insert(
        "Authentication error".to_string(),
        "Autentimise viga".to_string(),
    );
    translations.insert(
        "Token expired. Please reload page.".to_string(),
        "Kontrollkood aegus. Paelun lae leht uuesti.".to_string(),
    );
    translations.insert("Files".to_string(), "Failid".to_string());
    translations.insert("Text".to_string(), "Tekst".to_string());
    translations.insert("Images".to_string(), "Pildid".to_string());
    translations.insert(
        "%s enter the database username.".to_string(),
        "%s sisesta andmebaasi kasutajatunnus.".to_string(),
    );
    translations.insert(
        "%s enter the database name.".to_string(),
        "%s sisesta andmebaasi nimi.".to_string(),
    );
    translations.insert(
        "%s you may not use dots in the database name".to_string(),
        "%s punktide kasutamine andmebaasi nimes pole lubatud".to_string(),
    );
    translations.insert(
        "MS SQL username and/or password not valid: %s".to_string(),
        "MS SQL kasutajatunnus ja/või parool pole õiged: %s".to_string(),
    );
    translations.insert(
        "You need to enter either an existing account or the administrator.".to_string(),
        "Sisesta kas juba olemasolev konto või administrator.".to_string(),
    );
    translations.insert(
        "MySQL username and/or password not valid".to_string(),
        "MySQL kasutajatunnus ja/või parool pole õiged".to_string(),
    );
    translations.insert(
        "DB Error: \"%s\"".to_string(),
        "Andmebaasi viga: \"%s\"".to_string(),
    );
    translations.insert(
        "Offending command was: \"%s\"".to_string(),
        "Tõrkuv käsk oli: \"%s\"".to_string(),
    );
    translations.insert(
        "MySQL user '%s'@'localhost' exists already.".to_string(),
        "MySQL kasutaja '%s'@'localhost' on juba olemas.".to_string(),
    );
    translations.insert(
        "Drop this user from MySQL".to_string(),
        "Kustuta see kasutaja MySQL-ist".to_string(),
    );
    translations.insert(
        "MySQL user '%s'@'%%' already exists".to_string(),
        "MySQL kasutaja '%s'@'%%' on juba olemas".to_string(),
    );
    translations.insert(
        "Drop this user from MySQL.".to_string(),
        "Kustuta see kasutaja MySQL-ist.".to_string(),
    );
    translations.insert(
        "Oracle connection could not be established".to_string(),
        "Ei suuda luua ühendust Oracle baasiga".to_string(),
    );
    translations.insert(
        "Oracle username and/or password not valid".to_string(),
        "Oracle kasutajatunnus ja/või parool pole õiged".to_string(),
    );
    translations.insert(
        "Offending command was: \"%s\", name: %s, password: %s".to_string(),
        "Tõrkuv käsk oli: \"%s\", nimi: %s, parool: %s".to_string(),
    );
    translations.insert(
        "PostgreSQL username and/or password not valid".to_string(),
        "PostgreSQL kasutajatunnus ja/või parool pole õiged".to_string(),
    );
    translations.insert(
        "Set an admin username.".to_string(),
        "Määra admin kasutajanimi.".to_string(),
    );
    translations.insert(
        "Set an admin password.".to_string(),
        "Määra admini parool.".to_string(),
    );
    translations.insert(
        "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(),
        "Veebiserveri ei ole veel korralikult seadistatud võimaldamaks failide sünkroniseerimist, kuna WebDAV liides näib olevat mittetoimiv.".to_string(),
    );
    translations.insert(
        "Please double check the <a href='%s'>installation guides</a>.".to_string(),
        "Palun tutvu veelkord <a href='%s'>paigalduse juhenditega</a>.".to_string(),
    );
    translations.insert(
        "Could not find category \"%s\"".to_string(),
        "Ei leia kategooriat \"%s\"".to_string(),
    );
    translations.insert(
        "seconds ago".to_string(),
        "sekundit tagasi".to_string(),
    );
    translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        "::".to_string(),
    );
    translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        "::%n tundi tagasi".to_string(),
    );
    translations.insert("today".to_string(), "täna".to_string());
    translations.insert("yesterday".to_string(), "eile".to_string());
    translations.insert(
        "_%n day go_::_%n days ago_".to_string(),
        "::%n päeva tagasi".to_string(),
    );
    translations.insert("last month".to_string(), "viimasel kuul".to_string());
    translations.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        "::%n kuud tagasi".to_string(),
    );
    translations.insert(
        "last year".to_string(),
        "viimasel aastal".to_string(),
    );
    translations.insert("years ago".to_string(), "aastat tagasi".to_string());
    translations.insert("Caused by:".to_string(), "Põhjustaja:".to_string());

    translations
}

/// Define plural forms information for Estonian
pub fn et_plural_forms() -> String {
    "nplurals=2; plural=(n != 1);".to_string()
}

/// Register Estonian translations with the i18n system
pub fn register_et_translations() {
    let translations = et_ee_translations();
    let plural_forms = et_plural_forms();
    
    // This is donde differently depending on which i18n library you're using,
    // this is just a placeholder for the actual registration code
    i18n! {
        locale: "et_EE",
        translations: translations,
        plural_forms: plural_forms,
    }
}