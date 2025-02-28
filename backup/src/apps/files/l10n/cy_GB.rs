use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::I18n;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Methwyd symud %s - Mae ffeil gyda'r enw hwn eisoes yn bodoli");
        m.insert("Could not move %s", "Methwyd symud %s");
        m.insert("File name cannot be empty.", "Does dim hawl cael enw ffeil gwag.");
        m.insert("No file was uploaded. Unknown error", "Ni lwythwyd ffeil i fyny. Gwall anhysbys.");
        m.insert("There is no error, the file uploaded with success", "Does dim gwall, llwythodd y ffeil i fyny'n llwyddiannus");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Mae'r ffeil lwythwyd i fyny'n fwy na chyfarwyddeb upload_max_filesize yn php.ini:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Mae'r ffeil lwythwyd i fyny'n fwy na chyfarwyddeb MAX_FILE_SIZE bennwyd yn y ffurflen HTML");
        m.insert("The uploaded file was only partially uploaded", "Dim ond yn rhannol y llwythwyd y ffeil i fyny");
        m.insert("No file was uploaded", "Ni lwythwyd ffeil i fyny");
        m.insert("Missing a temporary folder", "Plygell dros dro yn eisiau");
        m.insert("Failed to write to disk", "Methwyd ysgrifennu i'r ddisg");
        m.insert("Not enough storage available", "Dim digon o le storio ar gael");
        m.insert("Invalid directory.", "Cyfeiriadur annilys.");
        m.insert("Files", "Ffeiliau");
        m.insert("Not enough space available", "Dim digon o le ar gael");
        m.insert("Upload cancelled.", "Diddymwyd llwytho i fyny.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Mae ffeiliau'n cael eu llwytho i fyny. Bydd gadael y dudalen hon nawr yn diddymu'r broses.");
        m.insert("{new_name} already exists", "{new_name} yn bodoli'n barod");
        m.insert("Share", "Rhannu");
        m.insert("Delete permanently", "Dileu'n barhaol");
        m.insert("Rename", "Ailenwi");
        m.insert("Pending", "I ddod");
        m.insert("replaced {new_name} with {old_name}", "newidiwyd {new_name} yn lle {old_name}");
        m.insert("undo", "dadwneud");
        m.insert("_%n folder_::_%n folders_", "");
        m.insert("_%n file_::_%n files_", "");
        m.insert("_Uploading %n file_::_Uploading %n files_", "");
        m.insert("'.' is an invalid file name.", "Mae '.' yn enw ffeil annilys.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Enw annilys, ni chaniateir, '\\', '/', '<', '>', ':', '\"', '|', '?' na '*'.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Mae eich storfa'n llawn, ni ellir diweddaru a chydweddu ffeiliau mwyach!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Mae eich storfa bron a bod yn llawn ({usedSpacePercent}%)");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Wrthi'n paratoi i lwytho i lawr. Gall gymryd peth amser os yw'r ffeiliau'n fawr.");
        m.insert("Error", "Gwall");
        m.insert("Name", "Enw");
        m.insert("Size", "Maint");
        m.insert("Modified", "Addaswyd");
        m.insert("Upload", "Llwytho i fyny");
        m.insert("File handling", "Trafod ffeiliau");
        m.insert("Maximum upload size", "Maint mwyaf llwytho i fyny");
        m.insert("max. possible: ", "mwyaf. posib:");
        m.insert("Needed for multi-file and folder downloads.", "Angen ar gyfer llwytho mwy nag un ffeil neu blygell i lawr yr un pryd.");
        m.insert("Enable ZIP-download", "Galluogi llwytho i lawr ZIP");
        m.insert("0 is unlimited", "0 yn ddiderfyn");
        m.insert("Maximum input size for ZIP files", "Maint mewnbynnu mwyaf ffeiliau ZIP");
        m.insert("Save", "Cadw");
        m.insert("New", "Newydd");
        m.insert("Text file", "Ffeil destun");
        m.insert("Folder", "Plygell");
        m.insert("From link", "Dolen o");
        m.insert("Deleted files", "Ffeiliau ddilewyd");
        m.insert("Cancel upload", "Diddymu llwytho i fyny");
        m.insert("Nothing in here. Upload something!", "Does dim byd fan hyn. Llwythwch rhywbeth i fyny!");
        m.insert("Download", "Llwytho i lawr");
        m.insert("Unshare", "Dad-rannu");
        m.insert("Delete", "Dileu");
        m.insert("Upload too large", "Maint llwytho i fyny'n rhy fawr");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Mae'r ffeiliau rydych yn ceisio llwytho i fyny'n fwy na maint mwyaf llwytho ffeiliau i fyny ar y gweinydd hwn.");
        m.insert("Files are being scanned, please wait.", "Arhoswch, mae ffeiliau'n cael eu sganio.");
        m.insert("Current scanning", "Sganio cyfredol");
        m.insert("Upgrading filesystem cache...", "Uwchraddio storfa system ffeiliau...");
        m
    };

    // Mapa para manejar los plurales
    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec!["", "", "", ""]);
        m.insert("_%n file_::_%n files_", vec!["", "", "", ""]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec!["", "", "", ""]);
        m
    };
}

pub fn get_plural_form(count: i64) -> usize {
    if count == 1 {
        0
    } else if count == 2 {
        1
    } else if count != 8 && count != 11 {
        2
    } else {
        3
    }
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn translate_plural(key: &str, count: i64) -> &'static str {
    if let Some(forms) = PLURAL_FORMS.get(key) {
        let index = get_plural_form(count);
        if index < forms.len() {
            return forms[index];
        }
    }
    key
}

pub fn format_translation(key: &str, args: &[&str]) -> String {
    let mut result = translate(key).to_string();
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("%{}", i + 1), arg);
        // Compatibilidad con %s
        if i == 0 {
            result = result.replace("%s", arg);
        }
    }
    result
}