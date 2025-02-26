use phf::phf_map;
use rust_i18n::locale::Locale;

pub struct HuHU;

impl Locale for HuHU {
    fn id(&self) -> &'static str {
        "hu_HU"
    }

    fn plural_forms(&self) -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }
}

pub static HU_HU_TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "App \"%s\" can't be installed because it is not compatible with this version of ownCloud." => "A(z) \"%s\" alkalmazást nem lehet telepíteni, mert nem kompatibilis az ownCloud telepített verziójával.",
    "No app name specified" => "Nincs az alkalmazás név megadva.",
    "Help" => "Súgó",
    "Personal" => "Személyes",
    "Settings" => "Beállítások",
    "Users" => "Felhasználók",
    "Admin" => "Adminsztráció",
    "Failed to upgrade \"%s\"." => "Sikertelen Frissítés \"%s\".",
    "Unknown filetype" => "Ismeretlen file tipús",
    "Invalid image" => "Hibás kép",
    "web services under your control" => "webszolgáltatások saját kézben",
    "cannot open \"%s\"" => "nem sikerült megnyitni \"%s\"",
    "ZIP download is turned off." => "A ZIP-letöltés nincs engedélyezve.",
    "Files need to be downloaded one by one." => "A fájlokat egyenként kell letölteni.",
    "Back to Files" => "Vissza a Fájlokhoz",
    "Selected files too large to generate zip file." => "A kiválasztott fájlok túl nagyok a zip tömörítéshez.",
    "Download the files in smaller chunks, seperately or kindly ask your administrator." => "Tölts le a fileokat kisebb chunkokban, kölün vagy kérj segitséget a rendszergazdádtól.",
    "No source specified when installing app" => "Az alkalmazás telepítéséhez nincs forrás megadva",
    "No href specified when installing app from http" => "Az alkalmazás http-n keresztül történő telepítéséhez nincs href hivetkozás megadva",
    "No path specified when installing app from local file" => "Az alkalmazás helyi telepítéséhez nincs útvonal (mappa) megadva",
    "Archives of type %s are not supported" => "A(z) %s típusú tömörített állomány nem támogatott",
    "Failed to open archive when installing app" => "Nem sikerült megnyitni a tömörített állományt a telepítés során",
    "App does not provide an info.xml file" => "Az alkalmazás nem szolgáltatott info.xml file-t",
    "App can't be installed because of not allowed code in the App" => "Az alkalmazást nem lehet telepíteni, mert abban nem engedélyezett programkód szerepel",
    "App can't be installed because it is not compatible with this version of ownCloud" => "Az alalmazás nem telepíthető, mert nem kompatibilis az ownClod ezzel a verziójával.",
    "App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps" => "Az alkalmazást nem lehet telepíteni, mert tartalmazza a \n<shipped>\ntrue\n</shipped>\ncímkét, ami a nem szállított alkalmazások esetén nem engedélyezett",
    "App can't be installed because the version in info.xml/version is not the same as the version reported from the app store" => "Az alkalmazást nem lehet telepíteni, mert az info.xml/version-ben megadott verzió nem egyezik az alkalmazás-áruházban feltüntetett verzióval.",
    "App directory already exists" => "Az alkalmazás mappája már létezik",
    "Can't create app folder. Please fix permissions. %s" => "Nem lehetett létrehozni az alkalmzás mappáját. Kérlek ellenőrizd a jogosultásgokat. %s",
    "Application is not enabled" => "Az alkalmazás nincs engedélyezve",
    "Authentication error" => "Azonosítási hiba",
    "Token expired. Please reload page." => "A token lejárt. Frissítse az oldalt.",
    "Files" => "Fájlok",
    "Text" => "Szöveg",
    "Images" => "Képek",
    "%s enter the database username." => "%s adja meg az adatbázist elérő felhasználó login nevét.",
    "%s enter the database name." => "%s adja meg az adatbázis nevét.",
    "%s you may not use dots in the database name" => "%s az adatbázis neve nem tartalmazhat pontot",
    "MS SQL username and/or password not valid: %s" => "Az MS SQL felhasználónév és/vagy jelszó érvénytelen: %s",
    "You need to enter either an existing account or the administrator." => "Vagy egy létező felhasználó vagy az adminisztrátor bejelentkezési nevét kell megadnia",
    "MySQL username and/or password not valid" => "A MySQL felhasználói név és/vagy jelszó érvénytelen",
    "DB Error: \"%s\"" => "Adatbázis hiba: \"%s\"",
    "Offending command was: \"%s\"" => "A hibát ez a parancs okozta: \"%s\"",
    "MySQL user '%s'@'localhost' exists already." => "A '%s'@'localhost' MySQL felhasználó már létezik.",
    "Drop this user from MySQL" => "Törölje ezt a felhasználót a MySQL-ből",
    "MySQL user '%s'@'%%' already exists" => "A '%s'@'%%' MySQL felhasználó már létezik",
    "Drop this user from MySQL." => "Törölje ezt a felhasználót a MySQL-ből.",
    "Oracle connection could not be established" => "Az Oracle kapcsolat nem hozható létre",
    "Oracle username and/or password not valid" => "Az Oracle felhasználói név és/vagy jelszó érvénytelen",
    "Offending command was: \"%s\", name: %s, password: %s" => "A hibát okozó parancs ez volt: \"%s\", login név: %s, jelszó: %s",
    "PostgreSQL username and/or password not valid" => "A PostgreSQL felhasználói név és/vagy jelszó érvénytelen",
    "Set an admin username." => "Állítson be egy felhasználói nevet az adminisztrációhoz.",
    "Set an admin password." => "Állítson be egy jelszót az adminisztrációhoz.",
    "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken." => "Az Ön webkiszolgálója nincs megfelelően beállítva az állományok szinkronizálásához, mert a WebDAV-elérés úgy tűnik, nem működik.",
    "Please double check the <a href='%s'>installation guides</a>." => "Kérjük tüzetesen tanulmányozza át a <a href='%s'>telepítési útmutatót</a>.",
    "Could not find category \"%s\"" => "Ez a kategória nem található: \"%s\"",
    "seconds ago" => "pár másodperce",
    "_%n minute ago_::_%n minutes ago_" => "%n perccel ezelőtt",
    "_%n hour ago_::_%n hours ago_" => "%n órával ezelőtt",
    "today" => "ma",
    "yesterday" => "tegnap",
    "_%n day go_::_%n days ago_" => "%n nappal ezelőtt",
    "last month" => "múlt hónapban",
    "_%n month ago_::_%n months ago_" => "%n hónappal ezelőtt",
    "last year" => "tavaly",
    "years ago" => "több éve",
    "Caused by:" => "Okozta:",
};

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_translations() -> &'static phf::Map<&'static str, &'static str> {
    &HU_HU_TRANSLATIONS
}