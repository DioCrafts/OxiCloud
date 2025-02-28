use std::collections::HashMap;

#[allow(dead_code)]
pub struct Translations {
    translations: HashMap<&'static str, SlValue>,
    plural_forms: &'static str,
}

#[derive(Clone)]
pub enum SlValue {
    String(&'static str),
    Array(Vec<&'static str>),
}

impl Default for Translations {
    fn default() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("%s shared »%s« with you", SlValue::String("%s je delil »%s« z vami"));
        translations.insert("Sunday", SlValue::String("nedelja"));
        translations.insert("Monday", SlValue::String("ponedeljek"));
        translations.insert("Tuesday", SlValue::String("torek"));
        translations.insert("Wednesday", SlValue::String("sreda"));
        translations.insert("Thursday", SlValue::String("četrtek"));
        translations.insert("Friday", SlValue::String("petek"));
        translations.insert("Saturday", SlValue::String("sobota"));
        translations.insert("January", SlValue::String("januar"));
        translations.insert("February", SlValue::String("februar"));
        translations.insert("March", SlValue::String("marec"));
        translations.insert("April", SlValue::String("april"));
        translations.insert("May", SlValue::String("maj"));
        translations.insert("June", SlValue::String("junij"));
        translations.insert("July", SlValue::String("julij"));
        translations.insert("August", SlValue::String("avgust"));
        translations.insert("September", SlValue::String("september"));
        translations.insert("October", SlValue::String("oktober"));
        translations.insert("November", SlValue::String("november"));
        translations.insert("December", SlValue::String("december"));
        translations.insert("Settings", SlValue::String("Nastavitve"));
        translations.insert("seconds ago", SlValue::String("pred nekaj sekundami"));
        translations.insert("_%n minute ago_::_%n minutes ago_", SlValue::Array(vec!["", "", "", ""]));
        translations.insert("_%n hour ago_::_%n hours ago_", SlValue::Array(vec!["", "", "", ""]));
        translations.insert("today", SlValue::String("danes"));
        translations.insert("yesterday", SlValue::String("včeraj"));
        translations.insert("_%n day ago_::_%n days ago_", SlValue::Array(vec!["", "", "", ""]));
        translations.insert("last month", SlValue::String("zadnji mesec"));
        translations.insert("_%n month ago_::_%n months ago_", SlValue::Array(vec!["", "", "", ""]));
        translations.insert("months ago", SlValue::String("mesecev nazaj"));
        translations.insert("last year", SlValue::String("lansko leto"));
        translations.insert("years ago", SlValue::String("let nazaj"));
        translations.insert("Choose", SlValue::String("Izbor"));
        translations.insert("Yes", SlValue::String("Da"));
        translations.insert("No", SlValue::String("Ne"));
        translations.insert("Ok", SlValue::String("V redu"));
        translations.insert("_{count} file conflict_::_{count} file conflicts_", SlValue::Array(vec!["", "", "", ""]));
        translations.insert("Cancel", SlValue::String("Prekliči"));
        translations.insert("Shared", SlValue::String("V souporabi"));
        translations.insert("Share", SlValue::String("Souporaba"));
        translations.insert("Error", SlValue::String("Napaka"));
        translations.insert("Error while sharing", SlValue::String("Napaka med souporabo"));
        translations.insert("Error while unsharing", SlValue::String("Napaka med odstranjevanjem souporabe"));
        translations.insert("Error while changing permissions", SlValue::String("Napaka med spreminjanjem dovoljenj"));
        translations.insert("Shared with you and the group {group} by {owner}", SlValue::String("V souporabi z vami in skupino {group}. Lastnik je {owner}."));
        translations.insert("Shared with you by {owner}", SlValue::String("V souporabi z vami. Lastnik je {owner}."));
        translations.insert("Password protect", SlValue::String("Zaščiti z geslom"));
        translations.insert("Password", SlValue::String("Geslo"));
        translations.insert("Allow Public Upload", SlValue::String("Dovoli javne prenose na strežnik"));
        translations.insert("Email link to person", SlValue::String("Posreduj povezavo po elektronski pošti"));
        translations.insert("Send", SlValue::String("Pošlji"));
        translations.insert("Set expiration date", SlValue::String("Nastavi datum preteka"));
        translations.insert("Expiration date", SlValue::String("Datum preteka"));
        translations.insert("Share via email:", SlValue::String("Souporaba preko elektronske pošte:"));
        translations.insert("No people found", SlValue::String("Ni najdenih uporabnikov"));
        translations.insert("group", SlValue::String("skupina"));
        translations.insert("Resharing is not allowed", SlValue::String("Nadaljnja souporaba ni dovoljena"));
        translations.insert("Shared in {item} with {user}", SlValue::String("V souporabi v {item} z {user}"));
        translations.insert("Unshare", SlValue::String("Prekliči souporabo"));
        translations.insert("can edit", SlValue::String("lahko ureja"));
        translations.insert("access control", SlValue::String("nadzor dostopa"));
        translations.insert("create", SlValue::String("ustvari"));
        translations.insert("update", SlValue::String("posodobi"));
        translations.insert("delete", SlValue::String("izbriši"));
        translations.insert("share", SlValue::String("določi souporabo"));
        translations.insert("Password protected", SlValue::String("Zaščiteno z geslom"));
        translations.insert("Error unsetting expiration date", SlValue::String("Napaka brisanja datuma preteka"));
        translations.insert("Error setting expiration date", SlValue::String("Napaka med nastavljanjem datuma preteka"));
        translations.insert("Sending ...", SlValue::String("Pošiljanje ..."));
        translations.insert("Email sent", SlValue::String("Elektronska pošta je poslana"));
        translations.insert("Warning", SlValue::String("Opozorilo"));
        translations.insert("The object type is not specified.", SlValue::String("Vrsta predmeta ni podana."));
        translations.insert("Delete", SlValue::String("Izbriši"));
        translations.insert("Add", SlValue::String("Dodaj"));
        translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", SlValue::String("Posodobitev ni uspela. Pošljite poročilo o napaki na sistemu <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud</a>."));
        translations.insert("The update was successful. Redirecting you to ownCloud now.", SlValue::String("Posodobitev je uspešno končana. Stran bo preusmerjena na oblak ownCloud."));
        translations.insert("Use the following link to reset your password: {link}", SlValue::String("Za ponastavitev gesla uporabite povezavo: {link}"));
        translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", SlValue::String("Povezava za ponastavitev gesla je bila poslana na elektronski naslov.<br>V kolikor sporočila ne prejmete v doglednem času, preverite tudi mape vsiljene pošte.<br>Če ne bo niti tam, stopite v stik s skrbnikom."));
        translations.insert("Request failed!<br>Did you make sure your email/username was right?", SlValue::String("Zahteva je spodletela!<br>Ali sta elektronski naslov oziroma uporabniško ime navedena pravilno?"));
        translations.insert("You will receive a link to reset your password via Email.", SlValue::String("Na elektronski naslov boste prejeli povezavo za ponovno nastavitev gesla."));
        translations.insert("Username", SlValue::String("Uporabniško ime"));
        translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", SlValue::String("Datoteke so šifrirane. Če niste omogočili ključa za obnovitev, žal podatkov ne bo mogoče pridobiti nazaj, ko boste geslo enkrat spremenili. Če niste prepričani, kaj storiti, se obrnite na skrbnika storitve. Ste prepričani, da želite nadaljevati?"));
        translations.insert("Yes, I really want to reset my password now", SlValue::String("Da, potrjujem ponastavitev gesla"));
        translations.insert("Reset", SlValue::String("Ponastavi"));
        translations.insert("Your password was reset", SlValue::String("Geslo je ponovno nastavljeno"));
        translations.insert("To login page", SlValue::String("Na prijavno stran"));
        translations.insert("New password", SlValue::String("Novo geslo"));
        translations.insert("Reset password", SlValue::String("Ponastavi geslo"));
        translations.insert("Personal", SlValue::String("Osebno"));
        translations.insert("Users", SlValue::String("Uporabniki"));
        translations.insert("Apps", SlValue::String("Programi"));
        translations.insert("Admin", SlValue::String("Skrbništvo"));
        translations.insert("Help", SlValue::String("Pomoč"));
        translations.insert("Access forbidden", SlValue::String("Dostop je prepovedan"));
        translations.insert("Cloud not found", SlValue::String("Oblaka ni mogoče najti"));
        translations.insert("Security Warning", SlValue::String("Varnostno opozorilo"));
        translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", SlValue::String("Uporabljena različica PHP je ranljiva za napad NULL Byte (CVE-2006-7243)"));
        translations.insert("Please update your PHP installation to use %s securely.", SlValue::String("Za varno uporabo storitve %s posodobite PHP"));
        translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", SlValue::String("Na voljo ni nobenega varnega ustvarjalnika naključnih števil. Omogočiti je treba razširitev PHP OpenSSL."));
        translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", SlValue::String("Brez varnega ustvarjalnika naključnih števil je mogoče napovedati žetone za ponastavitev gesla, s čimer je mogoče prevzeti nadzor nad računom."));
        translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", SlValue::String("Podatkovna mapa in datoteke so najverjetneje javno dostopni preko interneta, saj datoteka .htaccess ni ustrezno nastavljena."));
        translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", SlValue::String("Za navodila, kako pravilno nastaviti vaš strežnik, kliknite na povezavo do <a href=\"%s\" target=\"_blank\">dokumentacije</a>."));
        translations.insert("Create an <strong>admin account</strong>", SlValue::String("Ustvari <strong>skrbniški račun</strong>"));
        translations.insert("Advanced", SlValue::String("Napredne možnosti"));
        translations.insert("Data folder", SlValue::String("Podatkovna mapa"));
        translations.insert("Configure the database", SlValue::String("Nastavi podatkovno zbirko"));
        translations.insert("will be used", SlValue::String("bo uporabljen"));
        translations.insert("Database user", SlValue::String("Uporabnik podatkovne zbirke"));
        translations.insert("Database password", SlValue::String("Geslo podatkovne zbirke"));
        translations.insert("Database name", SlValue::String("Ime podatkovne zbirke"));
        translations.insert("Database tablespace", SlValue::String("Razpredelnica podatkovne zbirke"));
        translations.insert("Database host", SlValue::String("Gostitelj podatkovne zbirke"));
        translations.insert("Finish setup", SlValue::String("Končaj namestitev"));
        translations.insert("%s is available. Get more information on how to update.", SlValue::String("%s je na voljo. Pridobite več podrobnosti za posodobitev."));
        translations.insert("Log out", SlValue::String("Odjava"));
        translations.insert("Automatic logon rejected!", SlValue::String("Samodejno prijavljanje je zavrnjeno!"));
        translations.insert("If you did not change your password recently, your account may be compromised!", SlValue::String("V primeru, da gesla za dostop že nekaj časa niste spremenili, je račun lahko ogrožen!"));
        translations.insert("Please change your password to secure your account again.", SlValue::String("Spremenite geslo za izboljšanje zaščite računa."));
        translations.insert("Lost your password?", SlValue::String("Ali ste pozabili geslo?"));
        translations.insert("remember", SlValue::String("zapomni si"));
        translations.insert("Log in", SlValue::String("Prijava"));
        translations.insert("Alternative Logins", SlValue::String("Druge prijavne možnosti"));
        translations.insert("Updating ownCloud to version %s, this may take a while.", SlValue::String("Posodabljanje sistema ownCloud na različico %s je lahko dolgotrajno."));
        
        Translations {
            translations,
            plural_forms: "nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);",
        }
    }
}

impl Translations {
    pub fn get(&self, key: &str) -> Option<&SlValue> {
        self.translations.get(key)
    }

    pub fn plural_forms(&self) -> &str {
        self.plural_forms
    }
}