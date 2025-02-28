use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Klucz odzyskiwania włączony");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Nie można włączyć klucza odzyskiwania. Proszę sprawdzić swoje hasło odzyskiwania!");
        m.insert("Recovery key successfully disabled", "Klucz odzyskiwania wyłączony");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Nie można wyłączyć klucza odzyskiwania. Proszę sprawdzić swoje hasło odzyskiwania!");
        m.insert("Password successfully changed.", "Zmiana hasła udana.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Nie można zmienić hasła. Może stare hasło nie było poprawne.");
        m.insert("Private key password successfully updated.", "Pomyślnie zaktualizowano hasło klucza prywatnego.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Nie można zmienić prywatnego hasła. Może stare hasło nie było poprawne.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Szyfrowanie aplikacja nie została zainicjowane! Może szyfrowanie aplikacji zostało ponownie włączone podczas tej sesji. Spróbuj się wylogować i zalogować ponownie aby zainicjować szyfrowanie aplikacji.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Nie można odszyfrować tego pliku, prawdopodobnie jest to plik udostępniony. Poproś właściciela pliku o ponowne udostępnianie pliku Tobie.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Nieznany błąd proszę sprawdzić ustawienia systemu lub skontaktuj się z administratorem");
        m.insert("Missing requirements.", "Brak wymagań.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Proszę upewnić się, że PHP 5.3.3 lub nowszy jest zainstalowany i że OpenSSL oraz rozszerzenie PHP jest włączone i poprawnie skonfigurowane. Obecnie szyfrowanie aplikacji zostało wyłączone.");
        m.insert("Following users are not set up for encryption:", "Następujący użytkownicy nie mają skonfigurowanego szyfrowania:");
        m.insert("Saving...", "Zapisywanie...");
        m.insert("Go directly to your ", "Przejdź bezpośrednio do");
        m.insert("personal settings", "Ustawienia osobiste");
        m.insert("Encryption", "Szyfrowanie");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Włączhasło klucza odzyskiwania (pozwala odzyskać pliki użytkowników w przypadku utraty hasła):");
        m.insert("Recovery key password", "Hasło klucza odzyskiwania");
        m.insert("Repeat Recovery key password", "Powtórz hasło klucza odzyskiwania");
        m.insert("Enabled", "Włączone");
        m.insert("Disabled", "Wyłączone");
        m.insert("Change recovery key password:", "Zmień hasło klucza odzyskiwania");
        m.insert("Old Recovery key password", "Stare hasło klucza odzyskiwania");
        m.insert("New Recovery key password", "Nowe hasło klucza odzyskiwania");
        m.insert("Repeat New Recovery key password", "Powtórz nowe hasło klucza odzyskiwania");
        m.insert("Change Password", "Zmień hasło");
        m.insert("Your private key password no longer match your log-in password:", "Hasło klucza prywatnego nie pasuje do  hasła logowania:");
        m.insert("Set your old private key password to your current log-in password.", "Podaj swoje stare prywatne hasło aby ustawić nowe");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Jeśli nie pamiętasz swojego starego hasła, poproś swojego administratora, aby odzyskać pliki.");
        m.insert("Old log-in password", "Stare hasło logowania");
        m.insert("Current log-in password", "Bieżące hasło logowania");
        m.insert("Update Private Key Password", "Aktualizacja hasła klucza prywatnego");
        m.insert("Enable password recovery:", "Włącz hasło odzyskiwania:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Włączenie tej opcji umożliwia otrzymać dostęp do zaszyfrowanych plików w przypadku utraty hasła");
        m.insert("File recovery settings updated", "Ustawienia odzyskiwania plików zmienione");
        m.insert("Could not update file recovery", "Nie można zmienić pliku odzyskiwania");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}

// Función para obtener traducción
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

// Función para obtener la fórmula de plurales
pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}