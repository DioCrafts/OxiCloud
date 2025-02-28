use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Taastevõtme lubamine õnnestus");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Ei suutnud lubada taastevõtit. Palun kontrolli oma taastevõtme parooli!");
        m.insert("Recovery key successfully disabled", "Taastevõtme keelamine õnnestus");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Ei suuda keelata taastevõtit. Palun kontrolli oma taastevõtme parooli!");
        m.insert("Password successfully changed.", "Parool edukalt vahetatud.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Ei suutnud vahetada parooli. Võib-olla on vana parool valesti sisestatud.");
        m.insert("Private key password successfully updated.", "Privaatse võtme parool edukalt uuendatud.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Ei suutnud uuendada privaatse võtme parooli. Võib-olla polnud vana parool õige.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Krüpteerimise rakend pole käivitatud. Võib-olla krüpteerimise rakend taaskäivitati sinu sessiooni kestel. Palun proovi logida välja ning uuesti sisse käivitamaks krüpteerimise rakendit.");
        m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "Sinu provaatne võti pole kehtiv! Tõenäoliselt mudueti parooli väljaspool kausta %s (nt. sinu ettevõtte kaust). Sa saad uuendada oma privaatse võtme parooli oma isiklikes seadetes, et taastada ligipääs sinu krüpteeritud failidele.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Sa ei saa seda faili dekrüpteerida, see on tõenäoliselt jagatud fail. Palun lase omanikul seda faili sinuga uuesti jagada.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Tundmatu tõrge. Palun kontrolli süsteemi seadeid või võta ühendust oma süsteemi administraatoriga");
        m.insert("Missing requirements.", "Nõutavad on puudu.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Palun veendu, et on paigaldatud PHP 5.3.3 või uuem ning PHP OpenSSL laiendus on lubatud ning seadistatud korrektselt. Hetkel krüpteerimise rakendus on peatatud.");
        m.insert("Following users are not set up for encryption:", "Järgmised kasutajad pole seadistatud krüpteeringuks:");
        m.insert("Saving...", "Salvestamine...");
        m.insert("Go directly to your ", "Liigu otse oma");
        m.insert("personal settings", "isiklikes seadetes");
        m.insert("Encryption", "Krüpteerimine");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Luba taastevõti (võimalda kasutaja failide taastamine parooli kaotuse puhul):");
        m.insert("Recovery key password", "Taastevõtme parool");
        m.insert("Repeat Recovery key password", "Korda taastevõtme parooli");
        m.insert("Enabled", "Sisse lülitatud");
        m.insert("Disabled", "Väljalülitatud");
        m.insert("Change recovery key password:", "Muuda taastevõtme parooli:");
        m.insert("Old Recovery key password", "Vana taastevõtme parool");
        m.insert("New Recovery key password", "Uus taastevõtme parool");
        m.insert("Repeat New Recovery key password", "Korda uut taastevõtme parooli");
        m.insert("Change Password", "Muuda parooli");
        m.insert("Your private key password no longer match your log-in password:", "Sinu privaatse võtme parool ei ühti enam sinu sisselogimise parooliga:");
        m.insert("Set your old private key password to your current log-in password.", "Pane oma vana privaatvõtme parooliks oma praegune sisselogimise parool.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Kui sa ei mäleta oma vana parooli, siis palu oma süsteemihalduril taastada ligipääs failidele.");
        m.insert("Old log-in password", "Vana sisselogimise parool");
        m.insert("Current log-in password", "Praegune sisselogimise parool");
        m.insert("Update Private Key Password", "Uuenda privaatse võtme parooli");
        m.insert("Enable password recovery:", "Luba parooli taaste:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Valiku lubamine võimaldab taastada ligipääsu krüpteeritud failidele kui parooli kaotuse puhul");
        m.insert("File recovery settings updated", "Faili taaste seaded uuendatud");
        m.insert("Could not update file recovery", "Ei suuda uuendada taastefaili");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}