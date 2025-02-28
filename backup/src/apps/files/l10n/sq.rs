use std::collections::HashMap;
use rust_i18n::i18n;

#[derive(Default)]
pub struct Translations {
    translations: HashMap<String, String>,
    plural_forms: String,
}

impl Translations {
    pub fn new() -> Self {
        let mut sq = Self {
            translations: HashMap::new(),
            plural_forms: "nplurals=2; plural=(n != 1);".to_string(),
        };
        
        sq.translations.insert("Could not move %s - File with this name already exists".to_string(), "E pa mundur zhvendosja e %s - ekziston nje skedar me te njetin emer".to_string());
        sq.translations.insert("Could not move %s".to_string(), "Nuk mund të zhvendoset %s".to_string());
        sq.translations.insert("File name cannot be empty.".to_string(), "Emri i skedarit nuk mund të jetë bosh.".to_string());
        sq.translations.insert("Unable to set upload directory.".to_string(), "E pa mundur të vendoset dosja e ngarkimit".to_string());
        sq.translations.insert("Invalid Token".to_string(), "Shenjë e gabuar".to_string());
        sq.translations.insert("No file was uploaded. Unknown error".to_string(), "Asnjë skedar nuk u dërgua. Gabim i pa njohur".to_string());
        sq.translations.insert("There is no error, the file uploaded with success".to_string(), "Skedari u ngarkua me sukses".to_string());
        sq.translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(), "Skedari i ngarkuar tejkalon limitin hapsirës së lejuar në php.ini".to_string());
        sq.translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(), "Skedari i ngarkuar tejlakon vlerën MAX_FILE_SIZE të përcaktuar në formën HTML".to_string());
        sq.translations.insert("The uploaded file was only partially uploaded".to_string(), "Skedari është ngakruar vetëm pjesërisht".to_string());
        sq.translations.insert("No file was uploaded".to_string(), "Asnjë skedar nuk është ngarkuar".to_string());
        sq.translations.insert("Missing a temporary folder".to_string(), "Mungon dosja e përkohshme".to_string());
        sq.translations.insert("Failed to write to disk".to_string(), "Dështoi shkrimi në disk".to_string());
        sq.translations.insert("Not enough storage available".to_string(), "Hapsira e arkivimit e pamjaftueshme".to_string());
        sq.translations.insert("Invalid directory.".to_string(), "Dosje e pavlefshme".to_string());
        sq.translations.insert("Files".to_string(), "Skedarë".to_string());
        sq.translations.insert("Not enough space available".to_string(), "Nuk ka hapsirë të nevojshme".to_string());
        sq.translations.insert("Upload cancelled.".to_string(), "Ngarkimi u anullua".to_string());
        sq.translations.insert("File upload is in progress. Leaving the page now will cancel the upload.".to_string(), "Skedari duke u ngarkuar. Largimi nga faqja do të anullojë ngarkimin".to_string());
        sq.translations.insert("{new_name} already exists".to_string(), "{new_name} është ekzistues ".to_string());
        sq.translations.insert("Share".to_string(), "Ndaj".to_string());
        sq.translations.insert("Delete permanently".to_string(), "Fshi përfundimisht".to_string());
        sq.translations.insert("Rename".to_string(), "Riemëro".to_string());
        sq.translations.insert("Pending".to_string(), "Në vijim".to_string());
        sq.translations.insert("replaced {new_name} with {old_name}".to_string(), "u zëvendësua {new_name} me {old_name}".to_string());
        sq.translations.insert("undo".to_string(), "anullo".to_string());
        sq.translations.insert("_%n folder_::_%n folders_".to_string(), "%n dosje|%n dosje".to_string());
        sq.translations.insert("_%n file_::_%n files_".to_string(), "%n skedar|%n skedarë".to_string());
        sq.translations.insert("{dirs} and {files}".to_string(), "{dirs} dhe {files}".to_string());
        sq.translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), "Po ngarkoj %n skedar|Po ngarkoj %n skedarë".to_string());
        sq.translations.insert("'.' is an invalid file name.".to_string(), "'.' nuk është skedar i vlefshem.".to_string());
        sq.translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(), "Emër jo i vlefshëm, '\\', '/', '<', '>', ':', '\"', '|', '?' dhe '*' nuk lejohen.".to_string());
        sq.translations.insert("Your storage is full, files can not be updated or synced anymore!".to_string(), "Hapsira juaj e arkivimit është plot, skedarët nuk mund të përditësohen ose sinkronizohen!".to_string());
        sq.translations.insert("Your storage is almost full ({usedSpacePercent}%)".to_string(), "Hapsira juaj e arkivimit është pothuajse në fund  ({usedSpacePercent}%)".to_string());
        sq.translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.".to_string(), "Kodifikimi u çaktivizua por skedarët tuaj vazhdojnë të jenë të kodifikuar. Ju lutem shkoni tek parametrat personale për të dekodifikuar skedarët tuaj.".to_string());
        sq.translations.insert("Your download is being prepared. This might take some time if the files are big.".to_string(), "Shkarkimi juaj është duke u përgatitur. Kjo mund të kërkojë kohë nëse skedarët janë të mëdhenj.".to_string());
        sq.translations.insert("Error".to_string(), "Gabim".to_string());
        sq.translations.insert("Name".to_string(), "Emri".to_string());
        sq.translations.insert("Size".to_string(), "Madhësia".to_string());
        sq.translations.insert("Modified".to_string(), "Ndryshuar".to_string());
        sq.translations.insert("%s could not be renamed".to_string(), "Nuk është i mundur riemërtimi i %s".to_string());
        sq.translations.insert("Upload".to_string(), "Ngarko".to_string());
        sq.translations.insert("File handling".to_string(), "Trajtimi i Skedarëve".to_string());
        sq.translations.insert("Maximum upload size".to_string(), "Madhësia maksimale e nagarkimit".to_string());
        sq.translations.insert("max. possible: ".to_string(), "maks i mundshëm".to_string());
        sq.translations.insert("Needed for multi-file and folder downloads.".to_string(), "Nevojitej shkarkim i shumë skedarëve dhe dosjeve".to_string());
        sq.translations.insert("Enable ZIP-download".to_string(), "Mundëso skarkimin e ZIP".to_string());
        sq.translations.insert("0 is unlimited".to_string(), "o është pa limit".to_string());
        sq.translations.insert("Maximum input size for ZIP files".to_string(), "Maksimumi hyrës i skedarëve ZIP".to_string());
        sq.translations.insert("Save".to_string(), "Ruaj".to_string());
        sq.translations.insert("New".to_string(), "E re".to_string());
        sq.translations.insert("Text file".to_string(), "Skedar tekst".to_string());
        sq.translations.insert("Folder".to_string(), "Dosje".to_string());
        sq.translations.insert("From link".to_string(), "Nga lidhja".to_string());
        sq.translations.insert("Deleted files".to_string(), "Skedarë të fshirë ".to_string());
        sq.translations.insert("Cancel upload".to_string(), "Anullo ngarkimin".to_string());
        sq.translations.insert("Nothing in here. Upload something!".to_string(), "Këtu nuk ka asgje. Ngarko dicka".to_string());
        sq.translations.insert("Download".to_string(), "Shkarko".to_string());
        sq.translations.insert("Unshare".to_string(), "Hiq ndarjen".to_string());
        sq.translations.insert("Delete".to_string(), "Fshi".to_string());
        sq.translations.insert("Upload too large".to_string(), "Ngarkimi shumë i madh".to_string());
        sq.translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(), "Skedarët që po mundoheni të ngarkoni e tejkalojnë madhësinë maksimale të lejuar nga serveri.".to_string());
        sq.translations.insert("Files are being scanned, please wait.".to_string(), "Skanerizimi i skedarit në proces. Ju lutem prisni.".to_string());
        sq.translations.insert("Current scanning".to_string(), "Skanimi aktual".to_string());
        sq.translations.insert("Upgrading filesystem cache...".to_string(), "Përditësimi i cache-se së sistemit në procesim...".to_string());

        sq
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }

    pub fn get_plural_forms(&self) -> &str {
        &self.plural_forms
    }

    pub fn get_translation(&self, key: &str, params: &[&str]) -> String {
        match self.translations.get(key) {
            Some(translation) => {
                let mut result = translation.clone();
                for (i, param) in params.iter().enumerate() {
                    result = result.replace(&format!("%s"), param);
                    result = result.replace(&format!("%{}", i + 1), param);
                }
                result
            }
            None => key.to_string(),
        }
    }
}

// Register the translations module
i18n!("apps/files/l10n/");