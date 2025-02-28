use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Deletion failed", "Ștergerea a eșuat");
        map.insert("Success", "Succes");
        map.insert("Error", "Eroare");
        map.insert("Save", "Salvează");
        map.insert("Help", "Ajutor");
        map.insert("Host", "Gazdă");
        map.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", 
                   "Puteți omite protocolul, decât dacă folosiți SSL. Atunci se începe cu ldaps://");
        map.insert("Port", "Portul");
        map.insert("User DN", "DN al utilizatorului");
        map.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", 
                   "DN-ul clientului utilizator cu care se va efectua conectarea, d.e. uid=agent,dc=example,dc=com. Pentru acces anonim, lăsăți DN și Parolă libere.");
        map.insert("Password", "Parolă");
        map.insert("For anonymous access, leave DN and Password empty.", 
                   "Pentru acces anonim, lăsați DN și Parolă libere.");
        map.insert("One Base DN per line", "Un Base DN pe linie");
        map.insert("You can specify Base DN for users and groups in the Advanced tab", 
                   "Puteți să specificați DN de bază pentru utilizatori și grupuri în fila Avansat");
        map.insert("Back", "Înapoi");
        map.insert("Continue", "Continuă");
        map.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", 
                   "<b>Atenție</b> Modulul PHP LDAP nu este instalat, infrastructura nu va funcționa. Contactează administratorul sistemului pentru al instala.");
        map.insert("User Login Filter", "Filtrare după Nume Utilizator");
        map.insert("Case insensitve LDAP server (Windows)", 
                   "Server LDAP insensibil la majuscule (Windows)");
        map.insert("Turn off SSL certificate validation.", 
                   "Oprește validarea certificatelor SSL ");
        map.insert("in seconds. A change empties the cache.", 
                   "în secunde. O schimbare curăță memoria tampon.");
        map.insert("User Display Name Field", "Câmpul cu numele vizibil al utilizatorului");
        map.insert("Base User Tree", "Arborele de bază al Utilizatorilor");
        map.insert("One User Base DN per line", "Un User Base DN pe linie");
        map.insert("Group Display Name Field", "Câmpul cu numele grupului");
        map.insert("Base Group Tree", "Arborele de bază al Grupurilor");
        map.insert("One Group Base DN per line", "Un Group Base DN pe linie");
        map.insert("Group-Member association", "Asocierea Grup-Membru");
        map.insert("in bytes", "în octeți");
        map.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", 
                   "Lăsați gol pentru numele de utilizator (implicit). În caz contrar, specificați un atribut LDAP / AD.");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1?0:(((n%100>19)||((n%100==0)&&(n!=0)))?2:1));";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("_%s group found_::_%s groups found_", vec!["", "", ""]);
        map.insert("_%s user found_::_%s users found_", vec!["", "", ""]);
        map
    };
}

// Función para obtener una traducción
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

// Función para obtener una traducción plural
pub fn get_plural_translation(key: &str, count: usize) -> Option<&'static str> {
    let forms = PLURAL_TRANSLATIONS.get(key)?;
    
    // Aplicar la fórmula de plural que estaba en PHP
    let index = if count == 1 {
        0
    } else if (count % 100 > 19) || ((count % 100 == 0) && (count != 0)) {
        2
    } else {
        1
    };
    
    forms.get(index).copied()
}