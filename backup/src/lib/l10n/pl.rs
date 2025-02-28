use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "Aplikacja \"%s\" nie może zostać zainstalowana, ponieważ nie jest zgodna z tą wersją ownCloud.");
        m.insert("No app name specified", "Nie określono nazwy aplikacji");
        m.insert("Help", "Pomoc");
        m.insert("Personal", "Osobiste");
        m.insert("Settings", "Ustawienia");
        m.insert("Users", "Użytkownicy");
        m.insert("Admin", "Administrator");
        m.insert("Failed to upgrade \"%s\".", "Błąd przy aktualizacji \"%s\".");
        m.insert("Unknown filetype", "Nieznany typ pliku");
        m.insert("Invalid image", "Błędne zdjęcie");
        m.insert("web services under your control", "Kontrolowane serwisy");
        m.insert("cannot open \"%s\"", "Nie można otworzyć \"%s\"");
        m.insert("ZIP download is turned off.", "Pobieranie ZIP jest wyłączone.");
        m.insert("Files need to be downloaded one by one.", "Pliki muszą zostać pobrane pojedynczo.");
        m.insert("Back to Files", "Wróć do plików");
        m.insert("Selected files too large to generate zip file.", "Wybrane pliki są zbyt duże, aby wygenerować plik zip.");
        m.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Pobierz pliki w mniejszy kawałkach, oddzielnie lub poproś administratora o zwiększenie limitu.");
        m.insert("No source specified when installing app", "Nie określono źródła  podczas instalacji aplikacji");
        m.insert("No href specified when installing app from http", "Nie określono linku skąd aplikacja ma być zainstalowana");
        m.insert("No path specified when installing app from local file", "Nie określono lokalnego pliku z którego miała być instalowana aplikacja");
        m.insert("Archives of type %s are not supported", "Typ archiwum %s nie jest obsługiwany");
        m.insert("Failed to open archive when installing app", "Nie udało się otworzyć archiwum podczas instalacji aplikacji");
        m.insert("App does not provide an info.xml file", "Aplikacja nie posiada pliku info.xml");
        m.insert("App can't be installed because of not allowed code in the App", "Aplikacja nie może być zainstalowany ponieważ nie dopuszcza kod w aplikacji");
        m.insert("App can't be installed because it is not compatible with this version of ownCloud", "Aplikacja nie może zostać zainstalowana ponieważ jest niekompatybilna z tą wersja ownCloud");
        m.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "Aplikacja nie może być zainstalowana ponieważ true tag nie jest <shipped>true</shipped> , co nie jest dozwolone dla aplikacji nie wysłanych");
        m.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "Nie można zainstalować aplikacji, ponieważ w wersji info.xml/version nie jest taka sama, jak wersja z app store");
        m.insert("App directory already exists", "Katalog aplikacji już isnieje");
        m.insert("Can't create app folder. Please fix permissions. %s", "Nie mogę utworzyć katalogu aplikacji. Proszę popraw uprawnienia. %s");
        m.insert("Application is not enabled", "Aplikacja nie jest włączona");
        m.insert("Authentication error", "Błąd uwierzytelniania");
        m.insert("Token expired. Please reload page.", "Token wygasł. Proszę ponownie załadować stronę.");
        m.insert("Files", "Pliki");
        m.insert("Text", "Połączenie tekstowe");
        m.insert("Images", "Obrazy");
        m.insert("%s enter the database username.", "%s wpisz nazwę użytkownika do  bazy");
        m.insert("%s enter the database name.", "%s wpisz nazwę bazy.");
        m.insert("%s you may not use dots in the database name", "%s nie można używać kropki w nazwie bazy danych");
        m.insert("MS SQL username and/or password not valid: %s", "Nazwa i/lub hasło serwera MS SQL jest niepoprawne: %s.");
        m.insert("You need to enter either an existing account or the administrator.", "Należy wprowadzić istniejące konto użytkownika lub  administratora.");
        m.insert("MySQL username and/or password not valid", "MySQL: Nazwa użytkownika i/lub hasło jest niepoprawne");
        m.insert("DB Error: \"%s\"", "Błąd DB: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Niepoprawna komenda: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "Użytkownik MySQL  '%s'@'localhost' już istnieje");
        m.insert("Drop this user from MySQL", "Usuń tego użytkownika z MySQL");
        m.insert("MySQL user '%s'@'%%' already exists", "Użytkownik MySQL  '%s'@'%%t' już istnieje");
        m.insert("Drop this user from MySQL.", "Usuń tego użytkownika z MySQL.");
        m.insert("Oracle connection could not be established", "Nie można ustanowić połączenia z bazą Oracle");
        m.insert("Oracle username and/or password not valid", "Oracle: Nazwa użytkownika i/lub hasło jest niepoprawne");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Niepoprawne polecania:  \"%s\", nazwa: %s, hasło: %s");
        m.insert("PostgreSQL username and/or password not valid", "PostgreSQL: Nazwa użytkownika i/lub hasło jest niepoprawne");
        m.insert("Set an admin username.", "Ustaw nazwę administratora.");
        m.insert("Set an admin password.", "Ustaw hasło administratora.");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Serwer internetowy nie jest jeszcze poprawnie skonfigurowany, aby umożliwić synchronizację plików, ponieważ interfejs WebDAV wydaje się być uszkodzony.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Sprawdź ponownie <a href='%s'>przewodniki instalacji</a>.");
        m.insert("Could not find category \"%s\"", "Nie można odnaleźć kategorii \"%s\"");
        m.insert("seconds ago", "sekund temu");
        m.insert("today", "dziś");
        m.insert("yesterday", "wczoraj");
        m.insert("last month", "w zeszłym miesiącu");
        m.insert("last year", "w zeszłym roku");
        m.insert("years ago", "lat temu");
        m.insert("Caused by:", "Spowodowane przez:");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

    // Plurals require special handling in Rust
    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["%n minute temu", "%n minut temu", "%n minut temu"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["%n godzinę temu", "%n godzin temu", "%n godzin temu"]);
        m.insert("_%n day go_::_%n days ago_", vec!["%n dzień temu", "%n dni temu", "%n dni temu"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["%n miesiąc temu", "%n miesięcy temu", "%n miesięcy temu"]);
        m
    };
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, count: i64) -> &'static str {
    if let Some(forms) = PLURAL_TRANSLATIONS.get(key) {
        let idx = get_plural_index(count);
        forms.get(idx).copied().unwrap_or(key)
    } else {
        key
    }
}

fn get_plural_index(n: i64) -> usize {
    if n == 1 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}