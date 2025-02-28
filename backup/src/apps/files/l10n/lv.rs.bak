use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::locale::LocaleInfo;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Nevarēja pārvietot %s — jau eksistē datne ar tādu nosaukumu");
        m.insert("Could not move %s", "Nevarēja pārvietot %s");
        m.insert("File name cannot be empty.", "Datnes nosaukums nevar būt tukšs.");
        m.insert("Unable to set upload directory.", "Nevar uzstādīt augšupielādes mapi.");
        m.insert("Invalid Token", "Nepareiza pilnvara");
        m.insert("No file was uploaded. Unknown error", "Netika augšupielādēta neviena datne. Nezināma kļūda");
        m.insert("There is no error, the file uploaded with success", "Viss kārtībā, datne augšupielādēta veiksmīga");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Augšupielādētā datne pārsniedz upload_max_filesize norādījumu php.ini datnē:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Augšupielādētā datne pārsniedz MAX_FILE_SIZE norādi, kas ir norādīta HTML formā");
        m.insert("The uploaded file was only partially uploaded", "Augšupielādētā datne ir tikai daļēji augšupielādēta");
        m.insert("No file was uploaded", "Neviena datne netika augšupielādēta");
        m.insert("Missing a temporary folder", "Trūkst pagaidu mapes");
        m.insert("Failed to write to disk", "Neizdevās saglabāt diskā");
        m.insert("Not enough storage available", "Nav pietiekami daudz vietas");
        m.insert("Invalid directory.", "Nederīga direktorija.");
        m.insert("Files", "Datnes");
        m.insert("Not enough space available", "Nepietiek brīvas vietas");
        m.insert("Upload cancelled.", "Augšupielāde ir atcelta.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Notiek augšupielāde. Pametot lapu tagad, tiks atcelta augšupielāde.");
        m.insert("{new_name} already exists", "{new_name} jau eksistē");
        m.insert("Share", "Dalīties");
        m.insert("Delete permanently", "Dzēst pavisam");
        m.insert("Rename", "Pārsaukt");
        m.insert("Pending", "Gaida savu kārtu");
        m.insert("replaced {new_name} with {old_name}", "aizvietoja {new_name} ar {old_name}");
        m.insert("undo", "atsaukt");
        m.insert("'.' is an invalid file name.", "'.' ir nederīgs datnes nosaukums.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nederīgs nosaukums, nav atļauti '\\', '/', '<', '>', ':', '\"', '|', '?' un '*'.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Jūsu krātuve ir pilna, datnes vairs nevar augšupielādēt vai sinhronizēt!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Jūsu krātuve ir gandrīz pilna ({usedSpacePercent}%)");
        m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Šifrēšana tika atslēgta, tomēr jūsu faili joprojām ir šifrēti. Atšifrēt failus var Personiskajos uzstādījumos.");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Tiek sagatavota lejupielāde. Tas var aizņemt kādu laiciņu, ja datnes ir lielas.");
        m.insert("Error", "Kļūda");
        m.insert("Name", "Nosaukums");
        m.insert("Size", "Izmērs");
        m.insert("Modified", "Mainīts");
        m.insert("%s could not be renamed", "%s nevar tikt pārsaukts");
        m.insert("Upload", "Augšupielādēt");
        m.insert("File handling", "Datņu pārvaldība");
        m.insert("Maximum upload size", "Maksimālais datņu augšupielādes apjoms");
        m.insert("max. possible: ", "maksimālais iespējamais:");
        m.insert("Needed for multi-file and folder downloads.", "Vajadzīgs vairāku datņu un mapju lejupielādēšanai.");
        m.insert("Enable ZIP-download", "Aktivēt ZIP lejupielādi");
        m.insert("0 is unlimited", "0 ir neierobežots");
        m.insert("Maximum input size for ZIP files", "Maksimālais ievades izmērs ZIP datnēm");
        m.insert("Save", "Saglabāt");
        m.insert("New", "Jauna");
        m.insert("Text file", "Teksta datne");
        m.insert("Folder", "Mape");
        m.insert("From link", "No saites");
        m.insert("Deleted files", "Dzēstās datnes");
        m.insert("Cancel upload", "Atcelt augšupielādi");
        m.insert("Nothing in here. Upload something!", "Te vēl nekas nav. Rīkojies, sāc augšupielādēt!");
        m.insert("Download", "Lejupielādēt");
        m.insert("Unshare", "Pārtraukt dalīšanos");
        m.insert("Delete", "Dzēst");
        m.insert("Upload too large", "Datne ir par lielu, lai to augšupielādētu");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Augšupielādējamās datnes pārsniedz servera pieļaujamo datņu augšupielādes apjomu");
        m.insert("Files are being scanned, please wait.", "Datnes šobrīd tiek caurskatītas, lūdzu, uzgaidiet.");
        m.insert("Current scanning", "Šobrīd tiek caurskatīts");
        m.insert("Upgrading filesystem cache...", "Uzlabo datņu sistēmas kešatmiņu...");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec!["%n mapes", "%n mape", "%n mapes"]);
        m.insert("_%n file_::_%n files_", vec!["%n faili", "%n fails", "%n faili"]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec!["%n", "Augšupielāde %n failu", "Augšupielāde %n failus"]);
        m
    };
}

pub struct Lv;

impl Lv {
    pub fn get_translation(key: &str) -> Option<&'static str> {
        TRANSLATIONS.get(key).copied()
    }

    pub fn get_plural_form(key: &str, n: usize) -> Option<&'static str> {
        let forms = PLURAL_FORMS.get(key)?;
        
        // nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n != 0 ? 1 : 2);
        let idx = if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n != 0 {
            1
        } else {
            2
        };
        
        forms.get(idx).copied()
    }
}

impl LocaleInfo for Lv {
    fn locale_name() -> &'static str {
        "lv"
    }
    
    fn plural_forms() -> &'static str {
        "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n != 0 ? 1 : 2);"
    }
}