use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Portuguese (Portugal) translations for ownCloud
pub static PT_PT_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "A aplicação \"%s\" não pode ser instaladas por não ser compatível com esta versão da ownCloud.");
    translations.insert("No app name specified", "O nome da aplicação não foi especificado");
    translations.insert("Help", "Ajuda");
    translations.insert("Personal", "Pessoal");
    translations.insert("Settings", "Configurações");
    translations.insert("Users", "Utilizadores");
    translations.insert("Admin", "Admin");
    translations.insert("Failed to upgrade \"%s\".", "A actualização \"%s\" falhou.");
    translations.insert("Unknown filetype", "Ficheiro desconhecido");
    translations.insert("Invalid image", "Imagem inválida");
    translations.insert("web services under your control", "serviços web sob o seu controlo");
    translations.insert("cannot open \"%s\"", "Não foi possível abrir \"%s\"");
    translations.insert("ZIP download is turned off.", "Descarregamento em ZIP está desligado.");
    translations.insert("Files need to be downloaded one by one.", "Os ficheiros precisam de ser descarregados um por um.");
    translations.insert("Back to Files", "Voltar a Ficheiros");
    translations.insert("Selected files too large to generate zip file.", "Os ficheiros seleccionados são grandes demais para gerar um ficheiro zip.");
    translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Descarregue os ficheiros em partes menores, separados ou peça gentilmente ao seu administrador.");
    translations.insert("Archives of type %s are not supported", "Arquivos do tipo %s não são suportados");
    translations.insert("App does not provide an info.xml file", "A aplicação não disponibiliza um ficheiro info.xml");
    translations.insert("App can't be installed because of not allowed code in the App", "A aplicação não pode ser instalado devido a código não permitido dentro da aplicação");
    translations.insert("App directory already exists", "A directoria da aplicação já existe");
    translations.insert("Can't create app folder. Please fix permissions. %s", "Não foi possível criar a pasta da aplicação. Por favor verifique as permissões. %s");
    translations.insert("Application is not enabled", "A aplicação não está activada");
    translations.insert("Authentication error", "Erro na autenticação");
    translations.insert("Token expired. Please reload page.", "O token expirou. Por favor recarregue a página.");
    translations.insert("Files", "Ficheiros");
    translations.insert("Text", "Texto");
    translations.insert("Images", "Imagens");
    translations.insert("%s enter the database username.", "%s introduza o nome de utilizador da base de dados");
    translations.insert("%s enter the database name.", "%s introduza o nome da base de dados");
    translations.insert("%s you may not use dots in the database name", "%s não é permitido utilizar pontos (.) no nome da base de dados");
    translations.insert("MS SQL username and/or password not valid: %s", "Nome de utilizador/password do MySQL é inválido: %s");
    translations.insert("You need to enter either an existing account or the administrator.", "Precisa de introduzir uma conta existente ou de administrador");
    translations.insert("MySQL username and/or password not valid", "Nome de utilizador/password do MySQL inválida");
    translations.insert("DB Error: \"%s\"", "Erro na BD: \"%s\"");
    translations.insert("Offending command was: \"%s\"", "O comando gerador de erro foi: \"%s\"");
    translations.insert("MySQL user '%s'@'localhost' exists already.", "O utilizador '%s'@'localhost' do MySQL já existe.");
    translations.insert("Drop this user from MySQL", "Eliminar este utilizador do MySQL");
    translations.insert("MySQL user '%s'@'%%' already exists", "O utilizador '%s'@'%%' do MySQL já existe");
    translations.insert("Drop this user from MySQL.", "Eliminar este utilizador do MySQL");
    translations.insert("Oracle connection could not be established", "Não foi possível estabelecer a ligação Oracle");
    translations.insert("Oracle username and/or password not valid", "Nome de utilizador/password do Oracle inválida");
    translations.insert("Offending command was: \"%s\", name: %s, password: %s", "O comando gerador de erro foi: \"%s\", nome: %s, password: %s");
    translations.insert("PostgreSQL username and/or password not valid", "Nome de utilizador/password do PostgreSQL inválido");
    translations.insert("Set an admin username.", "Definir um nome de utilizador de administrador");
    translations.insert("Set an admin password.", "Definiar uma password de administrador");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "O seu servidor web não está configurado correctamente para autorizar sincronização de ficheiros, pois o interface WebDAV parece estar com problemas.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Por favor verifique <a href='%s'>installation guides</a>.");
    translations.insert("Could not find category \"%s\"", "Não foi encontrado a categoria \"%s\"");
    translations.insert("seconds ago", "Minutos atrás");
    translations.insert("today", "hoje");
    translations.insert("yesterday", "ontem");
    translations.insert("last month", "ultímo mês");
    translations.insert("last year", "ano passado");
    translations.insert("years ago", "anos atrás");
    translations.insert("Caused by:", "Causado por:");
    
    translations
});

pub static PT_PT_PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Maps plural keys to their translations in Portuguese (Portugal)
pub static PT_PT_PLURAL_TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert("_%n minute ago_::_%n minutes ago_", vec!["", "%n minutos atrás"]);
    plural_translations.insert("_%n hour ago_::_%n hours ago_", vec!["", "%n horas atrás"]);
    plural_translations.insert("_%n day go_::_%n days ago_", vec!["", "%n dias atrás"]);
    plural_translations.insert("_%n month ago_::_%n months ago_", vec!["", "%n meses atrás"]);
    
    plural_translations
});