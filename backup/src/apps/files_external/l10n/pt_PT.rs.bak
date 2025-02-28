use once_cell::sync::Lazy;
use std::collections::HashMap;

// Portuguese (Portugal) translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    m.insert("Access granted", "Acesso autorizado");
    m.insert("Error configuring Dropbox storage", "Erro ao configurar o armazenamento do Dropbox");
    m.insert("Grant access", "Conceder acesso");
    m.insert("Please provide a valid Dropbox app key and secret.", "Por favor forneça uma \"app key\" e \"secret\" do Dropbox válidas.");
    m.insert("Error configuring Google Drive storage", "Erro ao configurar o armazenamento do Google Drive");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Atenção:</b> O cliente \"smbclient\" não está instalado. Não é possível montar as partilhas CIFS/SMB . Peça ao seu administrador para instalar.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Aviso:</b> O suporte FTP no PHP não está activate ou instalado. Não é possível montar as partilhas FTP. Peça ao seu administrador para instalar.");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Atenção:<br> O suporte PHP para o Curl não está activado ou instalado. A montagem do ownCloud/WebDav ou GoolgeDriver não é possível. Por favor contacte o administrador para o instalar.");
    m.insert("External Storage", "Armazenamento Externo");
    m.insert("Folder name", "Nome da pasta");
    m.insert("External storage", "Armazenamento Externo");
    m.insert("Configuration", "Configuração");
    m.insert("Options", "Opções");
    m.insert("Applicable", "Aplicável");
    m.insert("Add storage", "Adicionar armazenamento");
    m.insert("None set", "Não definido");
    m.insert("All Users", "Todos os utilizadores");
    m.insert("Groups", "Grupos");
    m.insert("Users", "Utilizadores");
    m.insert("Delete", "Eliminar");
    m.insert("Enable User External Storage", "Activar Armazenamento Externo para o Utilizador");
    m.insert("Allow users to mount their own external storage", "Permitir que os utilizadores montem o seu próprio armazenamento externo");
    m.insert("SSL root certificates", "Certificados SSL de raiz");
    m.insert("Import Root Certificate", "Importar Certificado Root");
    
    m
});

// Plural forms for Portuguese (Portugal)
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    PLURAL_FORMS
}