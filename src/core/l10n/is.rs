use std::collections::HashMap;
use rust_i18n::t;

pub fn get_is_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Sunday", "Sunnudagur");
    translations.insert("Monday", "Mánudagur");
    translations.insert("Tuesday", "Þriðjudagur");
    translations.insert("Wednesday", "Miðvikudagur");
    translations.insert("Thursday", "Fimmtudagur");
    translations.insert("Friday", "Föstudagur");
    translations.insert("Saturday", "Laugardagur");
    translations.insert("January", "Janúar");
    translations.insert("February", "Febrúar");
    translations.insert("March", "Mars");
    translations.insert("April", "Apríl");
    translations.insert("May", "Maí");
    translations.insert("June", "Júní");
    translations.insert("July", "Júlí");
    translations.insert("August", "Ágúst");
    translations.insert("September", "September");
    translations.insert("October", "Október");
    translations.insert("November", "Nóvember");
    translations.insert("December", "Desember");
    translations.insert("Settings", "Stillingar");
    translations.insert("seconds ago", "sek.");
    translations.insert("today", "í dag");
    translations.insert("yesterday", "í gær");
    translations.insert("last month", "síðasta mánuði");
    translations.insert("months ago", "mánuðir síðan");
    translations.insert("last year", "síðasta ári");
    translations.insert("years ago", "einhverjum árum");
    translations.insert("Choose", "Veldu");
    translations.insert("Yes", "Já");
    translations.insert("No", "Nei");
    translations.insert("Ok", "Í lagi");
    translations.insert("Cancel", "Hætta við");
    translations.insert("Shared", "Deilt");
    translations.insert("Share", "Deila");
    translations.insert("Error", "Villa");
    translations.insert("Error while sharing", "Villa við deilingu");
    translations.insert("Error while unsharing", "Villa við að hætta deilingu");
    translations.insert("Error while changing permissions", "Villa við að breyta aðgangsheimildum");
    translations.insert("Shared with you and the group {group} by {owner}", "Deilt með þér og hópnum {group} af {owner}");
    translations.insert("Shared with you by {owner}", "Deilt með þér af {owner}");
    translations.insert("Password protect", "Verja með lykilorði");
    translations.insert("Password", "Lykilorð");
    translations.insert("Email link to person", "Senda vefhlekk í tölvupóstu til notenda");
    translations.insert("Send", "Senda");
    translations.insert("Set expiration date", "Setja gildistíma");
    translations.insert("Expiration date", "Gildir til");
    translations.insert("Share via email:", "Deila með tölvupósti:");
    translations.insert("No people found", "Engir notendur fundust");
    translations.insert("Resharing is not allowed", "Endurdeiling er ekki leyfð");
    translations.insert("Shared in {item} with {user}", "Deilt með {item} ásamt {user}");
    translations.insert("Unshare", "Hætta deilingu");
    translations.insert("can edit", "getur breytt");
    translations.insert("access control", "aðgangsstýring");
    translations.insert("create", "mynda");
    translations.insert("update", "uppfæra");
    translations.insert("delete", "eyða");
    translations.insert("share", "deila");
    translations.insert("Password protected", "Verja með lykilorði");
    translations.insert("Error unsetting expiration date", "Villa við að aftengja gildistíma");
    translations.insert("Error setting expiration date", "Villa við að setja gildistíma");
    translations.insert("Sending ...", "Sendi ...");
    translations.insert("Email sent", "Tölvupóstur sendur");
    translations.insert("Warning", "Aðvörun");
    translations.insert("The object type is not specified.", "Tegund ekki tilgreind");
    translations.insert("Delete", "Eyða");
    translations.insert("Add", "Bæta við");
    translations.insert("The update was successful. Redirecting you to ownCloud now.", "Uppfærslan heppnaðist. Beini þér til ownCloud nú.");
    translations.insert("Use the following link to reset your password: {link}", "Notað eftirfarandi veftengil til að endursetja lykilorðið þitt: {link}");
    translations.insert("You will receive a link to reset your password via Email.", "Þú munt fá veftengil í tölvupósti til að endursetja lykilorðið.");
    translations.insert("Username", "Notendanafn");
    translations.insert("Your password was reset", "Lykilorðið þitt hefur verið endursett.");
    translations.insert("To login page", "Fara á innskráningarsíðu");
    translations.insert("New password", "Nýtt lykilorð");
    translations.insert("Reset password", "Endursetja lykilorð");
    translations.insert("Personal", "Um mig");
    translations.insert("Users", "Notendur");
    translations.insert("Apps", "Forrit");
    translations.insert("Admin", "Stjórnun");
    translations.insert("Help", "Hjálp");
    translations.insert("Access forbidden", "Aðgangur bannaður");
    translations.insert("Cloud not found", "Ský finnst ekki");
    translations.insert("Security Warning", "Öryggis aðvörun");
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Enginn traustur slembitölugjafi í boði, vinsamlegast virkjaðu PHP OpenSSL viðbótina.");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Án öruggs slembitölugjafa er mögulegt að sjá fyrir öryggis auðkenni til að endursetja lykilorð og komast inn á aðganginn þinn.");
    translations.insert("Create an <strong>admin account</strong>", "Útbúa <strong>vefstjóra aðgang</strong>");
    translations.insert("Advanced", "Ítarlegt");
    translations.insert("Data folder", "Gagnamappa");
    translations.insert("Configure the database", "Stilla gagnagrunn");
    translations.insert("will be used", "verður notað");
    translations.insert("Database user", "Gagnagrunns notandi");
    translations.insert("Database password", "Gagnagrunns lykilorð");
    translations.insert("Database name", "Nafn gagnagrunns");
    translations.insert("Database tablespace", "Töflusvæði gagnagrunns");
    translations.insert("Database host", "Netþjónn gagnagrunns");
    translations.insert("Finish setup", "Virkja uppsetningu");
    translations.insert("%s is available. Get more information on how to update.", "%s er til boða. Fáðu meiri upplýsingar um hvernig þú uppfærir.");
    translations.insert("Log out", "Útskrá");
    translations.insert("Automatic logon rejected!", "Sjálfvirkri innskráningu hafnað!");
    translations.insert("If you did not change your password recently, your account may be compromised!", "Ef þú breyttir ekki lykilorðinu þínu fyrir skömmu, er mögulegt að einhver annar hafi komist inn á aðganginn þinn.");
    translations.insert("Please change your password to secure your account again.", "Vinsamlegast breyttu lykilorðinu þínu til að tryggja öryggi þitt.");
    translations.insert("Lost your password?", "Týndir þú lykilorðinu?");
    translations.insert("remember", "muna eftir mér");
    translations.insert("Log in", "<strong>Skrá inn</strong>");
    translations.insert("Updating ownCloud to version %s, this may take a while.", "Uppfæri ownCloud í útgáfu %s, það gæti tekið smá stund.");
    
    translations
}

pub fn get_is_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Función para registrar las traducciones plurales específicas
pub fn register_is_plurals() {
    // Definición de plurales usando la biblioteca rust-i18n (ejemplo de implementación)
    t!("_%n minute ago_::_%n minutes ago_", ""); // Versión singular vacía
    t!("_%n minute ago_::_%n minutes ago_", ""); // Versión plural vacía
    
    t!("_%n hour ago_::_%n hours ago_", ""); // Versión singular vacía
    t!("_%n hour ago_::_%n hours ago_", ""); // Versión plural vacía
    
    t!("_%n day ago_::_%n days ago_", ""); // Versión singular vacía
    t!("_%n day ago_::_%n days ago_", ""); // Versión plural vacía
    
    t!("_%n month ago_::_%n months ago_", ""); // Versión singular vacía
    t!("_%n month ago_::_%n months ago_", ""); // Versión plural vacía
    
    t!("_{count} file conflict_::_{count} file conflicts_", ""); // Versión singular vacía
    t!("_{count} file conflict_::_{count} file conflicts_", ""); // Versión plural vacía
}