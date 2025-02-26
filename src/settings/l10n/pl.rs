use std::collections::HashMap;
use rust_i18n::i18n;

pub fn pl_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Unable to load list from App Store", "Nie można wczytać listy aplikacji");
    translations.insert("Authentication error", "Błąd uwierzytelniania");
    translations.insert("Group already exists", "Grupa już istnieje");
    translations.insert("Unable to add group", "Nie można dodać grupy");
    translations.insert("Email saved", "E-mail zapisany");
    translations.insert("Invalid email", "Nieprawidłowy e-mail");
    translations.insert("Unable to delete group", "Nie można usunąć grupy");
    translations.insert("Unable to delete user", "Nie można usunąć użytkownika");
    translations.insert("Language changed", "Zmieniono język");
    translations.insert("Invalid request", "Nieprawidłowe żądanie");
    translations.insert("Admins can't remove themself from the admin group", "Administratorzy nie mogą usunąć siebie samych z grupy administratorów");
    translations.insert("Unable to add user to group %s", "Nie można dodać użytkownika do grupy %s");
    translations.insert("Unable to remove user from group %s", "Nie można usunąć użytkownika z grupy %s");
    translations.insert("Couldn't update app.", "Nie można uaktualnić aplikacji.");
    translations.insert("Wrong password", "Złe hasło");
    translations.insert("Unable to change password", "Nie można zmienić hasła");
    translations.insert("Update to {appversion}", "Aktualizacja do {appversion}");
    translations.insert("Disable", "Wyłącz");
    translations.insert("Enable", "Włącz");
    translations.insert("Please wait....", "Proszę czekać...");
    translations.insert("Error while disabling app", "Błąd podczas wyłączania aplikacji");
    translations.insert("Error while enabling app", "Błąd podczas włączania aplikacji");
    translations.insert("Updating....", "Aktualizacja w toku...");
    translations.insert("Error while updating app", "Błąd podczas aktualizacji aplikacji");
    translations.insert("Error", "Błąd");
    translations.insert("Update", "Aktualizuj");
    translations.insert("Updated", "Zaktualizowano");
    translations.insert("Select a profile picture", "Wybierz zdjęcie profilu");
    translations.insert("Decrypting files... Please wait, this can take some time.", "Odszyfrowuje pliki... Proszę czekać, to może zająć jakiś czas.");
    translations.insert("Saving...", "Zapisywanie...");
    translations.insert("deleted", "usunięto");
    translations.insert("undo", "cofnij");
    translations.insert("Unable to remove user", "Nie można usunąć użytkownika");
    translations.insert("Groups", "Grupy");
    translations.insert("Group Admin", "Administrator grupy");
    translations.insert("Delete", "Usuń");
    translations.insert("add group", "dodaj grupę");
    translations.insert("A valid username must be provided", "Należy podać prawidłową nazwę użytkownika");
    translations.insert("Error creating user", "Błąd podczas tworzenia użytkownika");
    translations.insert("A valid password must be provided", "Należy podać prawidłowe hasło");
    translations.insert("__language_name__", "polski");
    translations.insert("Security Warning", "Ostrzeżenie o zabezpieczeniach");
    translations.insert("Your data directory and your files are probably accessible from the internet. The .htaccess file is not working. We strongly suggest that you configure your webserver in a way that the data directory is no longer accessible or you move the data directory outside the webserver document root.", "Twój katalog danych i pliki są prawdopodobnie dostępne z Internetu. Plik .htaccess, który dostarcza ownCloud nie działa. Sugerujemy, aby skonfigurować serwer WWW w taki sposób, aby katalog danych nie był dostępny lub przenieść katalog danych poza główny katalog serwera WWW.");
    translations.insert("Setup Warning", "Ostrzeżenia konfiguracji");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Serwer internetowy nie jest jeszcze poprawnie skonfigurowany, aby umożliwić synchronizację plików, ponieważ interfejs WebDAV wydaje się być uszkodzony.");
    translations.insert("Please double check the <a href=\"%s\">installation guides</a>.", "Proszę sprawdź ponownie <a href=\"%s\">przewodnik instalacji</a>.");
    translations.insert("Module 'fileinfo' missing", "Brak modułu „fileinfo"");
    translations.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "Brak modułu PHP „fileinfo". Zalecamy włączenie tego modułu, aby uzyskać najlepsze wyniki podczas wykrywania typów MIME.");
    translations.insert("Locale not working", "Lokalizacja nie działa");
    translations.insert("System locale can't be set to %s. This means that there might be problems with certain characters in file names. We strongly suggest to install the required packages on your system to support %s.", "System lokalny nie może włączyć ustawień regionalnych %s. Może to oznaczać, że wystąpiły problemy z niektórymi znakami w nazwach plików. Zalecamy instalację wymaganych pakietów na tym systemie w celu wsparcia %s.");
    translations.insert("Internet connection not working", "Połączenie internetowe nie działa");
    translations.insert("This server has no working internet connection. This means that some of the features like mounting of external storage, notifications about updates or installation of 3rd party apps don´t work. Accessing files from remote and sending of notification emails might also not work. We suggest to enable internet connection for this server if you want to have all features.", "Ten serwer OwnCloud nie ma połączenia z Internetem. Oznacza to, że niektóre z funkcji, takich jak montowanie zewnętrznych zasobów, powiadomienia o aktualizacji lub 3-cie aplikacje mogą nie działać. Dostęp do plików z zewnątrz i wysyłanie powiadomienia e-mail nie może również działać. Sugerujemy, aby włączyć połączenia internetowego dla tego serwera, jeśli chcesz mieć wszystkie opcje.");
    translations.insert("Cron", "Cron");
    translations.insert("Execute one task with each page loaded", "Wykonuj jedno zadanie wraz z każdą wczytaną stroną");
    translations.insert("cron.php is registered at a webcron service to call cron.php every 15 minutes over http.", "cron.php jest zarejestrowany w serwisie webcron do uruchamiania cron.php raz na 15 minut przez http.");
    translations.insert("Use systems cron service to call the cron.php file every 15 minutes.", "Użyj systemowego cron-a do uruchamiania cron.php raz na 15 minut.");
    translations.insert("Sharing", "Udostępnianie");
    translations.insert("Enable Share API", "Włącz API udostępniania");
    translations.insert("Allow apps to use the Share API", "Zezwalaj aplikacjom na korzystanie z API udostępniania");
    translations.insert("Allow links", "Zezwalaj na odnośniki");
    translations.insert("Allow users to share items to the public with links", "Zezwalaj użytkownikom na publiczne współdzielenie zasobów za pomocą odnośników");
    translations.insert("Allow public uploads", "Pozwól na  publiczne wczytywanie");
    translations.insert("Allow users to enable others to upload into their publicly shared folders", "Użytkownicy mogą włączyć dla innych wgrywanie do ich publicznych katalogów");
    translations.insert("Allow resharing", "Zezwalaj na ponowne udostępnianie");
    translations.insert("Allow users to share items shared with them again", "Zezwalaj użytkownikom na ponowne współdzielenie zasobów już z nimi współdzielonych");
    translations.insert("Allow users to share with anyone", "Zezwalaj użytkownikom na współdzielenie z kimkolwiek");
    translations.insert("Allow users to only share with users in their groups", "Zezwalaj użytkownikom współdzielić z użytkownikami ze swoich grup");
    translations.insert("Allow mail notification", "Pozwól na mailowe powiadomienia");
    translations.insert("Allow user to send mail notification for shared files", "Pozwól użytkownikom wysyłać maile powiadamiające o udostępnionych plikach");
    translations.insert("Security", "Bezpieczeństwo");
    translations.insert("Enforce HTTPS", "Wymuś HTTPS");
    translations.insert("Forces the clients to connect to %s via an encrypted connection.", "Wymusza na klientach na łączenie się %s za pośrednictwem połączenia szyfrowanego.");
    translations.insert("Please connect to your %s via HTTPS to enable or disable the SSL enforcement.", "Proszę połącz się do twojego %s za pośrednictwem protokołu HTTPS, aby włączyć lub wyłączyć stosowanie protokołu SSL.");
    translations.insert("Log", "Logi");
    translations.insert("Log level", "Poziom logów");
    translations.insert("More", "Więcej");
    translations.insert("Less", "Mniej");
    translations.insert("Version", "Wersja");
    translations.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Stworzone przez <a href=\"http://ownCloud.org/contact\" target=\"_blank\">społeczność ownCloud</a>, <a href=\"https://github.com/owncloud\" target=\"_blank\">kod źródłowy</a> na licencji <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
    translations.insert("Add your App", "Dodaj swoją aplikację");
    translations.insert("More Apps", "Więcej aplikacji");
    translations.insert("Select an App", "Zaznacz aplikację");
    translations.insert("See application page at apps.owncloud.com", "Zobacz stronę aplikacji na apps.owncloud.com");
    translations.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-licencjonowane przez <span class=\"author\"></span>");
    translations.insert("User Documentation", "Dokumentacja użytkownika");
    translations.insert("Administrator Documentation", "Dokumentacja administratora");
    translations.insert("Online Documentation", "Dokumentacja online");
    translations.insert("Forum", "Forum");
    translations.insert("Bugtracker", "Zgłaszanie błędów");
    translations.insert("Commercial Support", "Wsparcie komercyjne");
    translations.insert("Get the apps to sync your files", "Pobierz aplikacje żeby synchronizować swoje pliki");
    translations.insert("Show First Run Wizard again", "Uruchom ponownie kreatora pierwszego uruchomienia");
    translations.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Wykorzystujesz <strong>%s</strong> z dostępnych <strong>%s</strong>");
    translations.insert("Password", "Hasło");
    translations.insert("Your password was changed", "Twoje hasło zostało zmienione");
    translations.insert("Unable to change your password", "Nie można zmienić hasła");
    translations.insert("Current password", "Bieżące hasło");
    translations.insert("New password", "Nowe hasło");
    translations.insert("Change password", "Zmień hasło");
    translations.insert("Email", "Email");
    translations.insert("Your email address", "Twój adres e-mail");
    translations.insert("Fill in an email address to enable password recovery", "Podaj adres e-mail, aby uzyskać możliwość odzyskania hasła");
    translations.insert("Profile picture", "Zdjęcie profilu");
    translations.insert("Upload new", "Wczytaj nowe");
    translations.insert("Select new from Files", "Wybierz nowe z plików");
    translations.insert("Remove image", "Usuń zdjęcie");
    translations.insert("Either png or jpg. Ideally square but you will be able to crop it.", "Png lub jpg. Idealnie kwadratowy, ale będzie można je przyciąć.");
    translations.insert("Abort", "Anuluj");
    translations.insert("Choose as profile image", "Wybierz zdjęcie profilu");
    translations.insert("Language", "Język");
    translations.insert("Help translate", "Pomóż w tłumaczeniu");
    translations.insert("WebDAV", "WebDAV");
    translations.insert("Encryption", "Szyfrowanie");
    translations.insert("The encryption app is no longer enabled, decrypt all your file", "Aplikacja szyfrowanie nie jest włączona, odszyfruj wszystkie plik");
    translations.insert("Log-in password", "Hasło logowania");
    translations.insert("Decrypt all Files", "Odszyfruj wszystkie pliki");
    translations.insert("Login Name", "Login");
    translations.insert("Create", "Utwórz");
    translations.insert("Admin Recovery Password", "Odzyskiwanie hasła administratora");
    translations.insert("Enter the recovery password in order to recover the users files during password change", "Wpisz hasło odzyskiwania, aby odzyskać pliki użytkowników podczas zmiany hasła");
    translations.insert("Default Storage", "Magazyn domyślny");
    translations.insert("Unlimited", "Bez limitu");
    translations.insert("Other", "Inne");
    translations.insert("Username", "Nazwa użytkownika");
    translations.insert("Storage", "Magazyn");
    translations.insert("set new password", "ustaw nowe hasło");
    translations.insert("Default", "Domyślny");
    
    translations
}

pub fn pl_plural_forms() -> &'static str {
    "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

#[cfg(feature = "i18n")]
i18n!("pl");