use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Chave de recuperação activada com sucesso");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Não foi possível activar a chave de recuperação. Por favor verifique a password da chave de recuperação!");
        m.insert("Recovery key successfully disabled", "Chave de recuperação descativada com sucesso");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Não foi possível desactivar a chave de recuperação. Por favor verifique a password da chave de recuperação.");
        m.insert("Password successfully changed.", "Password alterada com sucesso.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Não foi possivel alterar a password. Possivelmente a password antiga não está correcta.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Não foi possível alterar a chave. Possivelmente a password antiga não está correcta.");
        m.insert("Missing requirements.", "Faltam alguns requisitos.");
        m.insert("Following users are not set up for encryption:", "Os utilizadores seguintes não estão marcados para cifragem:");
        m.insert("Saving...", "A guardar...");
        m.insert("personal settings", "configurações personalizadas ");
        m.insert("Encryption", "Encriptação");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Active a chave de recuperação (permite recuperar os ficheiros no caso de perda da password):");
        m.insert("Recovery key password", "Chave de recuperação da conta");
        m.insert("Enabled", "Activado");
        m.insert("Disabled", "Desactivado");
        m.insert("Change recovery key password:", "Alterar a chave de recuperação:");
        m.insert("Old Recovery key password", "Chave anterior de recuperação da conta");
        m.insert("New Recovery key password", "Nova chave de recuperação da conta");
        m.insert("Change Password", "Mudar a Password");
        m.insert("Old log-in password", "Password anterior da conta");
        m.insert("Current log-in password", "Password actual da conta");
        m.insert("Enable password recovery:", "ativar recuperação do password:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Ao activar esta opção, tornar-lhe-a possível a obtenção de acesso aos seus ficheiros encriptados caso perca a password.");
        m.insert("File recovery settings updated", "Actualizadas as definições de recuperação de ficheiros");
        m.insert("Could not update file recovery", "Não foi possível actualizar a recuperação de ficheiros");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}