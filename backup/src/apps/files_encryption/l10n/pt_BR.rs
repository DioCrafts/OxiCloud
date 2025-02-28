use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Recuperação de chave habilitada com sucesso");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Impossível habilitar recuperação de chave. Por favor verifique sua senha para recuperação de chave!");
        m.insert("Recovery key successfully disabled", "Recuperação de chave desabilitada com sucesso");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Impossível desabilitar recuperação de chave. Por favor verifique sua senha para recuperação de chave!");
        m.insert("Password successfully changed.", "Senha alterada com sucesso.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Não foi possível alterar a senha. Talvez a senha antiga não estava correta.");
        m.insert("Private key password successfully updated.", "Senha de chave privada atualizada com sucesso.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Não foi possível atualizar a senha de chave privada. Talvez a senha antiga esteja incorreta.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Aplicativo de criptografia não foi inicializado! Talvez o aplicativo de criptografia tenha sido reativado durante essa sessão. Por favor, tente fazer logoff e login novamente para inicializar o aplicativo de criptografia.");
        m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "Sua chave privada não é válida! Provavelmente sua senha foi alterada fora de %s (por exemplo, seu diretório corporativo). Você pode atualizar sua senha de chave privada em suas configurações pessoais para recuperar o acesso a seus arquivos criptografados.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Este arquivo não pode ser decriptado, provavelmente este é um arquivo compartilhado. Poe favoe peça ao dono do arquivo para compartilha-lo com você.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Erro desconhecido, por favor verifique suas configurações ou faça contato com o administrador");
        m.insert("Missing requirements.", "Requisitos não encontrados.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Por favor, certifique-se que o PHP 5.3.3 ou mais recente está instalado e que a extensão PHP OpenSSL está habilitado e configurado corretamente. Por enquanto, o aplicativo de criptografia foi desativado.");
        m.insert("Following users are not set up for encryption:", "Seguintes usuários não estão configurados para criptografia:");
        m.insert("Saving...", "Salvando...");
        m.insert("Go directly to your ", "Ir diretamente para o seu");
        m.insert("personal settings", "configurações pessoais.");
        m.insert("Encryption", "Criptografia");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Habilitar chave de recuperação (permite recuperar arquivos de usuários em caso de perda de senha):");
        m.insert("Recovery key password", "Senha da chave de recuperação");
        m.insert("Repeat Recovery key password", "Repita Recuperação de senha da chave");
        m.insert("Enabled", "Habilitado");
        m.insert("Disabled", "Desabilitado");
        m.insert("Change recovery key password:", "Mudar a senha da chave de recuperação:");
        m.insert("Old Recovery key password", "Senha antiga da chave de recuperação");
        m.insert("New Recovery key password", "Nova senha da chave de recuperação");
        m.insert("Repeat New Recovery key password", "Repita Nova senha da chave de recuperação");
        m.insert("Change Password", "Trocar Senha");
        m.insert("Your private key password no longer match your log-in password:", "Sua senha de chave privada não coincide mais com sua senha de login:");
        m.insert("Set your old private key password to your current log-in password.", "Configure sua antiga senha de chave privada para sua atual senha de login.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Se você não se lembra de sua antiga senha você pode pedir ao administrador que recupere seus arquivos.");
        m.insert("Old log-in password", "Senha antiga de login");
        m.insert("Current log-in password", "Senha de login atual");
        m.insert("Update Private Key Password", "Atualizar senha de chave privada");
        m.insert("Enable password recovery:", "Habilitar recuperação de senha:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Habilitar essa opção vai permitir que você obtenha novamente acesso aos seus arquivos encriptados em caso de perda de senha");
        m.insert("File recovery settings updated", "Configurações de recuperação de arquivo atualizado");
        m.insert("Could not update file recovery", "Não foi possível atualizar a recuperação de arquivos");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}

/// Gets the translated string for the given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Gets the plural forms expression
pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}