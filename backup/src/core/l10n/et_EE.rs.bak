use std::collections::HashMap;
use rust_i18n::t;

#[derive(Debug, Clone)]
pub struct EtEE {
    pub translations: HashMap<String, String>,
    pub plural_forms: &'static str,
}

impl Default for EtEE {
    fn default() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("%s shared »%s« with you".to_string(), "%s jagas sinuga »%s«".to_string());
        translations.insert("Couldn't send mail to following users: %s ".to_string(), "Kirja saatmine järgnevatele kasutajatele ebaõnnestus: %s ".to_string());
        translations.insert("Turned on maintenance mode".to_string(), "Haldusrežiimis sisse lülitatud".to_string());
        translations.insert("Turned off maintenance mode".to_string(), "Haldusrežiimis välja lülitatud".to_string());
        translations.insert("Updated database".to_string(), "Uuendatud andmebaas".to_string());
        translations.insert("Updating filecache, this may take really long...".to_string(), "Failipuhvri uuendamine, see võib kesta väga kaua...".to_string());
        translations.insert("Updated filecache".to_string(), "Uuendatud failipuhver".to_string());
        translations.insert("... %d%% done ...".to_string(), "... %d%% tehtud ...".to_string());
        translations.insert("No image or file provided".to_string(), "Ühtegi pilti või faili pole pakutud".to_string());
        translations.insert("Unknown filetype".to_string(), "Tundmatu failitüüp".to_string());
        translations.insert("Invalid image".to_string(), "Vigane pilt".to_string());
        translations.insert("No temporary profile picture available, try again".to_string(), "Ühtegi ajutist profiili pilti pole saadaval, proovi uuesti".to_string());
        translations.insert("No crop data provided".to_string(), "Lõikeandmeid ei leitud".to_string());
        translations.insert("Sunday".to_string(), "Pühapäev".to_string());
        translations.insert("Monday".to_string(), "Esmaspäev".to_string());
        translations.insert("Tuesday".to_string(), "Teisipäev".to_string());
        translations.insert("Wednesday".to_string(), "Kolmapäev".to_string());
        translations.insert("Thursday".to_string(), "Neljapäev".to_string());
        translations.insert("Friday".to_string(), "Reede".to_string());
        translations.insert("Saturday".to_string(), "Laupäev".to_string());
        translations.insert("January".to_string(), "Jaanuar".to_string());
        translations.insert("February".to_string(), "Veebruar".to_string());
        translations.insert("March".to_string(), "Märts".to_string());
        translations.insert("April".to_string(), "Aprill".to_string());
        translations.insert("May".to_string(), "Mai".to_string());
        translations.insert("June".to_string(), "Juuni".to_string());
        translations.insert("July".to_string(), "Juuli".to_string());
        translations.insert("August".to_string(), "August".to_string());
        translations.insert("September".to_string(), "September".to_string());
        translations.insert("October".to_string(), "Oktoober".to_string());
        translations.insert("November".to_string(), "November".to_string());
        translations.insert("December".to_string(), "Detsember".to_string());
        translations.insert("Settings".to_string(), "Seaded".to_string());
        translations.insert("seconds ago".to_string(), "sekundit tagasi".to_string());
        translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), "%n minut tagasi,%n minutit tagasi".to_string());
        translations.insert("_%n hour ago_::_%n hours ago_".to_string(), "%n tund tagasi,%n tundi tagasi".to_string());
        translations.insert("today".to_string(), "täna".to_string());
        translations.insert("yesterday".to_string(), "eile".to_string());
        translations.insert("_%n day ago_::_%n days ago_".to_string(), "%n päev tagasi,%n päeva tagasi".to_string());
        translations.insert("last month".to_string(), "viimasel kuul".to_string());
        translations.insert("_%n month ago_::_%n months ago_".to_string(), "%n kuu tagasi,%n kuud tagasi".to_string());
        translations.insert("months ago".to_string(), "kuu tagasi".to_string());
        translations.insert("last year".to_string(), "viimasel aastal".to_string());
        translations.insert("years ago".to_string(), "aastat tagasi".to_string());
        translations.insert("Choose".to_string(), "Vali".to_string());
        translations.insert("Error loading file picker template: {error}".to_string(), "Viga failivalija malli laadimisel:  {error}".to_string());
        translations.insert("Yes".to_string(), "Jah".to_string());
        translations.insert("No".to_string(), "Ei".to_string());
        translations.insert("Ok".to_string(), "Ok".to_string());
        translations.insert("Error loading message template: {error}".to_string(), "Viga sõnumi malli laadimisel: {error}".to_string());
        translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), "{count} failikonflikt,{count} failikonflikti".to_string());
        translations.insert("One file conflict".to_string(), "Üks failikonflikt".to_string());
        translations.insert("Which files do you want to keep?".to_string(), "Milliseid faile sa soovid alles hoida?".to_string());
        translations.insert("If you select both versions, the copied file will have a number added to its name.".to_string(), "Kui valid mõlemad versioonid, siis lisatakse kopeeritud faili nimele number.".to_string());
        translations.insert("Cancel".to_string(), "Loobu".to_string());
        translations.insert("Continue".to_string(), "Jätka".to_string());
        translations.insert("(all selected)".to_string(), "(kõik valitud)".to_string());
        translations.insert("({count} selected)".to_string(), "({count} valitud)".to_string());
        translations.insert("Error loading file exists template".to_string(), "Viga faili olemasolu malli laadimisel".to_string());
        translations.insert("Shared".to_string(), "Jagatud".to_string());
        translations.insert("Share".to_string(), "Jaga".to_string());
        translations.insert("Error".to_string(), "Viga".to_string());
        translations.insert("Error while sharing".to_string(), "Viga jagamisel".to_string());
        translations.insert("Error while unsharing".to_string(), "Viga jagamise lõpetamisel".to_string());
        translations.insert("Error while changing permissions".to_string(), "Viga õiguste muutmisel".to_string());
        translations.insert("Shared with you and the group {group} by {owner}".to_string(), "Jagatud sinu ja {group} grupiga {owner} poolt".to_string());
        translations.insert("Shared with you by {owner}".to_string(), "Sinuga jagas {owner}".to_string());
        translations.insert("Share with user or group …".to_string(), "Jaga kasutaja või grupiga ...".to_string());
        translations.insert("Share link".to_string(), "Jaga linki".to_string());
        translations.insert("Password protect".to_string(), "Parooliga kaitstud".to_string());
        translations.insert("Password".to_string(), "Parool".to_string());
        translations.insert("Allow Public Upload".to_string(), "Luba avalik üleslaadimine".to_string());
        translations.insert("Email link to person".to_string(), "Saada link isikule e-postiga".to_string());
        translations.insert("Send".to_string(), "Saada".to_string());
        translations.insert("Set expiration date".to_string(), "Määra aegumise kuupäev".to_string());
        translations.insert("Expiration date".to_string(), "Aegumise kuupäev".to_string());
        translations.insert("Share via email:".to_string(), "Jaga e-postiga:".to_string());
        translations.insert("No people found".to_string(), "Ühtegi inimest ei leitud".to_string());
        translations.insert("group".to_string(), "grupp".to_string());
        translations.insert("Resharing is not allowed".to_string(), "Edasijagamine pole lubatud".to_string());
        translations.insert("Shared in {item} with {user}".to_string(), "Jagatud {item} kasutajaga {user}".to_string());
        translations.insert("Unshare".to_string(), "Lõpeta jagamine".to_string());
        translations.insert("notify by email".to_string(), "teavita e-postiga".to_string());
        translations.insert("can edit".to_string(), "saab muuta".to_string());
        translations.insert("access control".to_string(), "ligipääsukontroll".to_string());
        translations.insert("create".to_string(), "loo".to_string());
        translations.insert("update".to_string(), "uuenda".to_string());
        translations.insert("delete".to_string(), "kustuta".to_string());
        translations.insert("share".to_string(), "jaga".to_string());
        translations.insert("Password protected".to_string(), "Parooliga kaitstud".to_string());
        translations.insert("Error unsetting expiration date".to_string(), "Viga aegumise kuupäeva eemaldamisel".to_string());
        translations.insert("Error setting expiration date".to_string(), "Viga aegumise kuupäeva määramisel".to_string());
        translations.insert("Sending ...".to_string(), "Saatmine ...".to_string());
        translations.insert("Email sent".to_string(), "E-kiri on saadetud".to_string());
        translations.insert("Warning".to_string(), "Hoiatus".to_string());
        translations.insert("The object type is not specified.".to_string(), "Objekti tüüp pole määratletud.".to_string());
        translations.insert("Enter new".to_string(), "Sisesta uus".to_string());
        translations.insert("Delete".to_string(), "Kustuta".to_string());
        translations.insert("Add".to_string(), "Lisa".to_string());
        translations.insert("Edit tags".to_string(), "Muuda silte".to_string());
        translations.insert("Error loading dialog template: {error}".to_string(), "Viga dialoogi malli laadimisel: {error}".to_string());
        translations.insert("No tags selected for deletion.".to_string(), "Kustutamiseks pole ühtegi silti valitud.".to_string());
        translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.".to_string(), "Uuendus ebaõnnestus. Palun teavita probleemidest  <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud kogukonda</a>.".to_string());
        translations.insert("The update was successful. Redirecting you to ownCloud now.".to_string(), "Uuendus oli edukas. Kohe suunatakse Sind ownCloudi.".to_string());
        translations.insert("%s password reset".to_string(), "%s parooli lähtestus".to_string());
        translations.insert("Use the following link to reset your password: {link}".to_string(), "Kasuta järgnevat linki oma parooli taastamiseks: {link}".to_string());
        translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .".to_string(), "Link parooli vahetuseks on saadetud Sinu e-posti aadressile.<br>Kui kiri pole saabunud mõistliku aja jooksul, siis kontrolli oma spam-/rämpskirjade katalooge.<br>Kui kirja pole ka seal, siis küsi abi süsteemihaldurilt.".to_string());
        translations.insert("Request failed!<br>Did you make sure your email/username was right?".to_string(), "Päring ebaõnnestus!<br>Oled sa veendunud, et e-post/kasutajanimi on õiged?".to_string());
        translations.insert("You will receive a link to reset your password via Email.".to_string(), "Sinu parooli taastamise link saadetakse sulle e-postile.".to_string());
        translations.insert("Username".to_string(), "Kasutajanimi".to_string());
        translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?".to_string(), "Sinu failid on krüpteeritud. Kui sa pole taastamise võtit veel määranud, siis pole präast parooli taastamist mingit võimalust sinu andmeid tagasi saada. Kui sa pole kindel, mida teha, siis palun väta enne jätkamist ühendust oma administaatoriga. Oled sa kindel, et sa soovid jätkata?".to_string());
        translations.insert("Yes, I really want to reset my password now".to_string(), "Jah, ma tõesti soovin oma parooli praegu nullida".to_string());
        translations.insert("Reset".to_string(), "Algseaded".to_string());
        translations.insert("Your password was reset".to_string(), "Sinu parool on taastatud".to_string());
        translations.insert("To login page".to_string(), "Sisselogimise lehele".to_string());
        translations.insert("New password".to_string(), "Uus parool".to_string());
        translations.insert("Reset password".to_string(), "Nulli parool".to_string());
        translations.insert("Personal".to_string(), "Isiklik".to_string());
        translations.insert("Users".to_string(), "Kasutajad".to_string());
        translations.insert("Apps".to_string(), "Rakendused".to_string());
        translations.insert("Admin".to_string(), "Admin".to_string());
        translations.insert("Help".to_string(), "Abiinfo".to_string());
        translations.insert("Error loading tags".to_string(), "Viga siltide laadimisel".to_string());
        translations.insert("Tag already exists".to_string(), "Silt on juba olemas".to_string());
        translations.insert("Error deleting tag(s)".to_string(), "Viga sildi (siltide) kustutamisel".to_string());
        translations.insert("Error tagging".to_string(), "Viga sildi lisamisel".to_string());
        translations.insert("Error untagging".to_string(), "Viga sildi eemaldamisel".to_string());
        translations.insert("Error favoriting".to_string(), "Viga lemmikuks lisamisel".to_string());
        translations.insert("Error unfavoriting".to_string(), "Viga lemmikutest eemaldamisel".to_string());
        translations.insert("Access forbidden".to_string(), "Ligipääs on keelatud".to_string());
        translations.insert("Cloud not found".to_string(), "Pilve ei leitud".to_string());
        translations.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n".to_string(), "Hei,\n\nlihtsalt annan sulle teada, et %s jagas sulle välja %s.\nVaata seda: %s\n\n".to_string());
        translations.insert("The share will expire on %s.\n\n".to_string(), "Jagamine aegub %s.\n\n".to_string());
        translations.insert("Cheers!".to_string(), "Terekest!".to_string());
        translations.insert("Security Warning".to_string(), "Turvahoiatus".to_string());
        translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)".to_string(), "Sinu PHP versioon on haavatav NULL Baidi (CVE-2006-7243) rünnakuga.".to_string());
        translations.insert("Please update your PHP installation to use %s securely.".to_string(), "Palun uuenda oma paigaldatud PHP-d tagamaks %s turvalisus.".to_string());
        translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.".to_string(), "Turvalist juhuslike numbrite generaatorit pole saadaval. Palun luba PHP-s OpenSSL laiendus.".to_string());
        translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.".to_string(), "Ilma turvalise juhuslike numbrite generaatorita võib ründaja ennustada paroolivahetuse võtme ning hõivata su konto.".to_string());
        translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.".to_string(), "Su andmete kataloog ja failid on tõenäoliselt internetist vabalt saadaval kuna .htaccess fail ei toimi.".to_string());
        translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.".to_string(), "Serveri korrektseks seadistuseks palun tutvu <a href=\"%s\" target=\"_blank\">dokumentatsiooniga</a>.".to_string());
        translations.insert("Create an <strong>admin account</strong>".to_string(), "Loo <strong>admini konto</strong>".to_string());
        translations.insert("Advanced".to_string(), "Täpsem".to_string());
        translations.insert("Data folder".to_string(), "Andmete kaust".to_string());
        translations.insert("Configure the database".to_string(), "Seadista andmebaasi".to_string());
        translations.insert("will be used".to_string(), "kasutatakse".to_string());
        translations.insert("Database user".to_string(), "Andmebaasi kasutaja".to_string());
        translations.insert("Database password".to_string(), "Andmebaasi parool".to_string());
        translations.insert("Database name".to_string(), "Andmebasi nimi".to_string());
        translations.insert("Database tablespace".to_string(), "Andmebaasi tabeliruum".to_string());
        translations.insert("Database host".to_string(), "Andmebaasi host".to_string());
        translations.insert("Finish setup".to_string(), "Lõpeta seadistamine".to_string());
        translations.insert("Finishing …".to_string(), "Lõpetamine ...".to_string());
        translations.insert("%s is available. Get more information on how to update.".to_string(), "%s on saadaval. Vaata lähemalt kuidas uuendada.".to_string());
        translations.insert("Log out".to_string(), "Logi välja".to_string());
        translations.insert("Automatic logon rejected!".to_string(), "Automaatne sisselogimine lükati tagasi!".to_string());
        translations.insert("If you did not change your password recently, your account may be compromised!".to_string(), "Kui sa ei muutnud oma parooli hiljuti, siis võib su kasutajakonto olla ohustatud!".to_string());
        translations.insert("Please change your password to secure your account again.".to_string(), "Palun muuda parooli, et oma kasutajakonto uuesti turvata.".to_string());
        translations.insert("Server side authentication failed!".to_string(), "Serveripoolne autentimine ebaõnnestus!".to_string());
        translations.insert("Please contact your administrator.".to_string(), "Palun kontakteeru oma süsteemihalduriga.".to_string());
        translations.insert("Lost your password?".to_string(), "Kaotasid oma parooli?".to_string());
        translations.insert("remember".to_string(), "pea meeles".to_string());
        translations.insert("Log in".to_string(), "Logi sisse".to_string());
        translations.insert("Alternative Logins".to_string(), "Alternatiivsed sisselogimisviisid".to_string());
        translations.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>".to_string(), "Hei,<br><br>lihtsalt annan sulle teada, et %s jagas sulle välja »%s«.<br><a href=\"%s\">Vaata seda!</a><br><br>".to_string());
        translations.insert("The share will expire on %s.<br><br>".to_string(), "Jagamine aegub %s.<br><br>".to_string());
        translations.insert("Updating ownCloud to version %s, this may take a while.".to_string(), "ownCloudi uuendamine versioonile %s. See võib veidi aega võtta.".to_string());
        translations.insert("This ownCloud instance is currently being updated, which may take a while.".to_string(), "Seda ownCloud instantsi hetkel uuendatakse, võib võtta veidi aega.".to_string());
        translations.insert("Please reload this page after a short time to continue using ownCloud.".to_string(), "Palun laadi see leht uuesti veidi aja pärast jätkamaks ownCloud kasutamist.".to_string());
        translations.insert("Contact your system administrator if this message persists or appeared unexpectedly.".to_string(), "Kontakteeru oma süsteemihalduriga kui see teade püsib või on tekkinud ootamatult.".to_string());
        translations.insert("Thank you for your patience.".to_string(), "Täname kannatlikkuse eest.".to_string());

        Self {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);",
        }
    }
}

impl EtEE {
    pub fn get_translation(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }
    
    pub fn get_plural_forms(&self) -> &'static str {
        self.plural_forms
    }
}