// fi_FI.rs

use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Kohteen %s siirto ei onnistunut - Tiedosto samalla nimellä on jo olemassa");
        m.insert("Could not move %s", "Kohteen %s siirto ei onnistunut");
        m.insert("File name cannot be empty.", "Tiedoston nimi ei voi olla tyhjä.");
        m.insert("File name must not contain \"/\". Please choose a different name.", "Tiedoston nimessä ei saa olla merkkiä \"/\". Valitse toinen nimi.");
        m.insert("The name %s is already used in the folder %s. Please choose a different name.", "Nimi %s on jo käytössä kansiossa %s. Valitse toinen nimi.");
        m.insert("Not a valid source", "Virheellinen lähde");
        m.insert("Error when creating the file", "Virhe tiedostoa luotaessa");
        m.insert("Folder name cannot be empty.", "Kansion nimi ei voi olla tyhjä.");
        m.insert("Folder name must not contain \"/\". Please choose a different name.", "Kansion nimessä ei saa olla merkkiä \"/\". Valitse toinen nimi.");
        m.insert("Error when creating the folder", "Virhe kansiota luotaessa");
        m.insert("No file was uploaded. Unknown error", "Tiedostoa ei lähetetty. Tuntematon virhe");
        m.insert("There is no error, the file uploaded with success", "Ei virheitä, tiedosto lähetettiin onnistuneesti");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Lähetetyn tiedoston koko ylittää php.ini-tiedoston upload_max_filesize-säännön:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Ladattavan tiedoston maksimikoko ylittää MAX_FILE_SIZE dirketiivin, joka on määritelty HTML-lomakkeessa");
        m.insert("The uploaded file was only partially uploaded", "Tiedoston lähetys onnistui vain osittain");
        m.insert("No file was uploaded", "Yhtäkään tiedostoa ei lähetetty");
        m.insert("Missing a temporary folder", "Tilapäiskansio puuttuu");
        m.insert("Failed to write to disk", "Levylle kirjoitus epäonnistui");
        m.insert("Not enough storage available", "Tallennustilaa ei ole riittävästi käytettävissä");
        m.insert("Invalid directory.", "Virheellinen kansio.");
        m.insert("Files", "Tiedostot");
        m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Kohdetta {filename} ei voi lähettää, koska se on joko kansio tai sen koko on 0 tavua");
        m.insert("Not enough space available", "Tilaa ei ole riittävästi");
        m.insert("Upload cancelled.", "Lähetys peruttu.");
        m.insert("Could not get result from server.", "Tuloksien saaminen palvelimelta ei onnistunut.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Tiedoston lähetys on meneillään. Sivulta poistuminen nyt peruu tiedoston lähetyksen.");
        m.insert("URL cannot be empty", "Osoite ei voi olla tyhjä");
        m.insert("{new_name} already exists", "{new_name} on jo olemassa");
        m.insert("Could not create file", "Tiedoston luominen epäonnistui");
        m.insert("Could not create folder", "Kansion luominen epäonnistui");
        m.insert("Share", "Jaa");
        m.insert("Delete permanently", "Poista pysyvästi");
        m.insert("Rename", "Nimeä uudelleen");
        m.insert("Pending", "Odottaa");
        m.insert("Could not rename file", "Tiedoston nimeäminen uudelleen epäonnistui");
        m.insert("undo", "kumoa");
        m.insert("{dirs} and {files}", "{dirs} ja {files}");
        m.insert("'.' is an invalid file name.", "'.' on virheellinen nimi tiedostolle.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Virheellinen nimi, merkit '\\', '/', '<', '>', ':', '\"', '|', '?' ja '*' eivät ole sallittuja.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Tallennustila on loppu, tiedostoja ei voi enää päivittää tai synkronoida!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Tallennustila on melkein loppu ({usedSpacePercent}%)");
        m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Salaus poistettiin käytöstä, mutta tiedostosi ovat edelleen salattu. Siirry henkilökohtaisiin asetuksiisi avataksesi tiedostojesi salauksen.");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Lataustasi valmistellaan. Tämä saattaa kestää hetken, jos tiedostot ovat suuria kooltaan.");
        m.insert("Error moving file", "Virhe tiedostoa siirrettäessä");
        m.insert("Error", "Virhe");
        m.insert("Name", "Nimi");
        m.insert("Size", "Koko");
        m.insert("Modified", "Muokattu");
        m.insert("Upload", "Lähetä");
        m.insert("File handling", "Tiedostonhallinta");
        m.insert("Maximum upload size", "Lähetettävän tiedoston suurin sallittu koko");
        m.insert("max. possible: ", "suurin mahdollinen:");
        m.insert("Needed for multi-file and folder downloads.", "Tarvitaan useampien tiedostojen ja kansioiden latausta varten.");
        m.insert("Enable ZIP-download", "Ota ZIP-paketin lataaminen käytöön");
        m.insert("0 is unlimited", "0 on rajoittamaton");
        m.insert("Maximum input size for ZIP files", "ZIP-tiedostojen enimmäiskoko");
        m.insert("Save", "Tallenna");
        m.insert("New", "Uusi");
        m.insert("Text file", "Tekstitiedosto");
        m.insert("Folder", "Kansio");
        m.insert("From link", "Linkistä");
        m.insert("Deleted files", "Poistetut tiedostot");
        m.insert("Cancel upload", "Peru lähetys");
        m.insert("You don't have permission to upload or create files here", "Käyttöoikeutesi eivät riitä tiedostojen lähettämiseen tai kansioiden luomiseen tähän sijaintiin");
        m.insert("Nothing in here. Upload something!", "Täällä ei ole mitään. Lähetä tänne jotakin!");
        m.insert("Download", "Lataa");
        m.insert("Unshare", "Peru jakaminen");
        m.insert("Delete", "Poista");
        m.insert("Upload too large", "Lähetettävä tiedosto on liian suuri");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Lähetettäväksi valitsemasi tiedostot ylittävät palvelimen salliman tiedostokoon rajan.");
        m.insert("Files are being scanned, please wait.", "Tiedostoja tarkistetaan, odota hetki.");
        m.insert("Current scanning", "Tämänhetkinen tutkinta");
        m.insert("Upgrading filesystem cache...", "Päivitetään tiedostojärjestelmän välimuistia...");
        m
    };
    
    pub static ref PLURAL_FORMS: PluralForms = PluralForms {
        formula: |n| if n != 1 { 1 } else { 0 },
        forms: vec![
            // n=1 form
            {
                let mut m = HashMap::new();
                m.insert("_%n folder_::_%n folders_", "%n kansio");
                m.insert("_%n file_::_%n files_", "%n tiedosto");
                m.insert("_Uploading %n file_::_Uploading %n files_", "Lähetetään %n tiedosto");
                m
            },
            // n!=1 form
            {
                let mut m = HashMap::new();
                m.insert("_%n folder_::_%n folders_", "%n kansiota");
                m.insert("_%n file_::_%n files_", "%n tiedostoa");
                m.insert("_Uploading %n file_::_Uploading %n files_", "Lähetetään %n tiedostoa");
                m
            }
        ]
    };
}

pub struct PluralForms {
    formula: fn(n: i64) -> usize,
    forms: Vec<HashMap<&'static str, &'static str>>,
}

impl PluralForms {
    pub fn get_form(&self, key: &str, n: i64) -> Option<&str> {
        let form_index = (self.formula)(n);
        if form_index < self.forms.len() {
            self.forms[form_index].get(key).copied()
        } else {
            None
        }
    }
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, n: i64) -> &'static str {
    PLURAL_FORMS.get_form(key, n).unwrap_or(key)
}