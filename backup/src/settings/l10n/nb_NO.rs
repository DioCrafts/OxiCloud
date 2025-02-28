use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Unable to load list from App Store", "Lasting av liste fra App Store feilet.");
        map.insert("Authentication error", "Autentiseringsfeil");
        map.insert("Group already exists", "Gruppen finnes allerede");
        map.insert("Unable to add group", "Kan ikke legge til gruppe");
        map.insert("Email saved", "Epost lagret");
        map.insert("Invalid email", "Ugyldig epost");
        map.insert("Unable to delete group", "Kan ikke slette gruppe");
        map.insert("Unable to delete user", "Kan ikke slette bruker");
        map.insert("Language changed", "Språk endret");
        map.insert("Invalid request", "Ugyldig forespørsel");
        map.insert("Admins can't remove themself from the admin group", "Admin kan ikke flytte seg selv fra admingruppen");
        map.insert("Unable to add user to group %s", "Kan ikke legge bruker til gruppen %s");
        map.insert("Unable to remove user from group %s", "Kan ikke slette bruker fra gruppen %s");
        map.insert("Couldn't update app.", "Kunne ikke oppdatere app.");
        map.insert("Update to {appversion}", "Oppdater til {appversion}");
        map.insert("Disable", "Slå avBehandle ");
        map.insert("Enable", "Aktiver");
        map.insert("Please wait....", "Vennligst vent...");
        map.insert("Updating....", "Oppdaterer...");
        map.insert("Error while updating app", "Feil ved oppdatering av app");
        map.insert("Error", "Feil");
        map.insert("Update", "Oppdater");
        map.insert("Updated", "Oppdatert");
        map.insert("Saving...", "Lagrer...");
        map.insert("deleted", "slettet");
        map.insert("undo", "angre");
        map.insert("Unable to remove user", "Kunne ikke slette bruker");
        map.insert("Groups", "Grupper");
        map.insert("Group Admin", "Gruppeadministrator");
        map.insert("Delete", "Slett");
        map.insert("add group", "legg til gruppe");
        map.insert("A valid username must be provided", "Oppgi et gyldig brukernavn");
        map.insert("Error creating user", "Feil ved oppretting av bruker");
        map.insert("A valid password must be provided", "Oppgi et gyldig passord");
        map.insert("__language_name__", "__language_name__");
        map.insert("Security Warning", "Sikkerhetsadvarsel");
        map.insert("Setup Warning", "Installasjonsadvarsel");
        map.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Din nettservev er ikke konfigurert korrekt for filsynkronisering. WebDAV ser ut til å ikke funkere.");
        map.insert("Module 'fileinfo' missing", "Modulen 'fileinfo' mangler");
        map.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "PHP modulen 'fileinfo' mangler. Vi anbefaler at du aktiverer denne modulen for å kunne detektere mime-typen korrekt.");
        map.insert("Locale not working", "Språk virker ikke");
        map.insert("Internet connection not working", "Ingen internettilkopling");
        map.insert("Cron", "Cron");
        map.insert("Execute one task with each page loaded", "Utfør en oppgave med hver side som blir lastet");
        map.insert("Sharing", "Deling");
        map.insert("Enable Share API", "Aktiver API for Deling");
        map.insert("Allow apps to use the Share API", "Tillat apps å bruke API for Deling");
        map.insert("Allow links", "Tillat lenker");
        map.insert("Allow users to share items to the public with links", "Tillat brukere å dele filer med lenker");
        map.insert("Allow resharing", "TIllat videredeling");
        map.insert("Allow users to share items shared with them again", "Tillat brukere å dele filer som allerede har blitt delt med dem");
        map.insert("Allow users to share with anyone", "Tillat brukere å dele med alle");
        map.insert("Allow users to only share with users in their groups", "Tillat kun deling med andre brukere i samme gruppe");
        map.insert("Security", "Sikkerhet");
        map.insert("Enforce HTTPS", "Tving HTTPS");
        map.insert("Log", "Logg");
        map.insert("Log level", "Loggnivå");
        map.insert("More", "Mer");
        map.insert("Less", "Mindre");
        map.insert("Version", "Versjon");
        map.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Utviklet av<a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud sammfunnet</a>, <a href=\"https://github.com/owncloud\" target=\"_blank\">kildekoden</a> er lisensiert under <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
        map.insert("Add your App", "Legg til din App");
        map.insert("More Apps", "Flere Apps");
        map.insert("Select an App", "Velg en app");
        map.insert("See application page at apps.owncloud.com", "Se applikasjonens side på apps.owncloud.org");
        map.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-lisensiert av <span class=\"author\"></span>");
        map.insert("User Documentation", "Brukerdokumentasjon");
        map.insert("Administrator Documentation", "Administratordokumentasjon");
        map.insert("Online Documentation", "Online dokumentasjon");
        map.insert("Forum", "Forum");
        map.insert("Bugtracker", "Feilsporing");
        map.insert("Commercial Support", "Kommersiell støtte");
        map.insert("Get the apps to sync your files", "Få dine apps til å synkronisere dine filer");
        map.insert("Show First Run Wizard again", "Vis \"Førstegangs veiveiseren\" på nytt");
        map.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Du har brukt <strong>%s</strong> av tilgjengelig <strong>%s</strong>");
        map.insert("Password", "Passord");
        map.insert("Your password was changed", "Passord har blitt endret");
        map.insert("Unable to change your password", "Kunne ikke endre passordet ditt");
        map.insert("Current password", "Nåværende passord");
        map.insert("New password", "Nytt passord");
        map.insert("Change password", "Endre passord");
        map.insert("Email", "Epost");
        map.insert("Your email address", "Din e-postadresse");
        map.insert("Fill in an email address to enable password recovery", "Oppi epostadressen du vil tilbakestille passordet for");
        map.insert("Profile picture", "Profilbilde");
        map.insert("Language", "Språk");
        map.insert("Help translate", "Bidra til oversettelsen");
        map.insert("WebDAV", "WebDAV");
        map.insert("Encryption", "Kryptering");
        map.insert("Login Name", "Logginn navn");
        map.insert("Create", "Opprett");
        map.insert("Default Storage", "Standard lager");
        map.insert("Unlimited", "Ubegrenset");
        map.insert("Other", "Annet");
        map.insert("Username", "Brukernavn");
        map.insert("Storage", "Lager");
        map.insert("set new password", "sett nytt passord");
        map.insert("Default", "Standard");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}