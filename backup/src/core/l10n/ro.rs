use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("%s shared »%s« with you", "%s Partajat »%s« cu tine de");
    map.insert("Updated database", "Bază de date actualizată");
    map.insert("Unknown filetype", "Tip fișier necunoscut");
    map.insert("Invalid image", "Imagine invalidă");
    map.insert("Sunday", "Duminică");
    map.insert("Monday", "Luni");
    map.insert("Tuesday", "Marți");
    map.insert("Wednesday", "Miercuri");
    map.insert("Thursday", "Joi");
    map.insert("Friday", "Vineri");
    map.insert("Saturday", "Sâmbătă");
    map.insert("January", "Ianuarie");
    map.insert("February", "Februarie");
    map.insert("March", "Martie");
    map.insert("April", "Aprilie");
    map.insert("May", "Mai");
    map.insert("June", "Iunie");
    map.insert("July", "Iulie");
    map.insert("August", "August");
    map.insert("September", "Septembrie");
    map.insert("October", "Octombrie");
    map.insert("November", "Noiembrie");
    map.insert("December", "Decembrie");
    map.insert("Settings", "Setări");
    map.insert("seconds ago", "secunde în urmă");
    map.insert("today", "astăzi");
    map.insert("yesterday", "ieri");
    map.insert("last month", "ultima lună");
    map.insert("months ago", "luni în urmă");
    map.insert("last year", "ultimul an");
    map.insert("years ago", "ani în urmă");
    map.insert("Choose", "Alege");
    map.insert("Yes", "Da");
    map.insert("No", "Nu");
    map.insert("Ok", "Ok");
    map.insert("One file conflict", "Un conflict de fișier");
    map.insert("Which files do you want to keep?", "Ce fișiere vrei să păstrezi?");
    map.insert("If you select both versions, the copied file will have a number added to its name.", "Dacă alegi ambele versiuni, fișierul copiat va avea un număr atașat la denumirea sa.");
    map.insert("Cancel", "Anulare");
    map.insert("Continue", "Continuă");
    map.insert("Shared", "Partajat");
    map.insert("Share", "Partajează");
    map.insert("Error", "Eroare");
    map.insert("Error while sharing", "Eroare la partajare");
    map.insert("Error while unsharing", "Eroare la anularea partajării");
    map.insert("Error while changing permissions", "Eroare la modificarea permisiunilor");
    map.insert("Shared with you and the group {group} by {owner}", "Distribuie cu tine si grupul {group} de {owner}");
    map.insert("Shared with you by {owner}", "Distribuie cu tine de {owner}");
    map.insert("Password protect", "Protejare cu parolă");
    map.insert("Password", "Parolă");
    map.insert("Allow Public Upload", "Permiteţi încărcarea publică.");
    map.insert("Email link to person", "Expediază legătura prin poșta electronică");
    map.insert("Send", "Expediază");
    map.insert("Set expiration date", "Specifică data expirării");
    map.insert("Expiration date", "Data expirării");
    map.insert("Share via email:", "Distribuie prin email:");
    map.insert("No people found", "Nici o persoană găsită");
    map.insert("group", "grup");
    map.insert("Resharing is not allowed", "Repartajarea nu este permisă");
    map.insert("Shared in {item} with {user}", "Distribuie in {item} si {user}");
    map.insert("Unshare", "Anulare partajare");
    map.insert("can edit", "poate edita");
    map.insert("access control", "control acces");
    map.insert("create", "creare");
    map.insert("update", "actualizare");
    map.insert("delete", "ștergere");
    map.insert("share", "partajare");
    map.insert("Password protected", "Protejare cu parolă");
    map.insert("Error unsetting expiration date", "Eroare la anularea datei de expirare");
    map.insert("Error setting expiration date", "Eroare la specificarea datei de expirare");
    map.insert("Sending ...", "Se expediază...");
    map.insert("Email sent", "Mesajul a fost expediat");
    map.insert("Warning", "Atenție");
    map.insert("The object type is not specified.", "Tipul obiectului nu este specificat.");
    map.insert("Delete", "Șterge");
    map.insert("Add", "Adaugă");
    map.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Actualizarea a eșuat! Raportați problema către <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">comunitatea ownCloud</a>.");
    map.insert("The update was successful. Redirecting you to ownCloud now.", "Actualizare reușită. Ești redirecționat către ownCloud.");
    map.insert("Use the following link to reset your password: {link}", "Folosește următorul link pentru a reseta parola: {link}");
    map.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Linkul pentru resetarea parolei tale a fost trimis pe email. <br>Daca nu ai primit email-ul intr-un timp rezonabil, verifica folderul spam/junk. <br>Daca nu sunt acolo intreaba administratorul local.");
    map.insert("Request failed!<br>Did you make sure your email/username was right?", "Cerere esuata!<br>Esti sigur ca email-ul/numele de utilizator sunt corecte?");
    map.insert("You will receive a link to reset your password via Email.", "Vei primi un mesaj prin care vei putea reseta parola via email.");
    map.insert("Username", "Nume utilizator");
    map.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Fișierele tale sunt criptate. Dacă nu ai activat o cheie de recuperare, nu va mai exista nici o metodă prin care să îți recuperezi datele după resetarea parole. Dacă nu ești sigur în privința la ce ai de făcut, contactează un administrator înainte să continuii. Chiar vrei să continui?");
    map.insert("Yes, I really want to reset my password now", "Da, eu chiar doresc să îmi resetez parola acum");
    map.insert("Your password was reset", "Parola a fost resetată");
    map.insert("To login page", "Spre pagina de autentificare");
    map.insert("New password", "Noua parolă");
    map.insert("Reset password", "Resetează parola");
    map.insert("Personal", "Personal");
    map.insert("Users", "Utilizatori");
    map.insert("Apps", "Aplicații");
    map.insert("Admin", "Administrator");
    map.insert("Help", "Ajutor");
    map.insert("Access forbidden", "Acces restricționat");
    map.insert("Cloud not found", "Nu s-a găsit");
    map.insert("Security Warning", "Avertisment de securitate");
    map.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Versiunea dvs. PHP este vulnerabilă la un atac cu un octet NULL  (CVE-2006-7243)");
    map.insert("Please update your PHP installation to use %s securely.", "Te rog actualizează versiunea PHP pentru a utiliza %s în mod securizat.");
    map.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Nu este disponibil niciun generator securizat de numere aleatoare, vă rog activați extensia PHP OpenSSL.");
    map.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Fără generatorul securizat de numere aleatoare , un atacator poate anticipa simbolurile de resetare a parolei și poate prelua controlul asupra contului tău.");
    map.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Directorul tău de date și fișiere sunt probabil accesibile de pe Internet, deoarece fișierul .htaccess nu funcționează.");
    map.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Pentru informații despre cum să configurezi serverul, vezi <a href=\"%s\" target=\"_blank\">documentația</a>.");
    map.insert("Create an <strong>admin account</strong>", "Crează un <strong>cont de administrator</strong>");
    map.insert("Advanced", "Avansat");
    map.insert("Data folder", "Director date");
    map.insert("Configure the database", "Configurează baza de date");
    map.insert("will be used", "vor fi folosite");
    map.insert("Database user", "Utilizatorul bazei de date");
    map.insert("Database password", "Parola bazei de date");
    map.insert("Database name", "Numele bazei de date");
    map.insert("Database tablespace", "Tabela de spațiu a bazei de date");
    map.insert("Database host", "Bază date");
    map.insert("Finish setup", "Finalizează instalarea");
    map.insert("%s is available. Get more information on how to update.", "%s este disponibil. Vezi mai multe informații despre procesul de actualizare.");
    map.insert("Log out", "Ieșire");
    map.insert("Automatic logon rejected!", "Autentificare automată respinsă!");
    map.insert("If you did not change your password recently, your account may be compromised!", "Dacă nu ți-ai schimbat parola recent, contul tău ar putea fi compromis!");
    map.insert("Please change your password to secure your account again.", "Te rog schimbă-ți parola pentru a-ți securiza din nou contul.");
    map.insert("Lost your password?", "Ai uitat parola?");
    map.insert("remember", "amintește");
    map.insert("Log in", "Autentificare");
    map.insert("Alternative Logins", "Conectări alternative");
    map.insert("Updating ownCloud to version %s, this may take a while.", "Actualizăm ownCloud la versiunea %s, aceasta poate dura câteva momente.");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n==1?0:(((n%100>19)||((n%100==0)&&(n!=0)))?2:1));";

pub fn get_plural_translation(singular: &str, plural: &str, count: i64) -> String {
    match singular {
        "_%n minute ago_::_%n minutes ago_" => {
            if count == 1 {
                return format!("acum {} minut", count);
            } else {
                return format!("acum {} minute", count);
            }
        },
        "_%n hour ago_::_%n hours ago_" => {
            if count == 1 {
                return format!("acum {} oră", count);
            } else {
                return format!("acum {} ore", count);
            }
        },
        "_%n day ago_::_%n days ago_" => {
            if count == 1 {
                return format!("acum {} zi", count);
            } else {
                return format!("acum {} zile", count);
            }
        },
        "_%n month ago_::_%n months ago_" => "".to_string(),
        "_{count} file conflict_::_{count} file conflicts_" => "".to_string(),
        _ => format!("{} ({})", singular, count),
    }
}