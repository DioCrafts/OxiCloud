use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to clear the mappings.", "Nie udało się wyczyścić mapowania.");
        m.insert("Failed to delete the server configuration", "Nie można usunąć konfiguracji serwera");
        m.insert("The configuration is valid and the connection could be established!", "Konfiguracja jest prawidłowa i można ustanowić połączenie!");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Konfiguracja jest prawidłowa, ale Bind nie. Sprawdź ustawienia serwera i poświadczenia.");
        m.insert("Deletion failed", "Usunięcie nie powiodło się");
        m.insert("Take over settings from recent server configuration?", "Przejmij ustawienia z ostatnich konfiguracji serwera?");
        m.insert("Keep settings?", "Zachować ustawienia?");
        m.insert("Cannot add server configuration", "Nie można dodać konfiguracji serwera");
        m.insert("mappings cleared", "Mapoanie wyczyszczone");
        m.insert("Success", "Sukces");
        m.insert("Error", "Błąd");
        m.insert("Select groups", "Wybierz grupy");
        m.insert("Connection test succeeded", "Test połączenia udany");
        m.insert("Connection test failed", "Test połączenia nie udany");
        m.insert("Do you really want to delete the current Server Configuration?", "Czy chcesz usunąć bieżącą konfigurację serwera?");
        m.insert("Confirm Deletion", "Potwierdź usunięcie");
        m.insert("Save", "Zapisz");
        m.insert("Test Configuration", "Konfiguracja testowa");
        m.insert("Help", "Pomoc");
        m.insert("Add Server Configuration", "Dodaj konfigurację servera");
        m.insert("Host", "Host");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Można pominąć protokół, z wyjątkiem wymaganego protokołu SSL. Następnie uruchom z ldaps://");
        m.insert("Port", "Port");
        m.insert("User DN", "Użytkownik DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN użytkownika klienta, z którym powiązanie wykonuje się, np. uid=agent,dc=example,dc=com. Dla dostępu anonimowego pozostawić DN i hasło puste");
        m.insert("Password", "Hasło");
        m.insert("For anonymous access, leave DN and Password empty.", "Dla dostępu anonimowego pozostawić DN i hasło puste.");
        m.insert("One Base DN per line", "Jedna baza DN na linię");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "Bazę DN można określić dla użytkowników i grup w karcie Zaawansowane");
        m.insert("Back", "Wróć");
        m.insert("Continue", "Kontynuuj ");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Ostrzeżenie:</b>  Moduł PHP LDAP nie jest zainstalowany i nie będzie działał. Poproś administratora o włączenie go.");
        m.insert("Connection Settings", "Konfiguracja połączeń");
        m.insert("Configuration Active", "Konfiguracja archiwum");
        m.insert("When unchecked, this configuration will be skipped.", "Gdy niezaznaczone, ta konfiguracja zostanie pominięta.");
        m.insert("User Login Filter", "Filtr logowania użytkownika");
        m.insert("Backup (Replica) Host", "Kopia zapasowa (repliki) host");
        m.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Dać opcjonalnie  hosta kopii zapasowej . To musi być repliką głównego serwera LDAP/AD.");
        m.insert("Backup (Replica) Port", "Kopia zapasowa (repliki) Port");
        m.insert("Disable Main Server", "Wyłącz serwer główny");
        m.insert("Only connect to the replica server.", "Połącz tylko do repliki serwera.");
        m.insert("Case insensitve LDAP server (Windows)", "Wielkość liter serwera LDAP (Windows)");
        m.insert("Turn off SSL certificate validation.", "Wyłączyć sprawdzanie poprawności certyfikatu SSL.");
        m.insert("Cache Time-To-Live", "Przechowuj czas życia");
        m.insert("in seconds. A change empties the cache.", "w sekundach. Zmiana opróżnia pamięć podręczną.");
        m.insert("Directory Settings", "Ustawienia katalogów");
        m.insert("User Display Name Field", "Pole wyświetlanej nazwy użytkownika");
        m.insert("Base User Tree", "Drzewo bazy użytkowników");
        m.insert("One User Base DN per line", "Jeden użytkownik Bazy DN na linię");
        m.insert("User Search Attributes", "Szukaj atrybutów");
        m.insert("Optional; one attribute per line", "Opcjonalnie; jeden atrybut w wierszu");
        m.insert("Group Display Name Field", "Pole wyświetlanej nazwy grupy");
        m.insert("Base Group Tree", "Drzewo bazy grup");
        m.insert("One Group Base DN per line", "Jedna grupa bazy DN na linię");
        m.insert("Group Search Attributes", "Grupa atrybutów wyszukaj");
        m.insert("Group-Member association", "Członek grupy stowarzyszenia");
        m.insert("Special Attributes", "Specjalne atrybuty");
        m.insert("Quota Field", "Pole przydziału");
        m.insert("Quota Default", "Przydział domyślny");
        m.insert("in bytes", "w bajtach");
        m.insert("Email Field", "Pole email");
        m.insert("User Home Folder Naming Rule", "Reguły nazewnictwa folderu domowego użytkownika");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Pozostaw puste dla user name (domyślnie). W przeciwnym razie podaj atrybut LDAP/AD.");
        m.insert("Internal Username", "Wewnętrzna nazwa użytkownika");
        m.insert("Internal Username Attribute:", "Wewnętrzny atrybut nazwy uzżytkownika:");
        m.insert("Override UUID detection", "Zastąp wykrywanie UUID");
        m.insert("UUID Attribute for Users:", "Atrybuty UUID dla użytkowników:");
        m.insert("UUID Attribute for Groups:", "Atrybuty UUID dla grup:");
        m.insert("Username-LDAP User Mapping", "Mapowanie użytkownika LDAP");
        m.insert("Clear Username-LDAP User Mapping", "Czyść Mapowanie użytkownika LDAP");
        m.insert("Clear Groupname-LDAP Group Mapping", "Czyść Mapowanie nazwy grupy LDAP");
        m
    };
}

// Función para manejar plurales en polaco
pub fn get_plural_form(n: i64) -> usize {
    if n == 1 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

// Función para manejar el caso específico de los plurales
pub fn translate_plural(singular: &str, plural: &str, n: i64) -> String {
    let form = get_plural_form(n);
    match (singular, plural, form) {
        ("_%s group found_", "_%s groups found_", 0) => format!("{} grupa znaleziona", n),
        ("_%s group found_", "_%s groups found_", 1) => format!("{} grupy znalezione", n),
        ("_%s group found_", "_%s groups found_", 2) => format!("{} grup znalezionych", n),
        
        ("_%s user found_", "_%s users found_", 0) => format!("{} użytkownik znaleziony", n),
        ("_%s user found_", "_%s users found_", 1) => format!("{} użytkowników znalezionych", n),
        ("_%s user found_", "_%s users found_", 2) => format!("{} użytkowników znalezionych", n),
        
        _ => plural.to_string(),
    }
}

// Función principal para traducciones
pub fn translate(key: &str) -> String {
    match TRANSLATIONS.get(key) {
        Some(translation) => translation.to_string(),
        None => key.to_string(),
    }
}