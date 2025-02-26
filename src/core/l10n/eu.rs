use std::collections::HashMap;
use rust_i18n::i18n;

pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("%s shared »%s« with you".to_string(), "%s-ek »%s« zurekin partekatu du".to_string());
    translations.insert("Sunday".to_string(), "Igandea".to_string());
    translations.insert("Monday".to_string(), "Astelehena".to_string());
    translations.insert("Tuesday".to_string(), "Asteartea".to_string());
    translations.insert("Wednesday".to_string(), "Asteazkena".to_string());
    translations.insert("Thursday".to_string(), "Osteguna".to_string());
    translations.insert("Friday".to_string(), "Ostirala".to_string());
    translations.insert("Saturday".to_string(), "Larunbata".to_string());
    translations.insert("January".to_string(), "Urtarrila".to_string());
    translations.insert("February".to_string(), "Otsaila".to_string());
    translations.insert("March".to_string(), "Martxoa".to_string());
    translations.insert("April".to_string(), "Apirila".to_string());
    translations.insert("May".to_string(), "Maiatza".to_string());
    translations.insert("June".to_string(), "Ekaina".to_string());
    translations.insert("July".to_string(), "Uztaila".to_string());
    translations.insert("August".to_string(), "Abuztua".to_string());
    translations.insert("September".to_string(), "Iraila".to_string());
    translations.insert("October".to_string(), "Urria".to_string());
    translations.insert("November".to_string(), "Azaroa".to_string());
    translations.insert("December".to_string(), "Abendua".to_string());
    translations.insert("Settings".to_string(), "Ezarpenak".to_string());
    translations.insert("seconds ago".to_string(), "segundu".to_string());
    translations.insert("today".to_string(), "gaur".to_string());
    translations.insert("yesterday".to_string(), "atzo".to_string());
    translations.insert("last month".to_string(), "joan den hilabetean".to_string());
    translations.insert("months ago".to_string(), "hilabete".to_string());
    translations.insert("last year".to_string(), "joan den urtean".to_string());
    translations.insert("years ago".to_string(), "urte".to_string());
    translations.insert("Choose".to_string(), "Aukeratu".to_string());
    translations.insert("Yes".to_string(), "Bai".to_string());
    translations.insert("No".to_string(), "Ez".to_string());
    translations.insert("Ok".to_string(), "Ados".to_string());
    translations.insert("Cancel".to_string(), "Ezeztatu".to_string());
    translations.insert("Shared".to_string(), "Elkarbanatuta".to_string());
    translations.insert("Share".to_string(), "Elkarbanatu".to_string());
    translations.insert("Error".to_string(), "Errorea".to_string());
    translations.insert("Error while sharing".to_string(), "Errore bat egon da elkarbanatzean".to_string());
    translations.insert("Error while unsharing".to_string(), "Errore bat egon da elkarbanaketa desegitean".to_string());
    translations.insert("Error while changing permissions".to_string(), "Errore bat egon da baimenak aldatzean".to_string());
    translations.insert("Shared with you and the group {group} by {owner}".to_string(), "{owner}-k zu eta {group} taldearekin elkarbanatuta".to_string());
    translations.insert("Shared with you by {owner}".to_string(), "{owner}-k zurekin elkarbanatuta".to_string());
    translations.insert("Password protect".to_string(), "Babestu pasahitzarekin".to_string());
    translations.insert("Password".to_string(), "Pasahitza".to_string());
    translations.insert("Allow Public Upload".to_string(), "Gaitu igotze publikoa".to_string());
    translations.insert("Email link to person".to_string(), "Postaz bidali lotura ".to_string());
    translations.insert("Send".to_string(), "Bidali".to_string());
    translations.insert("Set expiration date".to_string(), "Ezarri muga data".to_string());
    translations.insert("Expiration date".to_string(), "Muga data".to_string());
    translations.insert("Share via email:".to_string(), "Elkarbanatu eposta bidez:".to_string());
    translations.insert("No people found".to_string(), "Ez da inor aurkitu".to_string());
    translations.insert("group".to_string(), "taldea".to_string());
    translations.insert("Resharing is not allowed".to_string(), "Berriz elkarbanatzea ez dago baimendua".to_string());
    translations.insert("Shared in {item} with {user}".to_string(), "{user}ekin {item}-n elkarbanatuta".to_string());
    translations.insert("Unshare".to_string(), "Ez elkarbanatu".to_string());
    translations.insert("can edit".to_string(), "editatu dezake".to_string());
    translations.insert("access control".to_string(), "sarrera kontrola".to_string());
    translations.insert("create".to_string(), "sortu".to_string());
    translations.insert("update".to_string(), "eguneratu".to_string());
    translations.insert("delete".to_string(), "ezabatu".to_string());
    translations.insert("share".to_string(), "elkarbanatu".to_string());
    translations.insert("Password protected".to_string(), "Pasahitzarekin babestuta".to_string());
    translations.insert("Error unsetting expiration date".to_string(), "Errorea izan da muga data kentzean".to_string());
    translations.insert("Error setting expiration date".to_string(), "Errore bat egon da muga data ezartzean".to_string());
    translations.insert("Sending ...".to_string(), "Bidaltzen ...".to_string());
    translations.insert("Email sent".to_string(), "Eposta bidalia".to_string());
    translations.insert("Warning".to_string(), "Abisua".to_string());
    translations.insert("The object type is not specified.".to_string(), "Objetu mota ez dago zehaztuta.".to_string());
    translations.insert("Delete".to_string(), "Ezabatu".to_string());
    translations.insert("Add".to_string(), "Gehitu".to_string());
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.".to_string(), "Eguneraketa ez da ongi egin. Mesedez egin arazoaren txosten bat <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud komunitatearentzako</a>.".to_string());
    translations.insert("The update was successful. Redirecting you to ownCloud now.".to_string(), "Eguneraketa ongi egin da. Orain zure ownClouderea berbideratua izango zara.".to_string());
    translations.insert("%s password reset".to_string(), "%s pasahitza berrezarri".to_string());
    translations.insert("Use the following link to reset your password: {link}".to_string(), "Eribili hurrengo lotura zure pasahitza berrezartzeko: {link}".to_string());
    translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .".to_string(), "Zure pasahitza berrezartzeko lotura zure postara bidalia izan da.<br>Ez baduzu arrazoizko denbora \nepe batean jasotzen begiratu zure zabor-posta karpetan.<br>Hor ere ez badago kudeatzailearekin harremanetan ipini.".to_string());
    translations.insert("Request failed!<br>Did you make sure your email/username was right?".to_string(), "Eskaerak huts egin du!<br>Ziur zaude posta/pasahitza zuzenak direla?".to_string());
    translations.insert("You will receive a link to reset your password via Email.".to_string(), "Zure pashitza berrezartzeko lotura bat jasoko duzu Epostaren bidez.".to_string());
    translations.insert("Username".to_string(), "Erabiltzaile izena".to_string());
    translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?".to_string(), "Zure fitxategiak enkriptaturik daude. Ez baduzu berreskuratze gakoa gaitzen pasahitza berrabiaraztean ez da zure fitxategiak berreskuratzeko modurik egongo. Zer egin ziur ez bazaude kudeatzailearekin harremanetan ipini jarraitu aurretik. Ziur zaude aurrera jarraitu nahi duzula?".to_string());
    translations.insert("Yes, I really want to reset my password now".to_string(), "Bai, nire pasahitza orain berrabiarazi nahi dut".to_string());
    translations.insert("Your password was reset".to_string(), "Zure pasahitza berrezarri da".to_string());
    translations.insert("To login page".to_string(), "Sarrera orrira".to_string());
    translations.insert("New password".to_string(), "Pasahitz berria".to_string());
    translations.insert("Reset password".to_string(), "Berrezarri pasahitza".to_string());
    translations.insert("Personal".to_string(), "Pertsonala".to_string());
    translations.insert("Users".to_string(), "Erabiltzaileak".to_string());
    translations.insert("Apps".to_string(), "Aplikazioak".to_string());
    translations.insert("Admin".to_string(), "Admin".to_string());
    translations.insert("Help".to_string(), "Laguntza".to_string());
    translations.insert("Access forbidden".to_string(), "Sarrera debekatuta".to_string());
    translations.insert("Cloud not found".to_string(), "Ez da hodeia aurkitu".to_string());
    translations.insert("Security Warning".to_string(), "Segurtasun abisua".to_string());
    translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)".to_string(), "Zure PHP bertsioa NULL Byte erasoak (CVE-2006-7243) mendera dezake.".to_string());
    translations.insert("Please update your PHP installation to use %s securely.".to_string(), "Mesedez eguneratu zure PHP instalazioa %s seguru erabiltzeko".to_string());
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.".to_string(), "Ez dago hausazko zenbaki sortzaile segururik eskuragarri, mesedez gatiu PHP OpenSSL extensioa.".to_string());
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.".to_string(), "Hausazko zenbaki sortzaile segururik gabe erasotzaile batek pasahitza berrezartzeko kodeak iragarri ditzake eta zure kontuaz jabetu.".to_string());
    translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.".to_string(), "Zure data karpeta eta fitxategiak interneten bidez eskuragarri egon daitezke .htaccess fitxategia ez delako funtzionatzen ari.".to_string());
    translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.".to_string(), "Zure zerbitrzaria ongi konfiguratzeko, mezedez <a href=\"%s\" target=\"_blank\">dokumentazioa</a> ikusi.".to_string());
    translations.insert("Create an <strong>admin account</strong>".to_string(), "Sortu <strong>kudeatzaile kontu<strong> bat".to_string());
    translations.insert("Advanced".to_string(), "Aurreratua".to_string());
    translations.insert("Data folder".to_string(), "Datuen karpeta".to_string());
    translations.insert("Configure the database".to_string(), "Konfiguratu datu basea".to_string());
    translations.insert("will be used".to_string(), "erabiliko da".to_string());
    translations.insert("Database user".to_string(), "Datubasearen erabiltzailea".to_string());
    translations.insert("Database password".to_string(), "Datubasearen pasahitza".to_string());
    translations.insert("Database name".to_string(), "Datubasearen izena".to_string());
    translations.insert("Database tablespace".to_string(), "Datu basearen taula-lekua".to_string());
    translations.insert("Database host".to_string(), "Datubasearen hostalaria".to_string());
    translations.insert("Finish setup".to_string(), "Bukatu konfigurazioa".to_string());
    translations.insert("%s is available. Get more information on how to update.".to_string(), "%s erabilgarri dago. Eguneratzeaz argibide gehiago eskuratu.".to_string());
    translations.insert("Log out".to_string(), "Saioa bukatu".to_string());
    translations.insert("Automatic logon rejected!".to_string(), "Saio hasiera automatikoa ez onartuta!".to_string());
    translations.insert("If you did not change your password recently, your account may be compromised!".to_string(), "Zure pasahitza orain dela gutxi ez baduzu aldatu, zure kontua arriskuan egon daiteke!".to_string());
    translations.insert("Please change your password to secure your account again.".to_string(), "Mesedez aldatu zure pasahitza zure kontua berriz segurtatzeko.".to_string());
    translations.insert("Lost your password?".to_string(), "Galdu duzu pasahitza?".to_string());
    translations.insert("remember".to_string(), "gogoratu".to_string());
    translations.insert("Log in".to_string(), "Hasi saioa".to_string());
    translations.insert("Alternative Logins".to_string(), "Beste erabiltzaile izenak".to_string());
    translations.insert("Updating ownCloud to version %s, this may take a while.".to_string(), "ownCloud %s bertsiora eguneratzen, denbora har dezake.".to_string());
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plurals_map() -> HashMap<String, Vec<String>> {
    let mut plurals = HashMap::new();
    
    plurals.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        vec!["orain dela minutu %n".to_string(), "orain dela %n minutu".to_string()]
    );
    
    plurals.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        vec!["orain dela ordu %n".to_string(), "orain dela %n ordu".to_string()]
    );
    
    plurals.insert(
        "_%n day ago_::_%n days ago_".to_string(),
        vec!["orain dela egun %n".to_string(), "orain dela %n egun".to_string()]
    );
    
    plurals.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        vec!["orain dela hilabete %n".to_string(), "orain dela %n hilabete".to_string()]
    );
    
    plurals.insert(
        "_{count} file conflict_::_{count} file conflicts_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plurals
}

// Implementa i18n mediante un trait
i18n!("eu");