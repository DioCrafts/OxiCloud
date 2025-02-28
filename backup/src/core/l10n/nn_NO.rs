use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%s shared »%s« with you", "%s delte «%s» med deg");
        m.insert("Turned on maintenance mode", "Skrudde på vedlikehaldsmodus");
        m.insert("Turned off maintenance mode", "Skrudde av vedlikehaldsmodus");
        m.insert("Updated database", "Database oppdatert");
        m.insert("Updating filecache, this may take really long...", "Oppdaterer mellomlager; dette kan ta ei god stund …");
        m.insert("Updated filecache", "Mellomlager oppdatert");
        m.insert("... %d%% done ...", "… %d %% ferdig …");
        m.insert("No image or file provided", "Inga bilete eller fil gitt");
        m.insert("Unknown filetype", "Ukjend filtype");
        m.insert("Invalid image", "Ugyldig bilete");
        m.insert("No temporary profile picture available, try again", "Inga midlertidig profilbilete tilgjengeleg, prøv igjen");
        m.insert("No crop data provided", "Ingen beskjeringsdata gitt");
        m.insert("Sunday", "Søndag");
        m.insert("Monday", "Måndag");
        m.insert("Tuesday", "Tysdag");
        m.insert("Wednesday", "Onsdag");
        m.insert("Thursday", "Torsdag");
        m.insert("Friday", "Fredag");
        m.insert("Saturday", "Laurdag");
        m.insert("January", "Januar");
        m.insert("February", "Februar");
        m.insert("March", "Mars");
        m.insert("April", "April");
        m.insert("May", "Mai");
        m.insert("June", "Juni");
        m.insert("July", "Juli");
        m.insert("August", "August");
        m.insert("September", "September");
        m.insert("October", "Oktober");
        m.insert("November", "November");
        m.insert("December", "Desember");
        m.insert("Settings", "Innstillingar");
        m.insert("seconds ago", "sekund sidan");
        m.insert("today", "i dag");
        m.insert("yesterday", "i går");
        m.insert("last month", "førre månad");
        m.insert("months ago", "månadar sidan");
        m.insert("last year", "i fjor");
        m.insert("years ago", "år sidan");
        m.insert("Choose", "Vel");
        m.insert("Error loading file picker template: {error}", "Klarte ikkje å lasta filplukkarmal: {error}");
        m.insert("Yes", "Ja");
        m.insert("No", "Nei");
        m.insert("Ok", "Greitt");
        m.insert("Error loading message template: {error}", "Klarte ikkje å lasta meldingsmal: {error}");
        m.insert("One file conflict", "Éin filkonflikt");
        m.insert("Which files do you want to keep?", "Kva filer vil du spara?");
        m.insert("If you select both versions, the copied file will have a number added to its name.", "Viss du vel begge utgåvene, vil den kopierte fila få eit tal lagt til namnet.");
        m.insert("Cancel", "Avbryt");
        m.insert("Continue", "Gå vidare");
        m.insert("(all selected)", "(alle valte)");
        m.insert("({count} selected)", "({count} valte)");
        m.insert("Error loading file exists template", "Klarte ikkje å lasta fil-finst-mal");
        m.insert("Shared", "Delt");
        m.insert("Share", "Del");
        m.insert("Error", "Feil");
        m.insert("Error while sharing", "Feil ved deling");
        m.insert("Error while unsharing", "Feil ved udeling");
        m.insert("Error while changing permissions", "Feil ved endring av tillatingar");
        m.insert("Shared with you and the group {group} by {owner}", "Delt med deg og gruppa {group} av {owner}");
        m.insert("Shared with you by {owner}", "Delt med deg av {owner}");
        m.insert("Password protect", "Passordvern");
        m.insert("Password", "Passord");
        m.insert("Allow Public Upload", "Tillat offentleg opplasting");
        m.insert("Email link to person", "Send lenkja over e-post");
        m.insert("Send", "Send");
        m.insert("Set expiration date", "Set utløpsdato");
        m.insert("Expiration date", "Utløpsdato");
        m.insert("Share via email:", "Del over e-post:");
        m.insert("No people found", "Fann ingen personar");
        m.insert("group", "gruppe");
        m.insert("Resharing is not allowed", "Vidaredeling er ikkje tillate");
        m.insert("Shared in {item} with {user}", "Delt i {item} med {brukar}");
        m.insert("Unshare", "Udel");
        m.insert("can edit", "kan endra");
        m.insert("access control", "tilgangskontroll");
        m.insert("create", "lag");
        m.insert("update", "oppdater");
        m.insert("delete", "slett");
        m.insert("share", "del");
        m.insert("Password protected", "Passordverna");
        m.insert("Error unsetting expiration date", "Klarte ikkje fjerna utløpsdato");
        m.insert("Error setting expiration date", "Klarte ikkje setja utløpsdato");
        m.insert("Sending ...", "Sender …");
        m.insert("Email sent", "E-post sendt");
        m.insert("Warning", "Åtvaring");
        m.insert("The object type is not specified.", "Objekttypen er ikkje spesifisert.");
        m.insert("Delete", "Slett");
        m.insert("Add", "Legg til");
        m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Oppdateringa feila. Ver venleg og rapporter feilen til <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud-fellesskapet</a>.");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "Oppdateringa er fullført. Sender deg vidare til ownCloud no.");
        m.insert("%s password reset", "%s passordnullstilling");
        m.insert("Use the following link to reset your password: {link}", "Klikk følgjande lenkje til å nullstilla passordet ditt: {link}");
        m.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Lenkja til å nullstilla passordet med er sendt til e-posten din.<br>Sjå i spam-/søppelmappa di viss du ikkje ser e-posten innan rimeleg tid.<br>Spør din lokale administrator viss han ikkje er der heller.");
        m.insert("Request failed!<br>Did you make sure your email/username was right?", "Førespurnaden feila!<br>Er du viss på at du skreiv inn rett e-post/brukarnamn?");
        m.insert("You will receive a link to reset your password via Email.", "Du vil få ein e-post med ei lenkje for å nullstilla passordet.");
        m.insert("Username", "Brukarnamn");
        m.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Filene dine er krypterte. Viss du ikkje har skrudd på gjenopprettingsnøkkelen, finst det ingen måte å få tilbake dataa dine når passordet ditt er nullstilt. Viss du ikkje er sikker på kva du skal gjera bør du spørja administratoren din før du går vidare. Vil du verkeleg fortsetja?");
        m.insert("Yes, I really want to reset my password now", "Ja, eg vil nullstilla passordet mitt no");
        m.insert("Your password was reset", "Passordet ditt er nullstilt");
        m.insert("To login page", "Til innloggingssida");
        m.insert("New password", "Nytt passord");
        m.insert("Reset password", "Nullstill passord");
        m.insert("Personal", "Personleg");
        m.insert("Users", "Brukarar");
        m.insert("Apps", "Program");
        m.insert("Admin", "Admin");
        m.insert("Help", "Hjelp");
        m.insert("Access forbidden", "Tilgang forbudt");
        m.insert("Cloud not found", "Fann ikkje skyen");
        m.insert("Security Warning", "Tryggleiksåtvaring");
        m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "PHP-utgåva di er sårbar for NULL-byteåtaket (CVE-2006-7243)");
        m.insert("Please update your PHP installation to use %s securely.", "Ver venleg og oppdater PHP-installasjonen din til å brukar %s trygt.");
        m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Ingen tilgjengeleg tilfeldig nummer-generator, ver venleg og aktiver OpenSSL-utvidinga i PHP.");
        m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Utan ein trygg tilfeldig nummer-generator er det enklare for ein åtakar å gjetta seg fram til passordnullstillingskodar og dimed ta over kontoen din.");
        m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Datamappa og filene dine er sannsynlegvis tilgjengelege frå Internett sidan .htaccess-fila ikkje fungerer.");
        m.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Ver venleg og les <a href=\"%s\" target=\"_blank\">dokumentasjonen</a> for meir informasjon om korleis du konfigurerer tenaren din.");
        m.insert("Create an <strong>admin account</strong>", "Lag ein <strong>admin-konto</strong>");
        m.insert("Advanced", "Avansert");
        m.insert("Data folder", "Datamappe");
        m.insert("Configure the database", "Set opp databasen");
        m.insert("will be used", "vil verta nytta");
        m.insert("Database user", "Databasebrukar");
        m.insert("Database password", "Databasepassord");
        m.insert("Database name", "Databasenamn");
        m.insert("Database tablespace", "Tabellnamnrom for database");
        m.insert("Database host", "Databasetenar");
        m.insert("Finish setup", "Fullfør oppsettet");
        m.insert("%s is available. Get more information on how to update.", "%s er tilgjengeleg. Få meir informasjon om korleis du oppdaterer.");
        m.insert("Log out", "Logg ut");
        m.insert("Automatic logon rejected!", "Automatisk innlogging avvist!");
        m.insert("If you did not change your password recently, your account may be compromised!", "Viss du ikkje endra passordet ditt nyleg, så kan kontoen din vera kompromittert!");
        m.insert("Please change your password to secure your account again.", "Ver venleg og endra passordet for å gjera kontoen din trygg igjen.");
        m.insert("Lost your password?", "Gløymt passordet?");
        m.insert("remember", "hugs");
        m.insert("Log in", "Logg inn");
        m.insert("Alternative Logins", "Alternative innloggingar");
        m.insert("Updating ownCloud to version %s, this may take a while.", "Oppdaterer ownCloud til utgåve %s, dette kan ta ei stund.");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["%n minutt sidan", "%n minutt sidan"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["%n time sidan", "%n timar sidan"]);
        m.insert("_%n day ago_::_%n days ago_", vec!["%n dag sidan", "%n dagar sidan"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["%n månad sidan", "%n månadar sidan"]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["{count} filkonflikt", "{count} filkonfliktar"]);
        m
    };
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn translate(key: &str) -> String {
    TRANSLATIONS.get(key).unwrap_or(&key).to_string()
}

pub fn translate_plural(key: &str, count: i32) -> String {
    if let Some(forms) = PLURAL_FORMS.get(key) {
        let form_index = if count != 1 { 1 } else { 0 };
        return forms.get(form_index).unwrap_or(&key).to_string();
    }
    key.to_string()
}