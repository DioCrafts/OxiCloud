use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%s shared »%s« with you", "%s delte »%s« med deg");
        m.insert("Sunday", "Søndag");
        m.insert("Monday", "Mandag");
        m.insert("Tuesday", "Tirsdag");
        m.insert("Wednesday", "Onsdag");
        m.insert("Thursday", "Torsdag");
        m.insert("Friday", "Fredag");
        m.insert("Saturday", "Lørdag");
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
        m.insert("Settings", "Innstillinger");
        m.insert("seconds ago", "sekunder siden");
        m.insert("today", "i dag");
        m.insert("yesterday", "i går");
        m.insert("last month", "forrige måned");
        m.insert("months ago", "måneder siden");
        m.insert("last year", "forrige år");
        m.insert("years ago", "år siden");
        m.insert("Choose", "Velg");
        m.insert("Yes", "Ja");
        m.insert("No", "Nei");
        m.insert("Ok", "Ok");
        m.insert("Cancel", "Avbryt");
        m.insert("Shared", "Delt");
        m.insert("Share", "Del");
        m.insert("Error", "Feil");
        m.insert("Error while sharing", "Feil under deling");
        m.insert("Shared with you by {owner}", "Delt med deg av {owner}");
        m.insert("Password protect", "Passordbeskyttet");
        m.insert("Password", "Passord");
        m.insert("Send", "Send");
        m.insert("Set expiration date", "Set utløpsdato");
        m.insert("Expiration date", "Utløpsdato");
        m.insert("Share via email:", "Del på epost");
        m.insert("No people found", "Ingen personer funnet");
        m.insert("group", "gruppe");
        m.insert("Unshare", "Avslutt deling");
        m.insert("can edit", "kan endre");
        m.insert("access control", "tilgangskontroll");
        m.insert("create", "opprett");
        m.insert("update", "oppdater");
        m.insert("delete", "slett");
        m.insert("share", "del");
        m.insert("Password protected", "Passordbeskyttet");
        m.insert("Error setting expiration date", "Kan ikke sette utløpsdato");
        m.insert("Sending ...", "Sender...");
        m.insert("Email sent", "E-post sendt");
        m.insert("Warning", "Advarsel");
        m.insert("Delete", "Slett");
        m.insert("Add", "Legg til");
        m.insert("Use the following link to reset your password: {link}", "Bruk følgende lenke for å tilbakestille passordet ditt: {link}");
        m.insert("You will receive a link to reset your password via Email.", "Du burde motta detaljer om å tilbakestille passordet ditt via epost.");
        m.insert("Username", "Brukernavn");
        m.insert("Your password was reset", "Passordet ditt ble tilbakestilt");
        m.insert("To login page", "Til innlogginssiden");
        m.insert("New password", "Nytt passord");
        m.insert("Reset password", "Tilbakestill passord");
        m.insert("Personal", "Personlig");
        m.insert("Users", "Brukere");
        m.insert("Apps", "Apper");
        m.insert("Admin", "Admin");
        m.insert("Help", "Hjelp");
        m.insert("Access forbidden", "Tilgang nektet");
        m.insert("Cloud not found", "Sky ikke funnet");
        m.insert("Security Warning", "Sikkerhetsadvarsel");
        m.insert("Create an <strong>admin account</strong>", "opprett en <strong>administrator-konto</strong>");
        m.insert("Advanced", "Avansert");
        m.insert("Data folder", "Datamappe");
        m.insert("Configure the database", "Konfigurer databasen");
        m.insert("will be used", "vil bli brukt");
        m.insert("Database user", "Databasebruker");
        m.insert("Database password", "Databasepassord");
        m.insert("Database name", "Databasenavn");
        m.insert("Database tablespace", "Database tabellområde");
        m.insert("Database host", "Databasevert");
        m.insert("Finish setup", "Fullfør oppsetting");
        m.insert("Log out", "Logg ut");
        m.insert("Automatic logon rejected!", "Automatisk pålogging avvist!");
        m.insert("If you did not change your password recently, your account may be compromised!", "Hvis du ikke har endret passordet ditt nylig kan kontoen din være kompromitert");
        m.insert("Please change your password to secure your account again.", "Vennligst skift passord for å gjøre kontoen din sikker igjen.");
        m.insert("Lost your password?", "Mistet passordet ditt?");
        m.insert("remember", "husk");
        m.insert("Log in", "Logg inn");
        m.insert("Updating ownCloud to version %s, this may take a while.", "Oppdaterer ownCloud til versjon %s, dette kan ta en stund.");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", ("", ""));
        m.insert("_%n hour ago_::_%n hours ago_", ("", ""));
        m.insert("_%n day ago_::_%n days ago_", ("", ""));
        m.insert("_%n month ago_::_%n months ago_", ("", ""));
        m.insert("_{count} file conflict_::_{count} file conflicts_", ("", ""));
        m
    };
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn translate_plural(key: &str, count: usize) -> &'static str {
    if let Some((singular, plural)) = PLURAL_FORMS.get(key) {
        if count == 1 {
            return if singular.is_empty() { key } else { singular };
        } else {
            return if plural.is_empty() { key } else { plural };
        }
    }
    key
}