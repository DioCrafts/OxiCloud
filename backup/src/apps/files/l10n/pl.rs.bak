use std::collections::HashMap;
use std::sync::LazyLock;

pub static TRANSLATIONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("Could not move %s - File with this name already exists", "Nie można było przenieść %s - Plik o takiej nazwie już istnieje");
    m.insert("Could not move %s", "Nie można było przenieść %s");
    m.insert("File name cannot be empty.", "Nazwa pliku nie może być pusta.");
    m.insert("Folder name cannot be empty.", "Nazwa folderu nie może być pusta.");
    m.insert("Unable to set upload directory.", "Nie można ustawić katalog wczytywania.");
    m.insert("Invalid Token", "Nieprawidłowy Token");
    m.insert("No file was uploaded. Unknown error", "Żaden plik nie został załadowany. Nieznany błąd");
    m.insert("There is no error, the file uploaded with success", "Nie było błędów, plik wysłano poprawnie.");
    m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Wgrany plik przekracza wartość upload_max_filesize zdefiniowaną w php.ini: ");
    m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Wysłany plik przekracza wielkość dyrektywy MAX_FILE_SIZE określonej w formularzu HTML");
    m.insert("The uploaded file was only partially uploaded", "Załadowany plik został wysłany tylko częściowo.");
    m.insert("No file was uploaded", "Nie wysłano żadnego pliku");
    m.insert("Missing a temporary folder", "Brak folderu tymczasowego");
    m.insert("Failed to write to disk", "Błąd zapisu na dysk");
    m.insert("Not enough storage available", "Za mało dostępnego miejsca");
    m.insert("Upload failed. Could not get file info.", "Nieudane przesłanie. Nie można pobrać informacji o pliku.");
    m.insert("Upload failed. Could not find uploaded file", "Nieudane przesłanie. Nie można znaleźć przesyłanego pliku");
    m.insert("Invalid directory.", "Zła ścieżka.");
    m.insert("Files", "Pliki");
    m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Nie można przesłać {filename} być może jest katalogiem lub posiada 0 bajtów");
    m.insert("Not enough space available", "Za mało miejsca");
    m.insert("Upload cancelled.", "Wczytywanie anulowane.");
    m.insert("Could not get result from server.", "Nie można uzyskać wyniku z serwera.");
    m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Wysyłanie pliku jest w toku. Jeśli opuścisz tę stronę, wysyłanie zostanie przerwane.");
    m.insert("{new_name} already exists", "{new_name} już istnieje");
    m.insert("Share", "Udostępnij");
    m.insert("Delete permanently", "Trwale usuń");
    m.insert("Rename", "Zmień nazwę");
    m.insert("Pending", "Oczekujące");
    m.insert("replaced {new_name} with {old_name}", "zastąpiono {new_name} przez {old_name}");
    m.insert("undo", "cofnij");
    m.insert("Error moving file", "Błąd prz przenoszeniu pliku");
    m.insert("Error", "Błąd");
    m.insert("Name", "Nazwa");
    m.insert("Size", "Rozmiar");
    m.insert("Modified", "Modyfikacja");
    m.insert("%s could not be renamed", "%s nie można zmienić nazwy");
    m.insert("Upload", "Wyślij");
    m.insert("File handling", "Zarządzanie plikami");
    m.insert("Maximum upload size", "Maksymalny rozmiar wysyłanego pliku");
    m.insert("max. possible: ", "maks. możliwy:");
    m.insert("Needed for multi-file and folder downloads.", "Wymagany do pobierania wielu plików i folderów");
    m.insert("Enable ZIP-download", "Włącz pobieranie ZIP-paczki");
    m.insert("0 is unlimited", "0 - bez limitów");
    m.insert("Maximum input size for ZIP files", "Maksymalna wielkość pliku wejściowego ZIP ");
    m.insert("Save", "Zapisz");
    m.insert("New", "Nowy");
    m.insert("Text file", "Plik tekstowy");
    m.insert("Folder", "Folder");
    m.insert("From link", "Z odnośnika");
    m.insert("Deleted files", "Pliki usunięte");
    m.insert("Cancel upload", "Anuluj wysyłanie");
    m.insert("Nothing in here. Upload something!", "Pusto. Wyślij coś!");
    m.insert("Download", "Pobierz");
    m.insert("Unshare", "Zatrzymaj współdzielenie");
    m.insert("Delete", "Usuń");
    m.insert("Upload too large", "Ładowany plik jest za duży");
    m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Pliki, które próbujesz przesłać, przekraczają maksymalną dopuszczalną wielkość.");
    m.insert("Files are being scanned, please wait.", "Skanowanie plików, proszę czekać.");
    m.insert("Current scanning", "Aktualnie skanowane");
    m.insert("Upgrading filesystem cache...", "Uaktualnianie plików pamięci podręcznej...");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

pub struct PluralFormHandler;

impl PluralFormHandler {
    pub fn get_plural_form(&self, count: i64) -> usize {
        if count == 1 {
            0
        } else if count % 10 >= 2 && count % 10 <= 4 && (count % 100 < 10 || count % 100 >= 20) {
            1
        } else {
            2
        }
    }

    pub fn get_plural_string(&self, singular: &str, plural: &str, count: i64) -> String {
        match self.get_plural_form(count) {
            0 => singular.replace("%n", &count.to_string()),
            1 => plural.replace("%n", &count.to_string()),
            _ => {
                let forms = match singular {
                    "_%n folder_::_%n folders_" => vec!["%n katalog", "%n katalogi", "%n katalogów"],
                    "_%n file_::_%n files_" => vec!["%n plik", "%n pliki", "%n plików"],
                    "_Uploading %n file_::_Uploading %n files_" => vec!["Wysyłanie %n pliku", "Wysyłanie %n plików", "Wysyłanie %n plików"],
                    _ => vec![plural],
                };
                
                if let Some(form) = forms.get(2) {
                    form.replace("%n", &count.to_string())
                } else {
                    plural.replace("%n", &count.to_string())
                }
            }
        }
    }

    pub fn format_dirs_and_files(&self, dirs: i64, files: i64) -> String {
        let dirs_str = self.get_plural_string("_%n folder_::_%n folders_", "_%n folders_", dirs);
        let files_str = self.get_plural_string("_%n file_::_%n files_", "_%n files_", files);
        
        "{dirs} and {files}"
            .replace("{dirs}", &dirs_str)
            .replace("{files}", &files_str)
            .replace("{katalogi}", &dirs_str)
            .replace("{pliki}", &files_str)
    }
}