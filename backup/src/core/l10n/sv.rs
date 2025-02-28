use std::collections::HashMap;
use std::collections::BTreeMap;
use phf::phf_map;

/// Swedish (sv) translation strings for ownCloud
pub struct SwedishTranslation {
    translations: HashMap<&'static str, &'static str>,
    plural_forms: &'static str,
    plural_translations: BTreeMap<&'static str, Vec<&'static str>>,
}

impl SwedishTranslation {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("%s shared »%s« with you", "%s delade »%s« med dig");
        translations.insert("Couldn't send mail to following users: %s ", "Gick inte att skicka e-post till följande användare: %s");
        translations.insert("Turned on maintenance mode", "Aktiverade underhållsläge");
        translations.insert("Turned off maintenance mode", "Deaktiverade underhållsläge");
        translations.insert("Updated database", "Uppdaterade databasen");
        translations.insert("Updating filecache, this may take really long...", "Uppdaterar filcache, det kan ta lång tid...");
        translations.insert("Updated filecache", "Uppdaterade filcache");
        translations.insert("... %d%% done ...", "... %d%% klart ...");
        translations.insert("No image or file provided", "Ingen bild eller fil har tillhandahållits");
        translations.insert("Unknown filetype", "Okänd filtyp");
        translations.insert("Invalid image", "Ogiltig bild");
        translations.insert("No temporary profile picture available, try again", "Ingen temporär profilbild finns tillgänglig, försök igen");
        translations.insert("No crop data provided", "Ingen beskärdata har angivits");
        translations.insert("Sunday", "Söndag");
        translations.insert("Monday", "Måndag");
        translations.insert("Tuesday", "Tisdag");
        translations.insert("Wednesday", "Onsdag");
        translations.insert("Thursday", "Torsdag");
        translations.insert("Friday", "Fredag");
        translations.insert("Saturday", "Lördag");
        translations.insert("January", "Januari");
        translations.insert("February", "Februari");
        translations.insert("March", "Mars");
        translations.insert("April", "April");
        translations.insert("May", "Maj");
        translations.insert("June", "Juni");
        translations.insert("July", "Juli");
        translations.insert("August", "Augusti");
        translations.insert("September", "September");
        translations.insert("October", "Oktober");
        translations.insert("November", "November");
        translations.insert("December", "December");
        translations.insert("Settings", "Inställningar");
        translations.insert("seconds ago", "sekunder sedan");
        translations.insert("today", "i dag");
        translations.insert("yesterday", "i går");
        translations.insert("last month", "förra månaden");
        translations.insert("months ago", "månader sedan");
        translations.insert("last year", "förra året");
        translations.insert("years ago", "år sedan");
        translations.insert("Choose", "Välj");
        translations.insert("Error loading file picker template: {error}", "Fel uppstod för filväljarmall: {error}");
        translations.insert("Yes", "Ja");
        translations.insert("No", "Nej");
        translations.insert("Ok", "Ok");
        translations.insert("Error loading message template: {error}", "Fel uppstod under inläsningen av meddelandemallen: {error}");
        translations.insert("One file conflict", "En filkonflikt");
        translations.insert("Which files do you want to keep?", "Vilken fil vill du behålla?");
        translations.insert("If you select both versions, the copied file will have a number added to its name.", "Om du väljer båda versionerna kommer de kopierade filerna ha nummer tillagda i filnamnet.");
        translations.insert("Cancel", "Avbryt");
        translations.insert("Continue", "Fortsätt");
        translations.insert("(all selected)", "(Alla valda)");
        translations.insert("Error loading file exists template", "Fel uppstod filmall existerar");
        translations.insert("Shared", "Delad");
        translations.insert("Share", "Dela");
        translations.insert("Error", "Fel");
        translations.insert("Error while sharing", "Fel vid delning");
        translations.insert("Error while unsharing", "Fel när delning skulle avslutas");
        translations.insert("Error while changing permissions", "Fel vid ändring av rättigheter");
        translations.insert("Shared with you and the group {group} by {owner}", "Delad med dig och gruppen {group} av {owner}");
        translations.insert("Shared with you by {owner}", "Delad med dig av {owner}");
        translations.insert("Share with user or group …", "Dela med användare eller grupp...");
        translations.insert("Share link", "Dela länk");
        translations.insert("Password protect", "Lösenordsskydda");
        translations.insert("Password", "Lösenord");
        translations.insert("Allow Public Upload", "Tillåt publik uppladdning");
        translations.insert("Email link to person", "E-posta länk till person");
        translations.insert("Send", "Skicka");
        translations.insert("Set expiration date", "Sätt utgångsdatum");
        translations.insert("Expiration date", "Utgångsdatum");
        translations.insert("Share via email:", "Dela via e-post:");
        translations.insert("No people found", "Hittar inga användare");
        translations.insert("group", "Grupp");
        translations.insert("Resharing is not allowed", "Dela vidare är inte tillåtet");
        translations.insert("Shared in {item} with {user}", "Delad i {item} med {user}");
        translations.insert("Unshare", "Sluta dela");
        translations.insert("notify by email", "informera via e-post");
        translations.insert("can edit", "kan redigera");
        translations.insert("access control", "åtkomstkontroll");
        translations.insert("create", "skapa");
        translations.insert("update", "uppdatera");
        translations.insert("delete", "radera");
        translations.insert("share", "dela");
        translations.insert("Password protected", "Lösenordsskyddad");
        translations.insert("Error unsetting expiration date", "Fel vid borttagning av utgångsdatum");
        translations.insert("Error setting expiration date", "Fel vid sättning av utgångsdatum");
        translations.insert("Sending ...", "Skickar ...");
        translations.insert("Email sent", "E-post skickat");
        translations.insert("Warning", "Varning");
        translations.insert("The object type is not specified.", "Objekttypen är inte specificerad.");
        translations.insert("Enter new", "Skriv nytt");
        translations.insert("Delete", "Radera");
        translations.insert("Add", "Lägg till");
        translations.insert("Edit tags", "Editera taggar");
        translations.insert("Error loading dialog template: {error}", "Fel under laddning utav dialog mall: {fel}");
        translations.insert("No tags selected for deletion.", "Inga taggar valda för borttagning.");
        translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Uppdateringen misslyckades. Rapportera detta problem till <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud Community</a>.");
        translations.insert("The update was successful. Redirecting you to ownCloud now.", "Uppdateringen lyckades. Du omdirigeras nu till OwnCloud.");
        translations.insert("%s password reset", "%s återställ lösenord");
        translations.insert("Use the following link to reset your password: {link}", "Använd följande länk för att återställa lösenordet: {link}");
        translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Länken för att återställa ditt lösenorden har skickats till din e-postadress<br>Om du inte har erhållit meddelandet inom kort, vänligen kontrollera din skräppost-mapp<br>Om den inte finns där, vänligen kontakta din administratör.");
        translations.insert("Request failed!<br>Did you make sure your email/username was right?", "Begäran misslyckades!<br>Är du helt säker på att din e-postadress/användarnamn är korrekt?");
        translations.insert("You will receive a link to reset your password via Email.", "Du får en länk att återställa ditt lösenord via e-post.");
        translations.insert("Username", "Användarnamn");
        translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Dina filer är krypterade. Om du inte har aktiverat återställningsnyckeln kommer det inte att finnas någon möjlighet att få tillbaka dina filer efter att ditt lösenord har återställts. Om du är osäker, kontakta din systemadministratör innan du fortsätter. Är du verkligen säker på att fortsätta?");
        translations.insert("Yes, I really want to reset my password now", "Ja, jag vill verkligen återställa mitt lösenord nu");
        translations.insert("Reset", "Återställ");
        translations.insert("Your password was reset", "Ditt lösenord har återställts");
        translations.insert("To login page", "Till logginsidan");
        translations.insert("New password", "Nytt lösenord");
        translations.insert("Reset password", "Återställ lösenordet");
        translations.insert("Personal", "Personligt");
        translations.insert("Users", "Användare");
        translations.insert("Apps", "Program");
        translations.insert("Admin", "Admin");
        translations.insert("Help", "Hjälp");
        translations.insert("Error loading tags", "Fel vid laddning utav taggar");
        translations.insert("Tag already exists", "Tagg existerar redan");
        translations.insert("Error deleting tag(s)", "Fel vid borttagning utav tagg(ar)");
        translations.insert("Error tagging", "Fel taggning");
        translations.insert("Error untagging", "Fel av taggning");
        translations.insert("Error favoriting", "Fel favorisering");
        translations.insert("Error unfavoriting", "Fel av favorisering ");
        translations.insert("Access forbidden", "Åtkomst förbjuden");
        translations.insert("Cloud not found", "Hittade inget moln");
        translations.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n", "Hej där,⏎\n⏎\nville bara meddela dig att %s delade %s med dig.⏎\nTitta på den: %s⏎\n⏎\n");
        translations.insert("The share will expire on %s.\n\n", "Utdelningen kommer att upphöra %s.⏎\n⏎\n");
        translations.insert("Cheers!", "Vi höres!");
        translations.insert("Security Warning", "Säkerhetsvarning");
        translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Din version av PHP är sårbar för NULL byte attack (CVE-2006-7243)");
        translations.insert("Please update your PHP installation to use %s securely.", "Var god uppdatera din PHP-installation för att använda %s säkert.");
        translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Ingen säker slumptalsgenerator finns tillgänglig. Du bör aktivera PHP OpenSSL-tillägget.");
        translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Utan en säker slumptalsgenerator kan angripare få möjlighet att förutsäga lösenordsåterställningar och ta över ditt konto.");
        translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Din datakatalog och filer är förmodligen tillgängliga från Internet, eftersom .htaccess-filen inte fungerar.");
        translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "För information hur du korrekt konfigurerar din servern, se ownCloud <a href=\"%s\" target=\"_blank\">dokumentationen</a>.");
        translations.insert("Create an <strong>admin account</strong>", "Skapa ett <strong>administratörskonto</strong>");
        translations.insert("Advanced", "Avancerad");
        translations.insert("Data folder", "Datamapp");
        translations.insert("Configure the database", "Konfigurera databasen");
        translations.insert("will be used", "kommer att användas");
        translations.insert("Database user", "Databasanvändare");
        translations.insert("Database password", "Lösenord till databasen");
        translations.insert("Database name", "Databasnamn");
        translations.insert("Database tablespace", "Databas tabellutrymme");
        translations.insert("Database host", "Databasserver");
        translations.insert("Finish setup", "Avsluta installation");
        translations.insert("Finishing …", "Avslutar ...");
        translations.insert("%s is available. Get more information on how to update.", "%s är tillgänglig. Få mer information om hur du går tillväga för att uppdatera.");
        translations.insert("Log out", "Logga ut");
        translations.insert("Automatic logon rejected!", "Automatisk inloggning inte tillåten!");
        translations.insert("If you did not change your password recently, your account may be compromised!", "Om du inte har ändrat ditt lösenord nyligen så kan ditt konto vara manipulerat!");
        translations.insert("Please change your password to secure your account again.", "Ändra genast lösenord för att säkra ditt konto.");
        translations.insert("Server side authentication failed!", "Servern misslyckades med autentisering!");
        translations.insert("Please contact your administrator.", "Kontakta din administratör.");
        translations.insert("Lost your password?", "Glömt ditt lösenord?");
        translations.insert("remember", "kom ihåg");
        translations.insert("Log in", "Logga in");
        translations.insert("Alternative Logins", "Alternativa inloggningar");
        translations.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>", "Hej där,<br><br>ville bara informera dig om att %s delade »%s« med dig.<br><a href=\"%s\">Titta på den!</a><br><br>");
        translations.insert("The share will expire on %s.<br><br>", "Utdelningen kommer att upphöra %s.<br><br>");
        translations.insert("Updating ownCloud to version %s, this may take a while.", "Uppdaterar ownCloud till version %s, detta kan ta en stund.");
        translations.insert("This ownCloud instance is currently being updated, which may take a while.", "Denna ownCloud instans håller på att uppdatera, vilket kan ta ett tag.");
        translations.insert("Please reload this page after a short time to continue using ownCloud.", "Var god och ladda om denna sida efter en kort stund för att fortsätta använda ownCloud.");
        translations.insert("Contact your system administrator if this message persists or appeared unexpectedly.", "Hör av dig till din system administratör ifall detta meddelande fortsätter eller visas oväntat.");
        translations.insert("Thank you for your patience.", "Tack för ditt tålamod.");

        let mut plural_translations = BTreeMap::new();
        plural_translations.insert("_%n minute ago_::_%n minutes ago_", vec!["%n minut sedan", "%n minuter sedan"]);
        plural_translations.insert("_%n hour ago_::_%n hours ago_", vec!["%n timme sedan", "%n timmar sedan"]);
        plural_translations.insert("_%n day ago_::_%n days ago_", vec!["%n dag sedan", "%n dagar sedan"]);
        plural_translations.insert("_%n month ago_::_%n months ago_", vec!["%n månad sedan", "%n månader sedan"]);
        plural_translations.insert("_{count} file conflict_::_{count} file conflicts_", vec!["{count} filkonflikt", "{count} filkonflikter"]);
        
        Self {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);",
            plural_translations,
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.translations.get(key).copied()
    }

    pub fn get_plural(&self, key: &str, count: usize) -> Option<&str> {
        let plural_index = if count != 1 { 1 } else { 0 };
        self.plural_translations.get(key).and_then(|forms| forms.get(plural_index).copied())
    }

    pub fn get_plural_form(&self) -> &str {
        self.plural_forms
    }
}

/// Create a default instance of the Swedish translation
pub fn get_swedish_translation() -> SwedishTranslation {
    SwedishTranslation::new()
}