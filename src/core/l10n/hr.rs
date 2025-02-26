use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Sunday", "nedelja");
    map.insert("Monday", "ponedeljak");
    map.insert("Tuesday", "utorak");
    map.insert("Wednesday", "srijeda");
    map.insert("Thursday", "četvrtak");
    map.insert("Friday", "petak");
    map.insert("Saturday", "subota");
    map.insert("January", "Siječanj");
    map.insert("February", "Veljača");
    map.insert("March", "Ožujak");
    map.insert("April", "Travanj");
    map.insert("May", "Svibanj");
    map.insert("June", "Lipanj");
    map.insert("July", "Srpanj");
    map.insert("August", "Kolovoz");
    map.insert("September", "Rujan");
    map.insert("October", "Listopad");
    map.insert("November", "Studeni");
    map.insert("December", "Prosinac");
    map.insert("Settings", "Postavke");
    map.insert("seconds ago", "sekundi prije");
    map.insert("today", "danas");
    map.insert("yesterday", "jučer");
    map.insert("last month", "prošli mjesec");
    map.insert("months ago", "mjeseci");
    map.insert("last year", "prošlu godinu");
    map.insert("years ago", "godina");
    map.insert("Choose", "Izaberi");
    map.insert("Yes", "Da");
    map.insert("No", "Ne");
    map.insert("Ok", "U redu");
    map.insert("Cancel", "Odustani");
    map.insert("Share", "Podijeli");
    map.insert("Error", "Greška");
    map.insert("Error while sharing", "Greška prilikom djeljenja");
    map.insert("Error while unsharing", "Greška prilikom isključivanja djeljenja");
    map.insert("Error while changing permissions", "Greška prilikom promjena prava");
    map.insert("Password protect", "Zaštiti lozinkom");
    map.insert("Password", "Lozinka");
    map.insert("Set expiration date", "Postavi datum isteka");
    map.insert("Expiration date", "Datum isteka");
    map.insert("Share via email:", "Dijeli preko email-a:");
    map.insert("No people found", "Osobe nisu pronađene");
    map.insert("Resharing is not allowed", "Ponovo dijeljenje nije dopušteno");
    map.insert("Unshare", "Makni djeljenje");
    map.insert("can edit", "može mjenjat");
    map.insert("access control", "kontrola pristupa");
    map.insert("create", "kreiraj");
    map.insert("update", "ažuriraj");
    map.insert("delete", "izbriši");
    map.insert("share", "djeli");
    map.insert("Password protected", "Zaštita lozinkom");
    map.insert("Error unsetting expiration date", "Greška prilikom brisanja datuma isteka");
    map.insert("Error setting expiration date", "Greška prilikom postavljanja datuma isteka");
    map.insert("Delete", "Obriši");
    map.insert("Add", "Dodaj");
    map.insert("Use the following link to reset your password: {link}", "Koristite ovaj link da biste poništili lozinku: {link}");
    map.insert("You will receive a link to reset your password via Email.", "Primit ćete link kako biste poništili zaporku putem e-maila.");
    map.insert("Username", "Korisničko ime");
    map.insert("Your password was reset", "Vaša lozinka je resetirana");
    map.insert("To login page", "Idi na stranicu za prijavu");
    map.insert("New password", "Nova lozinka");
    map.insert("Reset password", "Poništavanje lozinke");
    map.insert("Personal", "Osobno");
    map.insert("Users", "Korisnici");
    map.insert("Apps", "Aplikacije");
    map.insert("Admin", "Administrator");
    map.insert("Help", "Pomoć");
    map.insert("Access forbidden", "Pristup zabranjen");
    map.insert("Cloud not found", "Cloud nije pronađen");
    map.insert("Create an <strong>admin account</strong>", "Stvori <strong>administratorski račun</strong>");
    map.insert("Advanced", "Napredno");
    map.insert("Data folder", "Mapa baze podataka");
    map.insert("Configure the database", "Konfiguriraj bazu podataka");
    map.insert("will be used", "će se koristiti");
    map.insert("Database user", "Korisnik baze podataka");
    map.insert("Database password", "Lozinka baze podataka");
    map.insert("Database name", "Ime baze podataka");
    map.insert("Database tablespace", "Database tablespace");
    map.insert("Database host", "Poslužitelj baze podataka");
    map.insert("Finish setup", "Završi postavljanje");
    map.insert("Log out", "Odjava");
    map.insert("Lost your password?", "Izgubili ste lozinku?");
    map.insert("remember", "zapamtiti");
    map.insert("Log in", "Prijava");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2;";

pub struct PluralForms {
    pub minute: [&'static str; 3],
    pub hour: [&'static str; 3],
    pub day: [&'static str; 3],
    pub month: [&'static str; 3],
    pub file_conflict: [&'static str; 3],
}

pub static PLURAL_TRANSLATIONS: Lazy<PluralForms> = Lazy::new(|| {
    PluralForms {
        minute: ["", "", ""],
        hour: ["", "", ""],
        day: ["", "", ""],
        month: ["", "", ""],
        file_conflict: ["", "", ""],
    }
});