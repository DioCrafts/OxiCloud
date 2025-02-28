use once_cell::sync::Lazy;
use rust_i18n::t;
use std::collections::HashMap;

#[derive(Clone)]
pub struct LtLt;

impl LtLt {
    pub fn translations() -> &'static HashMap<&'static str, &'static str> {
        static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
            let mut map = HashMap::new();
            map.insert("Could not move %s - File with this name already exists", "Nepavyko perkelti %s - failas su tokiu pavadinimu jau egzistuoja");
            map.insert("Could not move %s", "Nepavyko perkelti %s");
            map.insert("File name cannot be empty.", "Failo pavadinimas negali būti tuščias.");
            map.insert("File name must not contain \"/\". Please choose a different name.", "Failo pavadinime negali būti simbolio \"/\". Prašome pasirinkti kitokį pavadinimą.");
            map.insert("Error while downloading %s to %s", "Klaida siunčiant %s į %s");
            map.insert("Error when creating the file", "Klaida kuriant failą");
            map.insert("Folder name cannot be empty.", "Aplanko pavadinimas negali būti tuščias.");
            map.insert("Folder name must not contain \"/\". Please choose a different name.", "Aplanko pavadinime negali būti simbolio \"/\". Prašome pasirinkti kitokį pavadinimą.");
            map.insert("Error when creating the folder", "Klaida kuriant aplanką");
            map.insert("Unable to set upload directory.", "Nepavyksta nustatyti įkėlimų katalogo.");
            map.insert("Invalid Token", "Netinkamas ženklas");
            map.insert("No file was uploaded. Unknown error", "Failai nebuvo įkelti dėl nežinomos priežasties");
            map.insert("There is no error, the file uploaded with success", "Failas įkeltas sėkmingai, be klaidų");
            map.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Įkeliamas failas yra didesnis nei leidžia upload_max_filesize php.ini faile:");
            map.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Įkeliamo failo dydis viršija MAX_FILE_SIZE nustatymą, kuris naudojamas HTML formoje.");
            map.insert("The uploaded file was only partially uploaded", "Failas buvo įkeltas tik dalinai");
            map.insert("No file was uploaded", "Nebuvo įkeltas joks failas");
            map.insert("Missing a temporary folder", "Nėra laikinojo katalogo");
            map.insert("Failed to write to disk", "Nepavyko įrašyti į diską");
            map.insert("Not enough storage available", "Nepakanka vietos serveryje");
            map.insert("Upload failed. Could not get file info.", "Įkėlimas nepavyko. Nepavyko gauti failo informacijos.");
            map.insert("Upload failed. Could not find uploaded file", "Įkėlimas nepavyko. Nepavyko rasti įkelto failo");
            map.insert("Invalid directory.", "Neteisingas aplankas");
            map.insert("Files", "Failai");
            map.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Nepavyksta įkelti {filename}, nes tai katalogas arba yra 0 baitų dydžio");
            map.insert("Not enough space available", "Nepakanka vietos");
            map.insert("Upload cancelled.", "Įkėlimas atšauktas.");
            map.insert("Could not get result from server.", "Nepavyko gauti rezultato iš serverio.");
            map.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Failo įkėlimas pradėtas. Jei paliksite šį puslapį, įkėlimas nutrūks.");
            map.insert("URL cannot be empty", "URL negali būti tuščias.");
            map.insert("{new_name} already exists", "{new_name} jau egzistuoja");
            map.insert("Could not create file", "Neįmanoma sukurti failo");
            map.insert("Could not create folder", "Neįmanoma sukurti aplanko");
            map.insert("Share", "Dalintis");
            map.insert("Delete permanently", "Ištrinti negrįžtamai");
            map.insert("Rename", "Pervadinti");
            map.insert("Pending", "Laukiantis");
            map.insert("Could not rename file", "Neįmanoma pervadinti failo");
            map.insert("replaced {new_name} with {old_name}", "pakeiskite {new_name} į {old_name}");
            map.insert("undo", "anuliuoti");
            map.insert("{dirs} and {files}", "{dirs} ir {files}");
            map.insert("'.' is an invalid file name.", "'.' yra neleidžiamas failo pavadinime.");
            map.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Neleistinas pavadinimas, '\\', '/', '<', '>', ':', '\"', '|', '?' ir '*' yra neleidžiami.");
            map.insert("Your storage is full, files can not be updated or synced anymore!", "Jūsų visa vieta serveryje užimta");
            map.insert("Your storage is almost full ({usedSpacePercent}%)", "Jūsų vieta serveryje beveik visa užimta ({usedSpacePercent}%)");
            map.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "Šifravimo programa įjungta, bet Jūsų raktai nėra pritaikyti. Prašome atsijungti ir vėl prisijungti");
            map.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Netinkamas privatus raktas Šifravimo programai. Prašome atnaujinti savo privataus rakto slaptažodį asmeniniuose nustatymuose, kad atkurti prieigą prie šifruotų failų.");
            map.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Šifravimas buvo išjungtas, bet Jūsų failai vis dar užšifruoti. Prašome eiti į asmeninius nustatymus ir iššifruoti savo failus.");
            map.insert("Your download is being prepared. This might take some time if the files are big.", "Jūsų atsisiuntimas yra paruošiamas. tai gali užtrukti jei atsisiunčiamas didelis failas.");
            map.insert("Error moving file", "Klaida perkeliant failą");
            map.insert("Error", "Klaida");
            map.insert("Name", "Pavadinimas");
            map.insert("Size", "Dydis");
            map.insert("Modified", "Pakeista");
            map.insert("%s could not be renamed", "%s negali būti pervadintas");
            map.insert("Upload", "Įkelti");
            map.insert("File handling", "Failų tvarkymas");
            map.insert("Maximum upload size", "Maksimalus įkeliamo failo dydis");
            map.insert("max. possible: ", "maks. galima:");
            map.insert("Needed for multi-file and folder downloads.", "Reikalinga daugybinui failų ir aplankalų atsisiuntimui.");
            map.insert("Enable ZIP-download", "Įjungti atsisiuntimą ZIP archyvu");
            map.insert("0 is unlimited", "0 yra neribotas");
            map.insert("Maximum input size for ZIP files", "Maksimalus ZIP archyvo failo dydis");
            map.insert("Save", "Išsaugoti");
            map.insert("New", "Naujas");
            map.insert("Text file", "Teksto failas");
            map.insert("Folder", "Katalogas");
            map.insert("From link", "Iš nuorodos");
            map.insert("Deleted files", "Ištrinti failai");
            map.insert("Cancel upload", "Atšaukti siuntimą");
            map.insert("You don't have permission to upload or create files here", "Jūs neturite leidimo čia įkelti arba kurti failus");
            map.insert("Nothing in here. Upload something!", "Čia tuščia. Įkelkite ką nors!");
            map.insert("Download", "Atsisiųsti");
            map.insert("Unshare", "Nebesidalinti");
            map.insert("Delete", "Ištrinti");
            map.insert("Upload too large", "Įkėlimui failas per didelis");
            map.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Bandomų įkelti failų dydis viršija maksimalų, kuris leidžiamas šiame serveryje");
            map.insert("Files are being scanned, please wait.", "Skenuojami failai, prašome palaukti.");
            map.insert("Current scanning", "Šiuo metu skenuojama");
            map.insert("Upgrading filesystem cache...", "Atnaujinamas sistemos kešavimas...");
            map
        });
        &TRANSLATIONS
    }

    pub fn plurals() -> &'static [(&'static str, &'static [&'static str])] {
        static PLURALS: Lazy<Vec<(&'static str, &'static [&'static str])>> = Lazy::new(|| {
            vec![
                ("_%n folder_::_%n folders_", &["%n aplankas", "%n aplankai", "%n aplankų"]),
                ("_%n file_::_%n files_", &["%n failas", "%n failai", "%n failų"]),
                ("_Uploading %n file_::_Uploading %n files_", &["Įkeliamas %n failas", "Įkeliami %n failai", "Įkeliama %n failų"]),
            ]
        });
        &PLURALS
    }

    pub fn plural_form(n: usize) -> usize {
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n % 10 >= 2 && (n % 100 < 10 || n % 100 >= 20) {
            1
        } else {
            2
        }
    }

    pub fn get_plural(key: &str, n: usize) -> String {
        for (plural_key, forms) in Self::plurals() {
            if key == *plural_key {
                let form_index = Self::plural_form(n);
                if form_index < forms.len() {
                    return forms[form_index].replace("%n", &n.to_string());
                }
                return forms[0].replace("%n", &n.to_string());
            }
        }
        key.to_string()
    }

    pub fn get(key: &str) -> String {
        match Self::translations().get(key) {
            Some(translation) => translation.to_string(),
            None => key.to_string(),
        }
    }
}

// Implementación de trait para el sistema de traducción
pub trait Translatable {
    fn translate(&self) -> String;
    fn translate_args(&self, args: &[&str]) -> String;
}

impl Translatable for str {
    fn translate(&self) -> String {
        LtLt::get(self)
    }

    fn translate_args(&self, args: &[&str]) -> String {
        let mut result = self.translate();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("%{}", i + 1), arg);
            result = result.replace("%s", arg);  // Para compatibilidad con formato %s
        }
        result
    }
}