use std::collections::HashMap;
use rust_i18n::i18n;

pub fn get_translations() -> HashMap<&'static str, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to clear the mappings.", "Falha ao limpar os mapeamentos.".to_string());
    translations.insert("Failed to delete the server configuration", "Falha ao deletar a configuração do servidor".to_string());
    translations.insert("The configuration is valid and the connection could be established!", "A configuração é válida e a conexão foi estabelecida!".to_string());
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "A configuração é válida, mas o Bind falhou. Confira as configurações do servidor e as credenciais.".to_string());
    translations.insert("The configuration is invalid. Please have a look at the logs for further details.", "Configuração inválida. Por favor, dê uma olhada nos logs para mais detalhes.".to_string());
    translations.insert("No action specified", "Nenhuma ação especificada".to_string());
    translations.insert("No configuration specified", "Nenhuma configuração especificada".to_string());
    translations.insert("No data specified", "Não há dados especificados".to_string());
    translations.insert(" Could not set configuration %s", "Não foi possível definir a configuração %s".to_string());
    translations.insert("Deletion failed", "Remoção falhou".to_string());
    translations.insert("Take over settings from recent server configuration?", "Tomar parámetros de recente configuração de servidor?".to_string());
    translations.insert("Keep settings?", "Manter ajustes?".to_string());
    translations.insert("Cannot add server configuration", "Impossível adicionar a configuração do servidor".to_string());
    translations.insert("mappings cleared", "mapeamentos limpos".to_string());
    translations.insert("Success", "Sucesso".to_string());
    translations.insert("Error", "Erro".to_string());
    translations.insert("Select groups", "Selecionar grupos".to_string());
    translations.insert("Select object classes", "Selecione classes de objetos".to_string());
    translations.insert("Select attributes", "Selecione os atributos".to_string());
    translations.insert("Connection test succeeded", "Teste de conexão bem sucedida".to_string());
    translations.insert("Connection test failed", "Teste de conexão falhou".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?", "Você quer realmente deletar as atuais Configurações de Servidor?".to_string());
    translations.insert("Confirm Deletion", "Confirmar Exclusão".to_string());
    translations.insert("_%s group found_::_%s groups found_", "grupo% s encontrado|grupos% s encontrado".to_string());
    translations.insert("_%s user found_::_%s users found_", "usuário %s encontrado|usuários %s encontrados".to_string());
    translations.insert("Invalid Host", "Host inválido".to_string());
    translations.insert("Could not find the desired feature", "Não foi possível encontrar a função desejada".to_string());
    translations.insert("Save", "Guardar".to_string());
    translations.insert("Test Configuration", "Teste de Configuração".to_string());
    translations.insert("Help", "Ajuda".to_string());
    translations.insert("Limit the access to %s to groups meeting this criteria:", "Limitar o acesso a %s para grupos que coincidam com estes critérios:".to_string());
    translations.insert("only those object classes:", "apenas essas classes de objetos:".to_string());
    translations.insert("only from those groups:", "apenas a partir dos grupos:".to_string());
    translations.insert("Edit raw filter instead", "Editar filtro raw ao invéz".to_string());
    translations.insert("Raw LDAP filter", "Filtro LDAP Raw".to_string());
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.", "O filtro especifica quais grupos LDAP devem ter acesso à instância do %s.".to_string());
    translations.insert("groups found", "grupos encontrados".to_string());
    translations.insert("What attribute shall be used as login name:", "O atributo deve ser usado como nome de login:".to_string());
    translations.insert("LDAP Username:", "Usuário LDAP:".to_string());
    translations.insert("LDAP Email Address:", "LDAP Endereço de E-mail:".to_string());
    translations.insert("Other Attributes:", "Outros atributos:".to_string());
    translations.insert("Add Server Configuration", "Adicionar Configuração de Servidor".to_string());
    translations.insert("Host", "Servidor".to_string());
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Você pode omitir o protocolo, exceto quando requerer SSL. Então inicie com ldaps://".to_string());
    translations.insert("Port", "Porta".to_string());
    translations.insert("User DN", "DN Usuário".to_string());
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "O DN do cliente usuário com qual a ligação deverá ser feita, ex. uid=agent,dc=example,dc=com. Para acesso anônimo, deixe DN e Senha vazios.".to_string());
    translations.insert("Password", "Senha".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.", "Para acesso anônimo, deixe DN e Senha vazios.".to_string());
    translations.insert("One Base DN per line", "Uma base DN por linha".to_string());
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", "Você pode especificar DN Base para usuários e grupos na guia Avançada".to_string());
    translations.insert("Limit the access to %s to users meeting this criteria:", "Limitar o acesso a %s para usuários que coincidam com estes critérios:".to_string());
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.", "O filtro especifica quais usuários LDAP devem ter acesso à instância do %s.".to_string());
    translations.insert("users found", "usuários encontrados".to_string());
    translations.insert("Back", "Voltar".to_string());
    translations.insert("Continue", "Continuar".to_string());
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Aviso:</b> Os aplicativos user_ldap e user_webdavauth são incompatíveis. Você pode experimentar comportamento inesperado. Por favor, peça ao seu administrador do sistema para desabilitar um deles.".to_string());
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Aviso:</b> O módulo PHP LDAP não está instalado, o backend não funcionará. Por favor, peça ao seu administrador do sistema para instalá-lo.".to_string());
    translations.insert("Connection Settings", "Configurações de Conexão".to_string());
    translations.insert("Configuration Active", "Configuração ativa".to_string());
    translations.insert("When unchecked, this configuration will be skipped.", "Quando não marcada, esta configuração será ignorada.".to_string());
    translations.insert("User Login Filter", "Filtro de Login de Usuário".to_string());
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Define o filtro a ser aplicado, o login é feito. %%uid substitui o nome do usuário na ação de login. Exemplo: \"uid=%%uid\"".to_string());
    translations.insert("Backup (Replica) Host", "Servidor de Backup (Réplica)".to_string());
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Defina um servidor de backup opcional. Ele deverá ser uma réplica do servidor LDAP/AD principal.".to_string());
    translations.insert("Backup (Replica) Port", "Porta do Backup (Réplica)".to_string());
    translations.insert("Disable Main Server", "Desativar Servidor Principal".to_string());
    translations.insert("Only connect to the replica server.", "Conectar-se somente ao servidor de réplica.".to_string());
    translations.insert("Case insensitve LDAP server (Windows)", "Servidor LDAP sensível à caixa alta (Windows)".to_string());
    translations.insert("Turn off SSL certificate validation.", "Desligar validação de certificado SSL.".to_string());
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Não recomendado, use-o somente para teste! Se a conexão só funciona com esta opção, importar o certificado SSL do servidor LDAP em seu servidor %s.".to_string());
    translations.insert("Cache Time-To-Live", "Cache Time-To-Live".to_string());
    translations.insert("in seconds. A change empties the cache.", "em segundos. Uma mudança esvaziará o cache.".to_string());
    translations.insert("Directory Settings", "Configurações de Diretório".to_string());
    translations.insert("User Display Name Field", "Campo Nome de Exibição de Usuário".to_string());
    translations.insert("The LDAP attribute to use to generate the user's display name.", "O atributo LDAP para usar para gerar o nome de exibição do usuário.".to_string());
    translations.insert("Base User Tree", "Árvore de Usuário Base".to_string());
    translations.insert("One User Base DN per line", "Um usuário-base DN por linha".to_string());
    translations.insert("User Search Attributes", "Atributos de Busca de Usuário".to_string());
    translations.insert("Optional; one attribute per line", "Opcional; um atributo por linha".to_string());
    translations.insert("Group Display Name Field", "Campo Nome de Exibição de Grupo".to_string());
    translations.insert("The LDAP attribute to use to generate the groups's display name.", "O atributo LDAP para usar para gerar o nome de apresentação do grupo.".to_string());
    translations.insert("Base Group Tree", "Árvore de Grupo Base".to_string());
    translations.insert("One Group Base DN per line", "Um grupo-base DN por linha".to_string());
    translations.insert("Group Search Attributes", "Atributos de Busca de Grupo".to_string());
    translations.insert("Group-Member association", "Associação Grupo-Membro".to_string());
    translations.insert("Special Attributes", "Atributos Especiais".to_string());
    translations.insert("Quota Field", "Campo de Cota".to_string());
    translations.insert("Quota Default", "Cota Padrão".to_string());
    translations.insert("in bytes", "em bytes".to_string());
    translations.insert("Email Field", "Campo de Email".to_string());
    translations.insert("User Home Folder Naming Rule", "Regra para Nome da Pasta Pessoal do Usuário".to_string());
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Deixe vazio para nome de usuário (padrão). Caso contrário, especifique um atributo LDAP/AD.".to_string());
    translations.insert("Internal Username", "Nome de usuário interno".to_string());
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Por padrão, o nome de usuário interno será criado a partir do atributo UUID. Ele garante que o nome de usuário é único e que caracteres não precisam ser convertidos. O nome de usuário interno tem a restrição de que apenas estes caracteres são permitidos: [a-zA-Z0-9_.@- ]. Outros caracteres são substituídos por seus correspondentes em ASCII ou simplesmente serão omitidos. Em caso de colisão um número será adicionado/aumentado. O nome de usuário interno é usado para identificar um usuário internamente. É também o nome padrão da pasta \"home\" do usuário. É também parte de URLs remotas, por exemplo, para todos as instâncias *DAV. Com esta definição, o comportamento padrão pode ser sobrescrito. Para alcançar um comportamento semelhante ao de antes do ownCloud 5, forneça o atributo do nome de exibição do usuário no campo seguinte. Deixe-o vazio para o comportamento padrão. As alterações terão efeito apenas para usuários LDAP recém mapeados (adicionados).".to_string());
    translations.insert("Internal Username Attribute:", "Atributo Interno de Nome de Usuário:".to_string());
    translations.insert("Override UUID detection", "Substituir detecção UUID".to_string());
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Por padrão, o atributo UUID é detectado automaticamente. O atributo UUID é usado para identificar, sem dúvidas, os usuários e grupos LDAP. Além disso, o nome de usuário interno será criado com base no UUID, se não especificado acima. Você pode substituir a configuração e passar um atributo de sua escolha. Você deve certificar-se de que o atributo de sua escolha pode ser lido tanto para usuários como para grupos, e que seja único. Deixe-o vazio para o comportamento padrão. As alterações terão efeito apenas para usuários e grupos LDAP recém mapeados (adicionados).".to_string());
    translations.insert("UUID Attribute for Users:", "UUID Atributos para Usuários:".to_string());
    translations.insert("UUID Attribute for Groups:", "UUID Atributos para Grupos:".to_string());
    translations.insert("Username-LDAP User Mapping", "Usuário-LDAP Mapeamento de Usuário".to_string());
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "Nomes de usuários sãi usados para armazenar e atribuir (meta) dados. A fim de identificar com precisão e reconhecer usuários, cada usuário LDAP terá um nome de usuário interno. Isso requer um mapeamento nome de usuário para usuário LDAP. O nome de usuário criado é mapeado para o UUID do usuário LDAP. Adicionalmente, o DN fica em cache, assim como para reduzir a interação LDAP, mas não é utilizado para a identificação. Se o DN muda, as mudanças serão encontradas. O nome de usuário interno é utilizado em todo lugar. Limpar os mapeamentos não influencia a configuração. Limpar os mapeamentos deixará rastros em todo lugar. Limpar os mapeamentos não influencia a configuração, mas afeta as configurações LDAP! Somente limpe os mapeamentos em embiente de testes ou em estágio experimental.".to_string());
    translations.insert("Clear Username-LDAP User Mapping", "Limpar Mapeamento de Usuário Nome de Usuário-LDAP".to_string());
    translations.insert("Clear Groupname-LDAP Group Mapping", "Limpar NomedoGrupo-LDAP Mapeamento do Grupo".to_string());
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}

i18n!("pt_BR", fallback = "en");