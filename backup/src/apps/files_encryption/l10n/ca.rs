use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Recovery key successfully enabled", "La clau de recuperació s'ha activat");
    m.insert("Could not enable recovery key. Please check your recovery key password!", "No s'ha pogut activar la clau de recuperació. Comproveu contrasenya de la clau de recuperació!");
    m.insert("Recovery key successfully disabled", "La clau de recuperació s'ha descativat");
    m.insert("Could not disable recovery key. Please check your recovery key password!", "No s'ha pogut desactivar la calu de recuperació. Comproveu la contrasenya de la clau de recuperació!");
    m.insert("Password successfully changed.", "La contrasenya s'ha canviat.");
    m.insert("Could not change the password. Maybe the old password was not correct.", "No s'ha pogut canviar la contrasenya. Potser la contrasenya anterior no era correcta.");
    m.insert("Private key password successfully updated.", "La contrasenya de la clau privada s'ha actualitzat.");
    m.insert("Could not update the private key password. Maybe the old password was not correct.", "No s'ha pogut actualitzar la contrasenya de la clau privada. Potser la contrasenya anterior no era correcta.");
    m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "L'aplicació d'encriptació  no està inicialitzada! Potser l'aplicació d'encriptació ha estat reiniciada durant la sessió. Intenteu sortir i acreditar-vos de nou per reinicialitzar l'aplicació d'encriptació.");
    m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "La clau privada no és vàlida! Probablement la contrasenya va ser canviada des de fora de %s (per exemple, en el directori de l'empresa). Vostè pot actualitzar la contrasenya de clau privada en la seva configuració personal per poder recuperar l'accés en els arxius xifrats.");
    m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "No es pot desencriptar aquest fitxer, probablement és un fitxer compartit. Demaneu al propietari del fitxer que el comparteixi de nou amb vós.");
    m.insert("Unknown error please check your system settings or contact your administrator", "Error desconegut. Comproveu l'arranjament del sistema o contacteu amb l'administrador");
    m.insert("Missing requirements.", "Manca de requisits.");
    m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Assegureu-vos que teniu instal·lat PHP 5.3.3 o una versió superior i que està activat Open SSL i habilitada i configurada correctament l'extensió de PHP. De moment, l'aplicació d'encriptació s'ha desactivat.");
    m.insert("Following users are not set up for encryption:", "Els usuaris següents no estan configurats per a l'encriptació:");
    m.insert("Saving...", "Desant...");
    m.insert("Go directly to your ", "Vés directament a");
    m.insert("personal settings", "arranjament personal");
    m.insert("Encryption", "Xifrat");
    m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Activa la clau de recuperació (permet recuperar fitxers d'usuaris en cas de pèrdua de contrasenya):");
    m.insert("Recovery key password", "Clau de recuperació de la contrasenya");
    m.insert("Repeat Recovery key password", "Repetiu la clau de recuperació de contrasenya");
    m.insert("Enabled", "Activat");
    m.insert("Disabled", "Desactivat");
    m.insert("Change recovery key password:", "Canvia la clau de recuperació de contrasenya:");
    m.insert("Old Recovery key password", "Antiga clau de recuperació de contrasenya");
    m.insert("New Recovery key password", "Nova clau de recuperació de contrasenya");
    m.insert("Repeat New Recovery key password", "Repetiu la nova clau de recuperació de contrasenya");
    m.insert("Change Password", "Canvia la contrasenya");
    m.insert("Your private key password no longer match your log-in password:", "La clau privada ja no es correspon amb la contrasenya d'accés:");
    m.insert("Set your old private key password to your current log-in password.", "Establiu la vostra contrasenya clau en funció de la contrasenya actual d'accés.");
    m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Si no recordeu la contrasenya anterior podeu demanar a l'administrador que recuperi els vostres fitxers.");
    m.insert("Old log-in password", "Contrasenya anterior d'accés");
    m.insert("Current log-in password", "Contrasenya d'accés actual");
    m.insert("Update Private Key Password", "Actualitza la contrasenya de clau privada");
    m.insert("Enable password recovery:", "Habilita la recuperació de contrasenya:");
    m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Activar aquesta opció us permetrà obtenir de nou accés als vostres fitxers encriptats en cas de perdre la contrasenya");
    m.insert("File recovery settings updated", "S'han  actualitzat els arranjaments de recuperació de fitxers");
    m.insert("Could not update file recovery", "No s'ha pogut actualitzar la recuperació de fitxers");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";