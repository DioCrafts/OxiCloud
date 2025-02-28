use std::collections::HashMap;
use rust_i18n::i18n;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Sunday", "Nedelja");
    translations.insert("Monday", "Ponedeljak");
    translations.insert("Tuesday", "Utorak");
    translations.insert("Wednesday", "Sreda");
    translations.insert("Thursday", "Četvrtak");
    translations.insert("Friday", "Petak");
    translations.insert("Saturday", "Subota");
    translations.insert("January", "Januar");
    translations.insert("February", "Februar");
    translations.insert("March", "Mart");
    translations.insert("April", "April");
    translations.insert("May", "Maj");
    translations.insert("June", "Jun");
    translations.insert("July", "Jul");
    translations.insert("August", "Avgust");
    translations.insert("September", "Septembar");
    translations.insert("October", "Oktobar");
    translations.insert("November", "Novembar");
    translations.insert("December", "Decembar");
    translations.insert("Settings", "Podešavanja");
    translations.insert("seconds ago", "Pre par sekundi");
    translations.insert("today", "Danas");
    translations.insert("yesterday", "juče");
    translations.insert("last month", "prošlog meseca");
    translations.insert("months ago", "pre nekoliko meseci");
    translations.insert("last year", "prošle godine");
    translations.insert("years ago", "pre nekoliko godina");
    translations.insert("Choose", "Izaberi");
    translations.insert("Yes", "Da");
    translations.insert("No", "Ne");
    translations.insert("Ok", "Ok");
    translations.insert("Cancel", "Otkaži");
    translations.insert("Shared", "Deljeno");
    translations.insert("Share", "Podeli");
    translations.insert("Error", "Greška");
    translations.insert("Error while sharing", "Greška pri deljenju");
    translations.insert("Error while unsharing", "Greška u uklanjanju deljenja");
    translations.insert("Error while changing permissions", "Greška u promeni dozvola");
    translations.insert("Shared with you and the group {group} by {owner}", "{owner} podelio sa Vama i grupom {group} ");
    translations.insert("Shared with you by {owner}", "Sa vama podelio {owner}");
    translations.insert("Password protect", "Zaštita lozinkom");
    translations.insert("Password", "Lozinka");
    translations.insert("Email link to person", "Pošalji link e-mailom");
    translations.insert("Send", "Pošalji");
    translations.insert("Set expiration date", "Datum isteka");
    translations.insert("Expiration date", "Datum isteka");
    translations.insert("Share via email:", "Deli putem e-maila");
    translations.insert("No people found", "Nema pronađenih ljudi");
    translations.insert("Resharing is not allowed", "Dalje deljenje nije dozvoljeno");
    translations.insert("Shared in {item} with {user}", "Deljeno u {item} sa {user}");
    translations.insert("Unshare", "Ukljoni deljenje");
    translations.insert("can edit", "dozvoljene izmene");
    translations.insert("access control", "kontrola pristupa");
    translations.insert("create", "napravi");
    translations.insert("update", "ažuriranje");
    translations.insert("delete", "brisanje");
    translations.insert("share", "deljenje");
    translations.insert("Password protected", "Zaštćeno lozinkom");
    translations.insert("Error unsetting expiration date", "Greška u uklanjanju datuma isteka");
    translations.insert("Error setting expiration date", "Greška u postavljanju datuma isteka");
    translations.insert("Sending ...", "Slanje...");
    translations.insert("Email sent", "Email poslat");
    translations.insert("The object type is not specified.", "Tip objekta nije zadan.");
    translations.insert("Delete", "Obriši");
    translations.insert("Add", "Dodaj");
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Ažuriranje nije uspelo. Molimo obavestite <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud zajednicu</a>.");
    translations.insert("The update was successful. Redirecting you to ownCloud now.", "Ažuriranje je uspelo. Prosleđivanje na ownCloud.");
    translations.insert("Use the following link to reset your password: {link}", "Koristite sledeći link za reset lozinke: {link}");
    translations.insert("You will receive a link to reset your password via Email.", "Dobićete vezu za resetovanje lozinke putem e-pošte.");
    translations.insert("Username", "Korisničko ime");
    translations.insert("Your password was reset", "Vaša lozinka je resetovana");
    translations.insert("To login page", "Na login stranicu");
    translations.insert("New password", "Nova lozinka");
    translations.insert("Reset password", "Resetuj lozinku");
    translations.insert("Personal", "Lično");
    translations.insert("Users", "Korisnici");
    translations.insert("Apps", "Programi");
    translations.insert("Admin", "Adninistracija");
    translations.insert("Help", "Pomoć");
    translations.insert("Access forbidden", "Pristup zabranjen");
    translations.insert("Cloud not found", "Oblak nije nađen");
    translations.insert("Security Warning", "Bezbednosno upozorenje");
    translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Vaša PHP verzija je ranjiva na ");
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Nije dostupan generator slučajnog broja, molimo omogućite PHP OpenSSL ekstenziju.");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Bez generatora slučajnog broja napadač može predvideti token za reset lozinke i preuzeti Vaš nalog.");
    translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Vaši podaci i direktorijumi su verovatno dostupni sa interneta jer .htaccess fajl ne funkcioniše.");
    translations.insert("Create an <strong>admin account</strong>", "Napravi <strong>administrativni nalog</strong>");
    translations.insert("Advanced", "Napredno");
    translations.insert("Data folder", "Fascikla podataka");
    translations.insert("Configure the database", "Podešavanje baze");
    translations.insert("will be used", "će biti korišćen");
    translations.insert("Database user", "Korisnik baze");
    translations.insert("Database password", "Lozinka baze");
    translations.insert("Database name", "Ime baze");
    translations.insert("Database tablespace", "tablespace baze");
    translations.insert("Database host", "Domaćin baze");
    translations.insert("Finish setup", "Završi podešavanje");
    translations.insert("Log out", "Odjava");
    translations.insert("Automatic logon rejected!", "Automatsko logovanje odbijeno!");
    translations.insert("Lost your password?", "Izgubili ste lozinku?");
    translations.insert("remember", "upamti");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%n minute ago_::_%n minutes ago_", vec!["", "", ""]);
    plural_translations.insert("_%n hour ago_::_%n hours ago_", vec!["", "", ""]);
    plural_translations.insert("_%n day ago_::_%n days ago_", vec!["", "", ""]);
    plural_translations.insert("_%n month ago_::_%n months ago_", vec!["", "", ""]);
    plural_translations.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", "", ""]);
    plural_translations
}