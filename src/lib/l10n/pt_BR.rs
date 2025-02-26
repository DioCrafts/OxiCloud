use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("pt_BR");

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "O aplicativo \"%s\" não pode ser instalado porque não é compatível com esta versão do ownCloud.");
    translations.insert("No app name specified", "O nome do aplicativo não foi especificado.");
    translations.insert("Help", "Ajuda");
    translations.insert("Personal", "Pessoal");
    translations.insert("Settings", "Ajustes");
    translations.insert("Users", "Usuários");
    translations.insert("Admin", "Admin");
    translations.insert("Failed to upgrade \"%s\".", "Falha na atualização de \"%s\".");
    translations.insert("Unknown filetype", "Tipo de arquivo desconhecido");
    translations.insert("Invalid image", "Imagem inválida");
    translations.insert("web services under your control", "serviços web sob seu controle");
    translations.insert("cannot open \"%s\"", "não pode abrir \"%s\"");
    translations.insert("ZIP download is turned off.", "Download ZIP está desligado.");
    translations.insert("Files need to be downloaded one by one.", "Arquivos precisam ser baixados um de cada vez.");
    translations.insert("Back to Files", "Voltar para Arquivos");
    translations.insert("Selected files too large to generate zip file.", "Arquivos selecionados são muito grandes para gerar arquivo zip.");
    translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Baixe os arquivos em pedaços menores, separadamente ou solicite educadamente  ao seu administrador.");
    translations.insert("No source specified when installing app", "Nenhuma fonte foi especificada enquanto instalava o aplicativo");
    translations.insert("No href specified when installing app from http", "Nenhuma href foi especificada enquanto instalava o aplicativo de httml");
    translations.insert("No path specified when installing app from local file", "Nenhum caminho foi especificado enquanto instalava o aplicativo do arquivo local");
    translations.insert("Archives of type %s are not supported", "Arquivos do tipo %s não são suportados");
    translations.insert("Failed to open archive when installing app", "Falha para abrir o arquivo enquanto instalava o aplicativo");
    translations.insert("App does not provide an info.xml file", "O aplicativo não fornece um arquivo info.xml");
    translations.insert("App can't be installed because of not allowed code in the App", "O aplicativo não pode ser instalado por causa do código não permitido no Aplivativo");
    translations.insert("App can't be installed because it is not compatible with this version of ownCloud", "O aplicativo não pode ser instalado porque não é compatível com esta versão do ownCloud");
    translations.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "O aplicativo não pode ser instalado porque ele contém a marca <shipped>verdadeiro</shipped> que não é permitido para aplicações não embarcadas");
    translations.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "O aplicativo não pode ser instalado porque a versão em info.xml /versão não é a mesma que a versão relatada na App Store");
    translations.insert("App directory already exists", "Diretório App  já existe");
    translations.insert("Can't create app folder. Please fix permissions. %s", "Não é possível criar pasta app. Corrija as permissões. %s");
    translations.insert("Application is not enabled", "Aplicação não está habilitada");
    translations.insert("Authentication error", "Erro de autenticação");
    translations.insert("Token expired. Please reload page.", "Token expirou. Por favor recarregue a página.");
    translations.insert("Files", "Arquivos");
    translations.insert("Text", "Texto");
    translations.insert("Images", "Imagens");
    translations.insert("%s enter the database username.", "%s insira o nome de usuário do banco de dados.");
    translations.insert("%s enter the database name.", "%s insira o nome do banco de dados.");
    translations.insert("%s you may not use dots in the database name", "%s você não pode usar pontos no nome do banco de dados");
    translations.insert("MS SQL username and/or password not valid: %s", "Nome de usuário e/ou senha MS SQL inválido(s): %s");
    translations.insert("You need to enter either an existing account or the administrator.", "Você precisa inserir uma conta existente ou o administrador.");
    translations.insert("MySQL username and/or password not valid", "Nome de usuário e/ou senha MySQL inválido(s)");
    translations.insert("DB Error: \"%s\"", "Erro no BD: \"%s\"");
    translations.insert("Offending command was: \"%s\"", "Comando ofensivo era: \"%s\"");
    translations.insert("MySQL user '%s'@'localhost' exists already.", "O usuário MySQL '%s'@'localhost' já existe.");
    translations.insert("Drop this user from MySQL", "Derrubar este usuário do MySQL");
    translations.insert("MySQL user '%s'@'%%' already exists", "Usuário MySQL '%s'@'%%' já existe");
    translations.insert("Drop this user from MySQL.", "Derrube este usuário do MySQL.");
    translations.insert("Oracle connection could not be established", "Conexão Oracle não pode ser estabelecida");
    translations.insert("Oracle username and/or password not valid", "Nome de usuário e/ou senha Oracle inválido(s)");
    translations.insert("Offending command was: \"%s\", name: %s, password: %s", "Comando ofensivo era: \"%s\", nome: %s, senha: %s");
    translations.insert("PostgreSQL username and/or password not valid", "Nome de usuário e/ou senha PostgreSQL inválido(s)");
    translations.insert("Set an admin username.", "Defina um nome de usuário de administrador.");
    translations.insert("Set an admin password.", "Defina uma senha de administrador.");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Seu servidor web não está configurado corretamente para permitir sincronização de arquivos porque a interface WebDAV parece estar quebrada.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Por favor, confira os <a href='%s'>guias de instalação</a>.");
    translations.insert("Could not find category \"%s\"", "Impossível localizar categoria \"%s\"");
    translations.insert("seconds ago", "segundos atrás");
    translations.insert("_%n minute ago_::_%n minutes ago_", "ha %n minutos");
    translations.insert("_%n hour ago_::_%n hours ago_", "ha %n horas");
    translations.insert("today", "hoje");
    translations.insert("yesterday", "ontem");
    translations.insert("_%n day go_::_%n days ago_", "ha %n dias");
    translations.insert("last month", "último mês");
    translations.insert("_%n month ago_::_%n months ago_", "ha %n meses");
    translations.insert("last year", "último ano");
    translations.insert("years ago", "anos atrás");
    translations.insert("Caused by:", "Causados ​​por:");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}