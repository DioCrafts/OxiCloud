use std::collections::HashMap;
use rust_gettext::Catalog;

// Catálogo de traducciones en portugués brasileño (pt_BR)
pub fn get_translations() -> Catalog {
    let mut translations = HashMap::new();
    
    translations.insert("%s shared »%s« with you".to_string(), "%s compartilhou »%s« com você".to_string());
    translations.insert("Couldn't send mail to following users: %s ".to_string(), "Não foi possível enviar e-mail para os seguintes usuários: %s".to_string());
    translations.insert("Turned on maintenance mode".to_string(), "Ativar modo de manutenção".to_string());
    translations.insert("Turned off maintenance mode".to_string(), "Desligar o modo de manutenção".to_string());
    translations.insert("Updated database".to_string(), "Atualizar o banco de dados".to_string());
    translations.insert("Updating filecache, this may take really long...".to_string(), "Atualizar cahe de arquivos, isto pode levar algum tempo...".to_string());
    translations.insert("Updated filecache".to_string(), "Atualizar cache de arquivo".to_string());
    translations.insert("... %d%% done ...".to_string(), "... %d%% concluído ...".to_string());
    translations.insert("No image or file provided".to_string(), "Nenhuma imagem ou arquivo fornecido".to_string());
    translations.insert("Unknown filetype".to_string(), "Tipo de arquivo desconhecido".to_string());
    translations.insert("Invalid image".to_string(), "Imagem inválida".to_string());
    translations.insert("No temporary profile picture available, try again".to_string(), "Sem imagem no perfil temporário disponível, tente novamente".to_string());
    translations.insert("No crop data provided".to_string(), "Nenhum dado para coleta foi fornecido".to_string());
    translations.insert("Sunday".to_string(), "Domingo".to_string());
    translations.insert("Monday".to_string(), "Segunda-feira".to_string());
    translations.insert("Tuesday".to_string(), "Terça-feira".to_string());
    translations.insert("Wednesday".to_string(), "Quarta-feira".to_string());
    translations.insert("Thursday".to_string(), "Quinta-feira".to_string());
    translations.insert("Friday".to_string(), "Sexta-feira".to_string());
    translations.insert("Saturday".to_string(), "Sábado".to_string());
    translations.insert("January".to_string(), "janeiro".to_string());
    translations.insert("February".to_string(), "fevereiro".to_string());
    translations.insert("March".to_string(), "março".to_string());
    translations.insert("April".to_string(), "abril".to_string());
    translations.insert("May".to_string(), "maio".to_string());
    translations.insert("June".to_string(), "junho".to_string());
    translations.insert("July".to_string(), "julho".to_string());
    translations.insert("August".to_string(), "agosto".to_string());
    translations.insert("September".to_string(), "setembro".to_string());
    translations.insert("October".to_string(), "outubro".to_string());
    translations.insert("November".to_string(), "novembro".to_string());
    translations.insert("December".to_string(), "dezembro".to_string());
    translations.insert("Settings".to_string(), "Ajustes".to_string());
    translations.insert("seconds ago".to_string(), "segundos atrás".to_string());
    translations.insert("today".to_string(), "hoje".to_string());
    translations.insert("yesterday".to_string(), "ontem".to_string());
    translations.insert("last month".to_string(), "último mês".to_string());
    translations.insert("months ago".to_string(), "meses atrás".to_string());
    translations.insert("last year".to_string(), "último ano".to_string());
    translations.insert("years ago".to_string(), "anos atrás".to_string());
    translations.insert("Choose".to_string(), "Escolha".to_string());
    translations.insert("Error loading file picker template: {error}".to_string(), "Erro no seletor de carregamento modelo de arquivos: {error}".to_string());
    translations.insert("Yes".to_string(), "Sim".to_string());
    translations.insert("No".to_string(), "Não".to_string());
    translations.insert("Ok".to_string(), "Ok".to_string());
    translations.insert("Error loading message template: {error}".to_string(), "Erro no carregamento de modelo de mensagem: {error}".to_string());
    translations.insert("One file conflict".to_string(), "Conflito em um arquivo".to_string());
    translations.insert("Which files do you want to keep?".to_string(), "Qual arquivo você quer manter?".to_string());
    translations.insert("If you select both versions, the copied file will have a number added to its name.".to_string(), "Se você selecionar ambas as versões, o arquivo copiado terá um número adicionado ao seu nome.".to_string());
    translations.insert("Cancel".to_string(), "Cancelar".to_string());
    translations.insert("Continue".to_string(), "Continuar".to_string());
    translations.insert("(all selected)".to_string(), "(todos os selecionados)".to_string());
    translations.insert("({count} selected)".to_string(), "({count} selecionados)".to_string());
    translations.insert("Error loading file exists template".to_string(), "Erro ao carregar arquivo existe modelo".to_string());
    translations.insert("Shared".to_string(), "Compartilhados".to_string());
    translations.insert("Share".to_string(), "Compartilhar".to_string());
    translations.insert("Error".to_string(), "Erro".to_string());
    translations.insert("Error while sharing".to_string(), "Erro ao compartilhar".to_string());
    translations.insert("Error while unsharing".to_string(), "Erro ao descompartilhar".to_string());
    translations.insert("Error while changing permissions".to_string(), "Erro ao mudar permissões".to_string());
    translations.insert("Shared with you and the group {group} by {owner}".to_string(), "Compartilhado com você e com o grupo {group} por {owner}".to_string());
    translations.insert("Shared with you by {owner}".to_string(), "Compartilhado com você por {owner}".to_string());
    translations.insert("Share with user or group …".to_string(), "Compartilhar com usuário ou grupo ...".to_string());
    translations.insert("Share link".to_string(), "Compartilher link".to_string());
    translations.insert("Password protect".to_string(), "Proteger com senha".to_string());
    translations.insert("Password".to_string(), "Senha".to_string());
    translations.insert("Allow Public Upload".to_string(), "Permitir upload público".to_string());
    translations.insert("Email link to person".to_string(), "Enviar link por e-mail".to_string());
    translations.insert("Send".to_string(), "Enviar".to_string());
    translations.insert("Set expiration date".to_string(), "Definir data de expiração".to_string());
    translations.insert("Expiration date".to_string(), "Data de expiração".to_string());
    translations.insert("Share via email:".to_string(), "Compartilhar via e-mail:".to_string());
    translations.insert("No people found".to_string(), "Nenhuma pessoa encontrada".to_string());
    translations.insert("group".to_string(), "grupo".to_string());
    translations.insert("Resharing is not allowed".to_string(), "Não é permitido re-compartilhar".to_string());
    translations.insert("Shared in {item} with {user}".to_string(), "Compartilhado em {item} com {user}".to_string());
    translations.insert("Unshare".to_string(), "Descompartilhar".to_string());
    translations.insert("notify by email".to_string(), "notificar por e-mail".to_string());
    translations.insert("can edit".to_string(), "pode editar".to_string());
    translations.insert("access control".to_string(), "controle de acesso".to_string());
    translations.insert("create".to_string(), "criar".to_string());
    translations.insert("update".to_string(), "atualizar".to_string());
    translations.insert("delete".to_string(), "remover".to_string());
    translations.insert("share".to_string(), "compartilhar".to_string());
    translations.insert("Password protected".to_string(), "Protegido com senha".to_string());
    translations.insert("Error unsetting expiration date".to_string(), "Erro ao remover data de expiração".to_string());
    translations.insert("Error setting expiration date".to_string(), "Erro ao definir data de expiração".to_string());
    translations.insert("Sending ...".to_string(), "Enviando ...".to_string());
    translations.insert("Email sent".to_string(), "E-mail enviado".to_string());
    translations.insert("Warning".to_string(), "Aviso".to_string());
    translations.insert("The object type is not specified.".to_string(), "O tipo de objeto não foi especificado.".to_string());
    translations.insert("Enter new".to_string(), "Entrar uma nova".to_string());
    translations.insert("Delete".to_string(), "Eliminar".to_string());
    translations.insert("Add".to_string(), "Adicionar".to_string());
    translations.insert("Edit tags".to_string(), "Editar etiqueta".to_string());
    translations.insert("Error loading dialog template: {error}".to_string(), "Erro carregando diálogo de formatação:{error}".to_string());
    translations.insert("No tags selected for deletion.".to_string(), "Nenhuma etiqueta selecionada para deleção.".to_string());
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.".to_string(), "A atualização falhou. Por favor, relate este problema para a <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">comunidade ownCloud</a>.".to_string());
    translations.insert("The update was successful. Redirecting you to ownCloud now.".to_string(), "A atualização teve êxito. Você será redirecionado ao ownCloud agora.".to_string());
    translations.insert("%s password reset".to_string(), "%s redefinir senha".to_string());
    translations.insert("Use the following link to reset your password: {link}".to_string(), "Use o seguinte link para redefinir sua senha: {link}".to_string());
    translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .".to_string(), "O link para redefinir sua senha foi enviada para o seu e-mail. <br> Se você não recebê-lo dentro de um período razoável de tempo, verifique o spam/lixo. <br> Se ele não estiver lá perguntar ao seu administrador local.".to_string());
    translations.insert("Request failed!<br>Did you make sure your email/username was right?".to_string(), "O pedido falhou! <br>Certifique-se que seu e-mail/username estavam corretos?".to_string());
    translations.insert("You will receive a link to reset your password via Email.".to_string(), "Você receberá um link para redefinir sua senha por e-mail.".to_string());
    translations.insert("Username".to_string(), "Nome de usuário".to_string());
    translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?".to_string(), "Seus arquivos estão encriptados. Se você não habilitou a chave de recuperação, não haverá maneira de recuperar seus dados após criar uma nova senha. Se você não tem certeza do que fazer,  por favor entre em contato com o administrador antes de continuar. Tem certeza que realmente quer continuar?".to_string());
    translations.insert("Yes, I really want to reset my password now".to_string(), "Sim, realmente quero criar uma nova senha.".to_string());
    translations.insert("Reset".to_string(), "Resetar".to_string());
    translations.insert("Your password was reset".to_string(), "Sua senha foi redefinida".to_string());
    translations.insert("To login page".to_string(), "Para a página de login".to_string());
    translations.insert("New password".to_string(), "Nova senha".to_string());
    translations.insert("Reset password".to_string(), "Redefinir senha".to_string());
    translations.insert("Personal".to_string(), "Pessoal".to_string());
    translations.insert("Users".to_string(), "Usuários".to_string());
    translations.insert("Apps".to_string(), "Aplicações".to_string());
    translations.insert("Admin".to_string(), "Admin".to_string());
    translations.insert("Help".to_string(), "Ajuda".to_string());
    translations.insert("Error loading tags".to_string(), " Erro carregando etiqueta".to_string());
    translations.insert("Tag already exists".to_string(), "tiqueta já existe".to_string());
    translations.insert("Error deleting tag(s)".to_string(), "Erro deletando etiqueta(s)".to_string());
    translations.insert("Error tagging".to_string(), "Erro etiquetando".to_string());
    translations.insert("Error untagging".to_string(), "Erro retirando etiquetando".to_string());
    translations.insert("Error favoriting".to_string(), "Erro colocando no favoritos".to_string());
    translations.insert("Error unfavoriting".to_string(), "Erro retirando do favoritos".to_string());
    translations.insert("Access forbidden".to_string(), "Acesso proibido".to_string());
    translations.insert("Cloud not found".to_string(), "Cloud não encontrado".to_string());
    translations.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n".to_string(), "Olá,\n\ngostaria que você soubesse que %s compartilhou %s com vecê.\nVeja isto: %s\n\n".to_string());
    translations.insert("The share will expire on %s.\n\n".to_string(), "O compartilhamento irá expirer em %s.\n\n".to_string());
    translations.insert("Cheers!".to_string(), "Saúde!".to_string());
    translations.insert("Security Warning".to_string(), "Aviso de Segurança".to_string());
    translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)".to_string(), "Sua versão do PHP está vulnerável ao ataque NULL Byte (CVE-2006-7243)".to_string());
    translations.insert("Please update your PHP installation to use %s securely.".to_string(), "Por favor, atualize sua instalação PHP para usar %s segurança.".to_string());
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.".to_string(), "Nenhum gerador de número aleatório de segurança disponível. Habilite a extensão OpenSSL do PHP.".to_string());
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.".to_string(), "Sem um gerador de número aleatório de segurança, um invasor pode ser capaz de prever os símbolos de redefinição de senhas e assumir sua conta.".to_string());
    translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.".to_string(), "Seu diretório de dados e arquivos são provavelmente acessíveis pela internet, porque o .htaccess não funciona.".to_string());
    translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.".to_string(), "Para obter informações sobre como configurar corretamente o seu servidor, consulte a <a href=\"%s\" target=\"_blank\">documentação</a>.".to_string());
    translations.insert("Create an <strong>admin account</strong>".to_string(), "Criar uma <strong>conta de administrador</strong>".to_string());
    translations.insert("Advanced".to_string(), "Avançado".to_string());
    translations.insert("Data folder".to_string(), "Pasta de dados".to_string());
    translations.insert("Configure the database".to_string(), "Configurar o banco de dados".to_string());
    translations.insert("will be used".to_string(), "será usado".to_string());
    translations.insert("Database user".to_string(), "Usuário do banco de dados".to_string());
    translations.insert("Database password".to_string(), "Senha do banco de dados".to_string());
    translations.insert("Database name".to_string(), "Nome do banco de dados".to_string());
    translations.insert("Database tablespace".to_string(), "Espaço de tabela do banco de dados".to_string());
    translations.insert("Database host".to_string(), "Host do banco de dados".to_string());
    translations.insert("Finish setup".to_string(), "Concluir configuração".to_string());
    translations.insert("Finishing …".to_string(), "Finalizando ...".to_string());
    translations.insert("%s is available. Get more information on how to update.".to_string(), "%s está disponível. Obtenha mais informações sobre como atualizar.".to_string());
    translations.insert("Log out".to_string(), "Sair".to_string());
    translations.insert("Automatic logon rejected!".to_string(), "Entrada Automática no Sistema Rejeitada!".to_string());
    translations.insert("If you did not change your password recently, your account may be compromised!".to_string(), "Se você não mudou a sua senha recentemente, a sua conta pode estar comprometida!".to_string());
    translations.insert("Please change your password to secure your account again.".to_string(), "Por favor troque sua senha para tornar sua conta segura novamente.".to_string());
    translations.insert("Server side authentication failed!".to_string(), "Autenticação do servidor falhou!".to_string());
    translations.insert("Please contact your administrator.".to_string(), "Por favor, contate o administrador.".to_string());
    translations.insert("Lost your password?".to_string(), "Esqueceu sua senha?".to_string());
    translations.insert("remember".to_string(), "lembrar".to_string());
    translations.insert("Log in".to_string(), "Fazer login".to_string());
    translations.insert("Alternative Logins".to_string(), "Logins alternativos".to_string());
    translations.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>".to_string(), "Olá,<br><br>só gostaria que você soubesse que %s compartilhou »%s« com você.<br><a href=\"%s\">Veja isto!</a><br><br".to_string());
    translations.insert("The share will expire on %s.<br><br>".to_string(), "O compartilhamento irá expirar em %s.<br><br>".to_string());
    translations.insert("Updating ownCloud to version %s, this may take a while.".to_string(), "Atualizando ownCloud para a versão %s, isto pode levar algum tempo.".to_string());
    translations.insert("This ownCloud instance is currently being updated, which may take a while.".to_string(), "Esta instância do ownCloud está sendo atualizada, o que pode demorar um pouco.".to_string());
    translations.insert("Please reload this page after a short time to continue using ownCloud.".to_string(), "Por favor, atualize esta página depois de um curto período de tempo para continuar usando ownCloud.".to_string());
    translations.insert("Contact your system administrator if this message persists or appeared unexpectedly.".to_string(), "Contacte o seu administrador do sistema se esta mensagem persistir ou aparecer inesperadamente.".to_string());
    translations.insert("Thank you for your patience.".to_string(), "Obrigado pela sua paciência.".to_string());
    
    // Plurales
    let mut plural_translations = HashMap::new();
    
    // _%n minute ago_::_%n minutes ago_
    let mut minutes_ago = HashMap::new();
    minutes_ago.insert(0, " ha %n minuto".to_string());
    minutes_ago.insert(1, "ha %n minutos".to_string());
    plural_translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), minutes_ago);
    
    // _%n hour ago_::_%n hours ago_
    let mut hours_ago = HashMap::new();
    hours_ago.insert(0, "ha %n hora".to_string());
    hours_ago.insert(1, "ha %n horas".to_string());
    plural_translations.insert("_%n hour ago_::_%n hours ago_".to_string(), hours_ago);
    
    // _%n day ago_::_%n days ago_
    let mut days_ago = HashMap::new();
    days_ago.insert(0, "ha %n dia".to_string());
    days_ago.insert(1, "ha %n dias".to_string());
    plural_translations.insert("_%n day ago_::_%n days ago_".to_string(), days_ago);
    
    // _%n month ago_::_%n months ago_
    let mut months_ago = HashMap::new();
    months_ago.insert(0, "ha %n mês".to_string());
    months_ago.insert(1, "ha %n meses".to_string());
    plural_translations.insert("_%n month ago_::_%n months ago_".to_string(), months_ago);
    
    // _{count} file conflict_::_{count} file conflicts_
    let mut file_conflicts = HashMap::new();
    file_conflicts.insert(0, "{count} conflito de arquivo".to_string());
    file_conflicts.insert(1, "{count} conflitos de arquivos".to_string());
    plural_translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), file_conflicts);
    
    Catalog {
        translations,
        plural_translations,
        plural_forms: "nplurals=2; plural=(n > 1);".to_string(),
        language: "pt_BR".to_string(),
    }
}