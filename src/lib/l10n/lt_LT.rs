use std::collections::HashMap;
use rust_gettext::plural::PluralCategory;

pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.".to_string(), "Programa „%s" negali būti įdiegta, nes yra nesuderinama su šia ownCloud versija.".to_string());
    translations.insert("No app name specified".to_string(), "Nenurodytas programos pavadinimas".to_string());
    translations.insert("Help".to_string(), "Pagalba".to_string());
    translations.insert("Personal".to_string(), "Asmeniniai".to_string());
    translations.insert("Settings".to_string(), "Nustatymai".to_string());
    translations.insert("Users".to_string(), "Vartotojai".to_string());
    translations.insert("Admin".to_string(), "Administravimas".to_string());
    translations.insert("Failed to upgrade \"%s\".".to_string(), "Nepavyko pakelti  „%s" versijos.".to_string());
    translations.insert("Unknown filetype".to_string(), "Nežinomas failo tipas".to_string());
    translations.insert("Invalid image".to_string(), "Netinkamas paveikslėlis".to_string());
    translations.insert("web services under your control".to_string(), "jūsų valdomos web paslaugos".to_string());
    translations.insert("cannot open \"%s\"".to_string(), "nepavyksta atverti „%s"".to_string());
    translations.insert("ZIP download is turned off.".to_string(), "ZIP atsisiuntimo galimybė yra išjungta.".to_string());
    translations.insert("Files need to be downloaded one by one.".to_string(), "Failai turi būti parsiunčiami vienas po kito.".to_string());
    translations.insert("Back to Files".to_string(), "Atgal į Failus".to_string());
    translations.insert("Selected files too large to generate zip file.".to_string(), "Pasirinkti failai per dideli archyvavimui į ZIP.".to_string());
    translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.".to_string(), "Atsisiųskite failus mažesnėmis dalimis atskirai, arba mandagiai prašykite savo administratoriaus.".to_string());
    translations.insert("No source specified when installing app".to_string(), "Nenurodytas šaltinis diegiant programą".to_string());
    translations.insert("No href specified when installing app from http".to_string(), "Nenurodytas href diegiant programą iš http".to_string());
    translations.insert("No path specified when installing app from local file".to_string(), "Nenurodytas kelias diegiant programą iš vietinio failo".to_string());
    translations.insert("Archives of type %s are not supported".to_string(), "%s tipo archyvai nepalaikomi".to_string());
    translations.insert("Failed to open archive when installing app".to_string(), "Nepavyko atverti archyvo diegiant programą".to_string());
    translations.insert("App does not provide an info.xml file".to_string(), "Programa nepateikia info.xml failo".to_string());
    translations.insert("App can't be installed because of not allowed code in the App".to_string(), "Programa negali būti įdiegta, nes turi neleistiną kodą".to_string());
    translations.insert("App can't be installed because it is not compatible with this version of ownCloud".to_string(), "Programa negali būti įdiegta, nes yra nesuderinama su šia ownCloud versija".to_string());
    translations.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps".to_string(), "Programa negali būti įdiegta, nes turi <shipped>true</shipped> žymę, kuri yra neleistina ne kartu platinamoms programoms".to_string());
    translations.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store".to_string(), "Programa negali būti įdiegta, nes versija pateikta info.xml/version nesutampa su versija deklaruota programų saugykloje".to_string());
    translations.insert("App directory already exists".to_string(), "Programos aplankas jau egzistuoja".to_string());
    translations.insert("Can't create app folder. Please fix permissions. %s".to_string(), "Nepavyksta sukurti aplanko. Prašome pataisyti leidimus. %s".to_string());
    translations.insert("Application is not enabled".to_string(), "Programa neįjungta".to_string());
    translations.insert("Authentication error".to_string(), "Autentikacijos klaida".to_string());
    translations.insert("Token expired. Please reload page.".to_string(), "Sesija baigėsi. Prašome perkrauti puslapį.".to_string());
    translations.insert("Files".to_string(), "Failai".to_string());
    translations.insert("Text".to_string(), "Žinučių".to_string());
    translations.insert("Images".to_string(), "Paveikslėliai".to_string());
    translations.insert("%s enter the database username.".to_string(), "%s įrašykite duombazės naudotojo vardą.".to_string());
    translations.insert("%s enter the database name.".to_string(), "%s įrašykite duombazės pavadinimą.".to_string());
    translations.insert("%s you may not use dots in the database name".to_string(), "%s negalite naudoti taškų duombazės pavadinime".to_string());
    translations.insert("MS SQL username and/or password not valid: %s".to_string(), "MS SQL naudotojo vardas ir/arba slaptažodis netinka: %s".to_string());
    translations.insert("You need to enter either an existing account or the administrator.".to_string(), "Turite prisijungti su egzistuojančia paskyra arba su administratoriumi.".to_string());
    translations.insert("MySQL username and/or password not valid".to_string(), "Neteisingas MySQL naudotojo vardas ir/arba slaptažodis".to_string());
    translations.insert("DB Error: \"%s\"".to_string(), "DB klaida: \"%s\"".to_string());
    translations.insert("Offending command was: \"%s\"".to_string(), "Vykdyta komanda buvo: \"%s\"".to_string());
    translations.insert("MySQL user '%s'@'localhost' exists already.".to_string(), "MySQL naudotojas '%s'@'localhost' jau egzistuoja.".to_string());
    translations.insert("Drop this user from MySQL".to_string(), "Pašalinti šį naudotoją iš MySQL".to_string());
    translations.insert("MySQL user '%s'@'%%' already exists".to_string(), "MySQL naudotojas '%s'@'%%' jau egzistuoja".to_string());
    translations.insert("Drop this user from MySQL.".to_string(), "Pašalinti šį naudotoją iš MySQL.".to_string());
    translations.insert("Oracle connection could not be established".to_string(), "Nepavyko sukurti Oracle ryšio".to_string());
    translations.insert("Oracle username and/or password not valid".to_string(), "Neteisingas Oracle naudotojo vardas ir/arba slaptažodis".to_string());
    translations.insert("Offending command was: \"%s\", name: %s, password: %s".to_string(), "Vykdyta komanda buvo: \"%s\", name: %s, password: %s".to_string());
    translations.insert("PostgreSQL username and/or password not valid".to_string(), "Neteisingas PostgreSQL naudotojo vardas ir/arba slaptažodis".to_string());
    translations.insert("Set an admin username.".to_string(), "Nustatyti administratoriaus naudotojo vardą.".to_string());
    translations.insert("Set an admin password.".to_string(), "Nustatyti administratoriaus slaptažodį.".to_string());
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(), "Jūsų serveris nėra tvarkingai nustatytas leisti failų sinchronizaciją, nes WebDAV sąsaja panašu, kad yra sugadinta.".to_string());
    translations.insert("Please double check the <a href='%s'>installation guides</a>.".to_string(), "Prašome pažiūrėkite dar kartą <a href='%s'>diegimo instrukcijas</a>.".to_string());
    translations.insert("Could not find category \"%s\"".to_string(), "Nepavyko rasti kategorijos „%s"".to_string());
    translations.insert("seconds ago".to_string(), "prieš sekundę".to_string());
    translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), "prieš %n min.::Prieš % minutes::Prieš %n minučių".to_string());
    translations.insert("_%n hour ago_::_%n hours ago_".to_string(), "Prieš %n valandą::Prieš %n valandas::Prieš %n valandų".to_string());
    translations.insert("today".to_string(), "šiandien".to_string());
    translations.insert("yesterday".to_string(), "vakar".to_string());
    translations.insert("_%n day go_::_%n days ago_".to_string(), "Prieš %n dieną::Prieš %n dienas::Prieš %n dienų".to_string());
    translations.insert("last month".to_string(), "praeitą mėnesį".to_string());
    translations.insert("_%n month ago_::_%n months ago_".to_string(), "Prieš %n mėnesį::Prieš %n mėnesius::Prieš %n mėnesių".to_string());
    translations.insert("last year".to_string(), "praeitais metais".to_string());
    translations.insert("years ago".to_string(), "prieš metus".to_string());
    translations.insert("Caused by:".to_string(), "Iššaukė:".to_string());
    
    translations
}

pub fn get_plural_form() -> String {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && (n%100<10 || n%100>=20) ? 1 : 2);".to_string()
}

pub fn get_plural_category(n: i64) -> PluralCategory {
    if n % 10 == 1 && n % 100 != 11 {
        PluralCategory::One
    } else if n % 10 >= 2 && (n % 100 < 10 || n % 100 >= 20) {
        PluralCategory::Few
    } else {
        PluralCategory::Many
    }
}