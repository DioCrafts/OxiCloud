// sq.rs - Albanian translations

use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%s shared »%s« with you", "%s ndau »%s« me ju");
        m.insert("Turned on maintenance mode", "Mënyra e mirëmbajtjes u aktivizua");
        m.insert("Turned off maintenance mode", "Mënyra e mirëmbajtjes u çaktivizua");
        m.insert("Updated database", "Database-i u azhurnua");
        m.insert("Updating filecache, this may take really long...", "Po azhurnoj memorjen e skedarëve, mund të zgjasi pak...");
        m.insert("Updated filecache", "Memorja e skedarëve u azhornua");
        m.insert("... %d%% done ...", "... %d%% u krye ...");
        m.insert("Sunday", "E djelë");
        m.insert("Monday", "E hënë");
        m.insert("Tuesday", "E martë");
        m.insert("Wednesday", "E mërkurë");
        m.insert("Thursday", "E enjte");
        m.insert("Friday", "E premte");
        m.insert("Saturday", "E shtunë");
        m.insert("January", "Janar");
        m.insert("February", "Shkurt");
        m.insert("March", "Mars");
        m.insert("April", "Prill");
        m.insert("May", "Maj");
        m.insert("June", "Qershor");
        m.insert("July", "Korrik");
        m.insert("August", "Gusht");
        m.insert("September", "Shtator");
        m.insert("October", "Tetor");
        m.insert("November", "Nëntor");
        m.insert("December", "Dhjetor");
        m.insert("Settings", "Parametra");
        m.insert("seconds ago", "sekonda më parë");
        m.insert("today", "sot");
        m.insert("yesterday", "dje");
        m.insert("last month", "muajin e shkuar");
        m.insert("months ago", "muaj më parë");
        m.insert("last year", "vitin e shkuar");
        m.insert("years ago", "vite më parë");
        m.insert("Choose", "Zgjidh");
        m.insert("Yes", "Po");
        m.insert("No", "Jo");
        m.insert("Ok", "Në rregull");
        m.insert("Cancel", "Anulo");
        m.insert("Shared", "Ndarë");
        m.insert("Share", "Nda");
        m.insert("Error", "Veprim i gabuar");
        m.insert("Error while sharing", "Veprim i gabuar gjatë ndarjes");
        m.insert("Error while unsharing", "Veprim i gabuar gjatë heqjes së ndarjes");
        m.insert("Error while changing permissions", "Veprim i gabuar gjatë ndryshimit të lejeve");
        m.insert("Shared with you and the group {group} by {owner}", "Ndarë me ju dhe me grupin {group} nga {owner}");
        m.insert("Shared with you by {owner}", "Ndarë me ju nga {owner}");
        m.insert("Password protect", "Mbro me kod");
        m.insert("Password", "Kodi");
        m.insert("Allow Public Upload", "Lejo Ngarkimin Publik");
        m.insert("Email link to person", "Dërgo email me lidhjen");
        m.insert("Send", "Dërgo");
        m.insert("Set expiration date", "Cakto datën e përfundimit");
        m.insert("Expiration date", "Data e përfundimit");
        m.insert("Share via email:", "Nda me email:");
        m.insert("No people found", "Nuk u gjet asnjë person");
        m.insert("group", "grupi");
        m.insert("Resharing is not allowed", "Rindarja nuk lejohet");
        m.insert("Shared in {item} with {user}", "Ndarë në {item} me {user}");
        m.insert("Unshare", "Hiq ndarjen");
        m.insert("can edit", "mund të ndryshosh");
        m.insert("access control", "kontrollimi i hyrjeve");
        m.insert("create", "krijo");
        m.insert("update", "azhurno");
        m.insert("delete", "elimino");
        m.insert("share", "nda");
        m.insert("Password protected", "Mbrojtur me kod");
        m.insert("Error unsetting expiration date", "Veprim i gabuar gjatë heqjes së datës së përfundimit");
        m.insert("Error setting expiration date", "Veprim i gabuar gjatë caktimit të datës së përfundimit");
        m.insert("Sending ...", "Duke dërguar...");
        m.insert("Email sent", "Email-i u dërgua");
        m.insert("The object type is not specified.", "Nuk është specifikuar tipi i objektit.");
        m.insert("Delete", "Elimino");
        m.insert("Add", "Shto");
        m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Azhurnimi dështoi. Ju lutemi njoftoni për këtë problem <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">komunitetin ownCloud</a>.");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "Azhurnimi u krye. Tani do t'ju kaloj tek ownCloud-i.");
        m.insert("%s password reset", "Kodi i %s -it u rivendos");
        m.insert("Use the following link to reset your password: {link}", "Përdorni lidhjen në vijim për të rivendosur kodin: {link}");
        m.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Lidhja për rivendosjen e kodit tuaj u dërgua tek email-i juaj.<br>Nëqoftëse nuk e merrni brenda një kohe të arsyeshme, kontrolloni dosjet e postës së padëshirueshme (spam).<br>Nëqoftëse nuk është as aty, pyesni administratorin tuaj lokal.");
        m.insert("Request failed!<br>Did you make sure your email/username was right?", "Kërkesa dështoi!<br>A u siguruat që email-i/përdoruesi juaj ishte i saktë?");
        m.insert("You will receive a link to reset your password via Email.", "Do t'iu vijë një email që përmban një lidhje për ta rivendosur kodin.");
        m.insert("Username", "Përdoruesi");
        m.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Skedarët tuaj janë të kodifikuar. Nëqoftëse nuk keni aktivizuar çelësin e restaurimit, të dhënat tuaja nuk do të jenë të arritshme pasi të keni rivendosur kodin. Nëqoftëse nuk jeni i sigurt, ju lutemi kontaktoni administratorin tuaj para se të vazhdoni. Jeni i sigurt që dëshironi të vazhdoni?");
        m.insert("Yes, I really want to reset my password now", "Po, dua ta rivendos kodin tani");
        m.insert("Your password was reset", "Kodi yt u rivendos");
        m.insert("To login page", "Tek faqja e hyrjes");
        m.insert("New password", "Kodi i ri");
        m.insert("Reset password", "Rivendos kodin");
        m.insert("Personal", "Personale");
        m.insert("Users", "Përdoruesit");
        m.insert("Apps", "App");
        m.insert("Admin", "Admin");
        m.insert("Help", "Ndihmë");
        m.insert("Access forbidden", "Ndalohet hyrja");
        m.insert("Cloud not found", "Cloud-i nuk u gjet");
        m.insert("Security Warning", "Paralajmërim sigurie");
        m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Versioni juaj i PHP-së është i cënueshëm nga sulmi NULL Byte (CVE-2006-7243)");
        m.insert("Please update your PHP installation to use %s securely.", "Ju lutem azhurnoni instalimin tuaj të PHP-së që të përdorni %s -in në mënyrë të sigurt.");
        m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Nuk disponohet asnjë krijues numrash të rastësishëm, ju lutem aktivizoni shtesën PHP OpenSSL.");
        m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Pa një krijues numrash të rastësishëm të sigurt një person i huaj mund të jetë në gjendje të parashikojë kodin dhe të marri llogarinë tuaj.");
        m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Dosja dhe skedarët e të dhënave tuaja mbase janë të arritshme nga interneti sepse skedari .htaccess nuk po punon.");
        m.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Për më shumë informacion mbi konfigurimin e duhur të serverit tuaj, ju lutem shikoni <a href=\"%s\" target=\"_blank\">dokumentacionin</a>.");
        m.insert("Create an <strong>admin account</strong>", "Krijo një <strong>llogari administruesi</strong>");
        m.insert("Advanced", "Të përparuara");
        m.insert("Data folder", "Emri i dosjes");
        m.insert("Configure the database", "Konfiguro database-in");
        m.insert("will be used", "do të përdoret");
        m.insert("Database user", "Përdoruesi i database-it");
        m.insert("Database password", "Kodi i database-it");
        m.insert("Database name", "Emri i database-it");
        m.insert("Database tablespace", "Tablespace-i i database-it");
        m.insert("Database host", "Pozicioni (host) i database-it");
        m.insert("Finish setup", "Mbaro setup-in");
        m.insert("%s is available. Get more information on how to update.", "%s është i disponueshëm. Merrni më shumë informacione mbi azhurnimin.");
        m.insert("Log out", "Dalje");
        m.insert("Automatic logon rejected!", "Hyrja automatike u refuzua!");
        m.insert("If you did not change your password recently, your account may be compromised!", "Nqse nuk keni ndryshuar kodin kohët e fundit, llogaria juaj mund të jetë komprometuar.");
        m.insert("Please change your password to secure your account again.", "Ju lutemi, ndryshoni kodin për ta siguruar përsëri llogarinë tuaj.");
        m.insert("Lost your password?", "Ke humbur kodin?");
        m.insert("remember", "kujto");
        m.insert("Log in", "Hyrje");
        m.insert("Alternative Logins", "Hyrje alternative");
        m.insert("Updating ownCloud to version %s, this may take a while.", "Po azhurnoj ownCloud-in me versionin %s. Mund të zgjasi pak.");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["%n minut më parë", "%n minuta më parë"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["%n orë më parë", "%n orë më parë"]);
        m.insert("_%n day ago_::_%n days ago_", vec!["%n ditë më parë", "%n ditë më parë"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["%n muaj më parë", "%n muaj më parë"]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
        m
    };
}

pub fn get_plural_form(count: i64) -> usize {
    if count != 1 {
        return 1;
    }
    0
}

pub fn translate(text: &str) -> &'static str {
    TRANSLATIONS.get(text).unwrap_or(&text)
}

pub fn translate_plural(text: &str, count: i64) -> &'static str {
    if let Some(forms) = PLURAL_FORMS.get(text) {
        let form_index = get_plural_form(count);
        if form_index < forms.len() {
            return forms[form_index];
        }
    }
    text
}