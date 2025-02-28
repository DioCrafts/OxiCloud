use std::collections::HashMap;
use once_cell::sync::Lazy;

// Portuguese (Portugal) translations
pub static PT_PT: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Failed to clear the mappings.", "Falhou a limpar os mapas");
    translations.insert("Failed to delete the server configuration", "Erro ao eliminar as configurações do servidor");
    translations.insert("The configuration is valid and the connection could be established!", "A configuração está correcta e foi possível estabelecer a ligação!");
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "A configuração está correcta, mas não foi possível estabelecer o \"laço\", por favor, verifique as configurações do servidor e as credenciais.");
    translations.insert("Deletion failed", "Erro ao apagar");
    translations.insert("Take over settings from recent server configuration?", "Assumir as configurações da configuração do servidor mais recente?");
    translations.insert("Keep settings?", "Manter as definições?");
    translations.insert("Cannot add server configuration", "Não foi possível adicionar as configurações do servidor.");
    translations.insert("mappings cleared", "Mapas limpos");
    translations.insert("Success", "Sucesso");
    translations.insert("Error", "Erro");
    translations.insert("Select groups", "Seleccionar grupos");
    translations.insert("Connection test succeeded", "Teste de conecção passado com sucesso.");
    translations.insert("Connection test failed", "Erro no teste de conecção.");
    translations.insert("Do you really want to delete the current Server Configuration?", "Deseja realmente apagar as configurações de servidor actuais?");
    translations.insert("Confirm Deletion", "Confirmar a operação de apagar");
    translations.insert("_%s group found_::_%s groups found_", "");
    translations.insert("_%s user found_::_%s users found_", "");
    translations.insert("Save", "Guardar");
    translations.insert("Test Configuration", "Testar a configuração");
    translations.insert("Help", "Ajuda");
    translations.insert("Add Server Configuration", "Adicionar configurações do servidor");
    translations.insert("Host", "Anfitrião");
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Pode omitir o protocolo, excepto se necessitar de SSL. Neste caso, comece com ldaps://");
    translations.insert("Port", "Porto");
    translations.insert("User DN", "DN do utilizador");
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "O DN to cliente ");
    translations.insert("Password", "Password");
    translations.insert("For anonymous access, leave DN and Password empty.", "Para acesso anónimo, deixe DN e a Palavra-passe vazios.");
    translations.insert("One Base DN per line", "Uma base DN por linho");
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", "Pode especificar o ND Base para utilizadores e grupos no separador Avançado");
    translations.insert("Back", "Voltar");
    translations.insert("Continue", "Continuar");
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Aviso:</b> O módulo PHP LDAP não está instalado, logo não irá funcionar. Por favor peça ao administrador para o instalar.");
    translations.insert("Connection Settings", "Definições de ligação");
    translations.insert("Configuration Active", "Configuração activa");
    translations.insert("When unchecked, this configuration will be skipped.", "Se não estiver marcada, esta definição não será tida em conta.");
    translations.insert("User Login Filter", "Filtro de login de utilizador");
    translations.insert("Backup (Replica) Host", "Servidor de Backup (Réplica)");
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Forneça um servidor (anfitrião) de backup. Deve ser uma réplica do servidor principal de LDAP/AD ");
    translations.insert("Backup (Replica) Port", "Porta do servidor de backup (Replica)");
    translations.insert("Disable Main Server", "Desactivar servidor principal");
    translations.insert("Case insensitve LDAP server (Windows)", "Servidor LDAP (Windows) não sensível a maiúsculas.");
    translations.insert("Turn off SSL certificate validation.", "Desligar a validação de certificado SSL.");
    translations.insert("Cache Time-To-Live", "Cache do tempo de vida dos objetos no servidor");
    translations.insert("in seconds. A change empties the cache.", "em segundos. Uma alteração esvazia a cache.");
    translations.insert("Directory Settings", "Definições de directorias");
    translations.insert("User Display Name Field", "Mostrador do nome de utilizador.");
    translations.insert("Base User Tree", "Base da árvore de utilizadores.");
    translations.insert("One User Base DN per line", "Uma base de utilizador DN por linha");
    translations.insert("User Search Attributes", "Utilizar atributos de pesquisa");
    translations.insert("Optional; one attribute per line", "Opcional; Um atributo por linha");
    translations.insert("Group Display Name Field", "Mostrador do nome do grupo.");
    translations.insert("Base Group Tree", "Base da árvore de grupos.");
    translations.insert("One Group Base DN per line", "Uma base de grupo DN por linha");
    translations.insert("Group Search Attributes", "Atributos de pesquisa de grupo");
    translations.insert("Group-Member association", "Associar utilizador ao grupo.");
    translations.insert("Special Attributes", "Atributos especiais");
    translations.insert("Quota Field", "Quota");
    translations.insert("Quota Default", "Quota padrão");
    translations.insert("in bytes", "em bytes");
    translations.insert("Email Field", "Campo de email");
    translations.insert("User Home Folder Naming Rule", "Regra da pasta inicial do utilizador");
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Deixe vazio para nome de utilizador (padrão). De outro modo, especifique um atributo LDAP/AD.");
    translations.insert("Internal Username", "Nome de utilizador interno");
    translations.insert("Internal Username Attribute:", "Atributo do nome de utilizador interno");
    translations.insert("Override UUID detection", "Passar a detecção do UUID");
    translations.insert("Username-LDAP User Mapping", "Mapeamento do utilizador LDAP");
    translations.insert("Clear Username-LDAP User Mapping", "Limpar mapeamento do utilizador-LDAP");
    translations.insert("Clear Groupname-LDAP Group Mapping", "Limpar o mapeamento do nome de grupo LDAP");
    translations
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

// Function to get translation
pub fn get_translation(key: &str) -> Option<&'static str> {
    PT_PT.get(key).copied()
}

// Function to translate a string
pub fn translate(key: &str) -> String {
    get_translation(key).unwrap_or(key).to_string()
}