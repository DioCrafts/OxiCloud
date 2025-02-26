use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to clear the mappings.", "Fel vid rensning av mappningar");
        m.insert("Failed to delete the server configuration", "Misslyckades med att radera serverinställningen");
        m.insert("The configuration is valid and the connection could be established!", "Inställningen är giltig och anslutningen kunde upprättas!");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Konfigurationen är riktig, men Bind felade. Var vänlig och kontrollera serverinställningar och logininformation.");
        m.insert("No action specified", "Ingen åtgärd har angetts");
        m.insert("No configuration specified", "Ingen konfiguration har angetts");
        m.insert("No data specified", "Ingen data har angetts");
        m.insert(" Could not set configuration %s", "Kunde inte sätta inställning %s");
        m.insert("Deletion failed", "Raderingen misslyckades");
        m.insert("Take over settings from recent server configuration?", "Ta över inställningar från tidigare serverkonfiguration?");
        m.insert("Keep settings?", "Behåll inställningarna?");
        m.insert("Cannot add server configuration", "Kunde inte lägga till serverinställning");
        m.insert("mappings cleared", "mappningar rensade");
        m.insert("Success", "Lyckat");
        m.insert("Error", "Fel");
        m.insert("Select groups", "Välj grupper");
        m.insert("Select object classes", "Välj Objekt-klasser");
        m.insert("Select attributes", "Välj attribut");
        m.insert("Connection test succeeded", "Anslutningstestet lyckades");
        m.insert("Connection test failed", "Anslutningstestet misslyckades");
        m.insert("Do you really want to delete the current Server Configuration?", "Vill du verkligen radera den nuvarande serverinställningen?");
        m.insert("Confirm Deletion", "Bekräfta radering");
        m.insert("_%s group found_::_%s groups found_", "%s grupp hittad|%s grupper hittade");
        m.insert("_%s user found_::_%s users found_", "%s användare hittad|%s användare hittade");
        m.insert("Invalid Host", "Felaktig Host");
        m.insert("Could not find the desired feature", "Det gick inte hitta den önskade funktionen");
        m.insert("Save", "Spara");
        m.insert("Test Configuration", "Testa konfigurationen");
        m.insert("Help", "Hjälp");
        m.insert("Limit the access to %s to groups meeting this criteria:", "Begränsa åtkomsten till %s till grupper som möter följande kriterie:");
        m.insert("only those object classes:", "Endast de objekt-klasserna:");
        m.insert("only from those groups:", "endast ifrån de här grupperna:");
        m.insert("The filter specifies which LDAP groups shall have access to the %s instance.", "Filtret specifierar vilka LDAD-grupper som ska ha åtkomst till %s instans");
        m.insert("groups found", "grupper hittade");
        m.insert("What attribute shall be used as login name:", "Vilket attribut ska användas som login namn:");
        m.insert("LDAP Username:", "LDAP användarnamn:");
        m.insert("LDAP Email Address:", "LDAP e-postadress:");
        m.insert("Other Attributes:", "Övriga attribut:");
        m.insert("Add Server Configuration", "Lägg till serverinställning");
        m.insert("Host", "Server");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Du behöver inte ange protokoll förutom om du använder SSL. Starta då med ldaps://");
        m.insert("Port", "Port");
        m.insert("User DN", "Användare DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN för användaren som skall användas, t.ex. uid=agent, dc=example, dc=com. För anonym åtkomst, lämna DN och lösenord tomt.");
        m.insert("Password", "Lösenord");
        m.insert("For anonymous access, leave DN and Password empty.", "För anonym åtkomst, lämna DN och lösenord tomt.");
        m.insert("One Base DN per line", "Ett Start DN per rad");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "Du kan ange start DN för användare och grupper under fliken Avancerat");
        m.insert("Limit the access to %s to users meeting this criteria:", "Begränsa åtkomsten till %s till användare som möter följande kriterie:");
        m.insert("The filter specifies which LDAP users shall have access to the %s instance.", "Filtret specifierar vilka LDAP-användare som skall ha åtkomst till %s instans");
        m.insert("users found", "användare funna");
        m.insert("Back", "Tillbaka");
        m.insert("Continue", "Fortsätt");
        m.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Varning:</b> Apps user_ldap och user_webdavauth är inkompatibla. Oväntade problem kan uppstå. Be din systemadministratör att inaktivera en av dom.");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Varning:</b> PHP LDAP - modulen är inte installerad, serversidan kommer inte att fungera. Kontakta din systemadministratör för installation.");
        m.insert("Connection Settings", "Uppkopplingsinställningar");
        m.insert("Configuration Active", "Konfiguration aktiv");
        m.insert("When unchecked, this configuration will be skipped.", "Ifall denna är avbockad så kommer konfigurationen att skippas.");
        m.insert("User Login Filter", "Filter logga in användare");
        m.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Definierar filter som tillämpas vid inloggning. %%uid ersätter användarnamn vid inloggningen. Exempel: \"uid=%%uid\"");
        m.insert("Backup (Replica) Host", "Säkerhetskopierings-värd (Replika)");
        m.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Ange en valfri värd för säkerhetskopiering. Den måste vara en replika av den huvudsakliga LDAP/AD-servern");
        m.insert("Backup (Replica) Port", "Säkerhetskopierins-port (Replika)");
        m.insert("Disable Main Server", "Inaktivera huvudserver");
        m.insert("Only connect to the replica server.", "Anslut endast till replikaservern.");
        m.insert("Case insensitve LDAP server (Windows)", "LDAP-servern är okänslig för gemener och versaler (Windows)");
        m.insert("Turn off SSL certificate validation.", "Stäng av verifiering av SSL-certifikat.");
        m.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Rekommenderas inte, använd endast för test! Om anslutningen bara fungerar med denna inställning behöver du importera LDAP-serverns SSL-certifikat till din %s server.");
        m.insert("Cache Time-To-Live", "Cache Time-To-Live");
        m.insert("in seconds. A change empties the cache.", "i sekunder. En förändring tömmer cache.");
        m.insert("Directory Settings", "Mappinställningar");
        m.insert("User Display Name Field", "Attribut för användarnamn");
        m.insert("The LDAP attribute to use to generate the user's display name.", "LDAP-attributet som ska användas för att generera användarens visningsnamn.");
        m.insert("Base User Tree", "Bas för användare i katalogtjänst");
        m.insert("One User Base DN per line", "En Användare start DN per rad");
        m.insert("User Search Attributes", "Användarsökningsattribut");
        m.insert("Optional; one attribute per line", "Valfritt; ett attribut per rad");
        m.insert("Group Display Name Field", "Attribut för gruppnamn");
        m.insert("The LDAP attribute to use to generate the groups's display name.", "LDAP-attributet som ska användas för att generera gruppens visningsnamn.");
        m.insert("Base Group Tree", "Bas för grupper i katalogtjänst");
        m.insert("One Group Base DN per line", "En Grupp start DN per rad");
        m.insert("Group Search Attributes", "Gruppsökningsattribut");
        m.insert("Group-Member association", "Attribut för gruppmedlemmar");
        m.insert("Special Attributes", "Specialattribut");
        m.insert("Quota Field", "Kvotfält");
        m.insert("Quota Default", "Datakvot standard");
        m.insert("in bytes", "i bytes");
        m.insert("Email Field", "E-postfält");
        m.insert("User Home Folder Naming Rule", "Namnregel för hemkatalog");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Lämnas tomt för användarnamn (standard). Ange annars ett LDAP/AD-attribut.");
        m.insert("Internal Username", "Internt Användarnamn");
        m.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Som standard skapas det interna användarnamnet från UUID-attributet. Det säkerställer att användarnamnet är unikt och tecken inte behöver konverteras. Det interna användarnamnet har restriktionerna att endast följande tecken är tillåtna:  [ a-zA-Z0-9_.@- ]. Andra tecken blir ersatta av deras motsvarighet i ASCII eller utelämnas helt. En siffra kommer att läggas till eller ökas på vid en kollision. Det interna användarnamnet används för att identifiera användaren internt. Det är även förvalt som användarens användarnamn i ownCloud. Det är även en port för fjärråtkomst, t.ex. för alla *DAV-tjänster. Med denna inställning kan det förvalda beteendet åsidosättas. För att uppnå ett liknande beteende som innan ownCloud 5, ange attributet för användarens visningsnamn i detta fält. Lämna det tomt för förvalt beteende. Ändringarna kommer endast att påverka nyligen mappade (tillagda) LDAP-användare");
        m.insert("Internal Username Attribute:", "Internt Användarnamn Attribut:");
        m.insert("Override UUID detection", "Åsidosätt UUID detektion");
        m.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Som standard upptäcker ownCloud automatiskt UUID-attributet. Det UUID-attributet används för att utan tvivel identifiera LDAP-användare och grupper. Dessutom kommer interna användarnamn skapas baserat på detta UUID, om inte annat anges ovan. Du kan åsidosätta inställningen och passera ett attribut som du själv väljer. Du måste se till att attributet som du väljer kan hämtas för både användare och grupper och att det är unikt. Lämna det tomt för standard beteende. Förändringar kommer endast att påverka nyligen mappade (tillagda) LDAP-användare och grupper.");
        m.insert("UUID Attribute for Users:", "UUID Attribut för Användare:");
        m.insert("UUID Attribute for Groups:", "UUID Attribut för Grupper:");
        m.insert("Username-LDAP User Mapping", "Användarnamn-LDAP User Mapping");
        m.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "ownCloud använder sig av användarnamn för att lagra och tilldela (meta) data. För att exakt kunna identifiera och känna igen användare, kommer varje LDAP-användare ha ett internt användarnamn. Detta kräver en mappning från ownCloud-användarnamn till LDAP-användare. Det skapade användarnamnet mappas till UUID för LDAP-användaren. Dessutom cachas DN  samt minska LDAP-interaktionen, men den används inte för identifiering. Om DN förändras, kommer förändringarna hittas av ownCloud. Det interna ownCloud-namnet används överallt i ownCloud. Om du rensar/raderar mappningarna kommer att lämna referenser överallt i systemet. Men den är inte konfigurationskänslig, den påverkar alla LDAP-konfigurationer! Rensa/radera aldrig mappningarna i en produktionsmiljö. Utan gör detta endast på i testmiljö!");
        m.insert("Clear Username-LDAP User Mapping", "Rensa Användarnamn-LDAP User Mapping");
        m.insert("Clear Groupname-LDAP Group Mapping", "Rensa Gruppnamn-LDAP Group Mapping");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Function to get translation
pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

// Function to handle plurals (simplified version)
pub fn translate_plural(key: &str, count: usize) -> String {
    if let Some(translations) = TRANSLATIONS.get(key) {
        let parts: Vec<&str> = translations.split('|').collect();
        if count == 1 && parts.len() > 0 {
            return parts[0].replace("%s", &count.to_string());
        } else if parts.len() > 1 {
            return parts[1].replace("%s", &count.to_string());
        }
    }
    key.to_string()
}