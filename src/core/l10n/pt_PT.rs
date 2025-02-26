use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%s shared »%s« with you", "%s partilhado »%s« contigo");
        m.insert("Turned on maintenance mode", "Activado o modo de manutenção");
        m.insert("Turned off maintenance mode", "Desactivado o modo de manutenção");
        m.insert("Updated database", "Base de dados actualizada");
        m.insert("Updating filecache, this may take really long...", "A actualizar o cache dos ficheiros, poderá demorar algum tempo...");
        m.insert("Updated filecache", "Actualizado o cache dos ficheiros");
        m.insert("... %d%% done ...", "... %d%% feito ...");
        m.insert("No image or file provided", "Não foi selecionado nenhum ficheiro para importar");
        m.insert("Unknown filetype", "Ficheiro desconhecido");
        m.insert("Invalid image", "Imagem inválida");
        m.insert("No temporary profile picture available, try again", "Foto temporária de perfil indisponível, tente novamente");
        m.insert("Sunday", "Domingo");
        m.insert("Monday", "Segunda");
        m.insert("Tuesday", "Terça");
        m.insert("Wednesday", "Quarta");
        m.insert("Thursday", "Quinta");
        m.insert("Friday", "Sexta");
        m.insert("Saturday", "Sábado");
        m.insert("January", "Janeiro");
        m.insert("February", "Fevereiro");
        m.insert("March", "Março");
        m.insert("April", "Abril");
        m.insert("May", "Maio");
        m.insert("June", "Junho");
        m.insert("July", "Julho");
        m.insert("August", "Agosto");
        m.insert("September", "Setembro");
        m.insert("October", "Outubro");
        m.insert("November", "Novembro");
        m.insert("December", "Dezembro");
        m.insert("Settings", "Configurações");
        m.insert("seconds ago", "Minutos atrás");
        m.insert("today", "hoje");
        m.insert("yesterday", "ontem");
        m.insert("last month", "ultímo mês");
        m.insert("months ago", "meses atrás");
        m.insert("last year", "ano passado");
        m.insert("years ago", "anos atrás");
        m.insert("Choose", "Escolha");
        m.insert("Yes", "Sim");
        m.insert("No", "Não");
        m.insert("Ok", "Ok");
        m.insert("Error loading message template: {error}", "Erro ao carregar o template: {error}");
        m.insert("Cancel", "Cancelar");
        m.insert("Continue", "Continuar");
        m.insert("(all selected)", "(todos seleccionados)");
        m.insert("({count} selected)", "({count} seleccionados)");
        m.insert("Shared", "Partilhado");
        m.insert("Share", "Partilhar");
        m.insert("Error", "Erro");
        m.insert("Error while sharing", "Erro ao partilhar");
        m.insert("Error while unsharing", "Erro ao deixar de partilhar");
        m.insert("Error while changing permissions", "Erro ao mudar permissões");
        m.insert("Shared with you and the group {group} by {owner}", "Partilhado consigo e com o grupo {group} por {owner}");
        m.insert("Shared with you by {owner}", "Partilhado consigo por {owner}");
        m.insert("Password protect", "Proteger com palavra-passe");
        m.insert("Password", "Password");
        m.insert("Allow Public Upload", "Permitir Envios Públicos");
        m.insert("Email link to person", "Enviar o link por e-mail");
        m.insert("Send", "Enviar");
        m.insert("Set expiration date", "Especificar data de expiração");
        m.insert("Expiration date", "Data de expiração");
        m.insert("Share via email:", "Partilhar via email:");
        m.insert("No people found", "Não foi encontrado ninguém");
        m.insert("group", "grupo");
        m.insert("Resharing is not allowed", "Não é permitido partilhar de novo");
        m.insert("Shared in {item} with {user}", "Partilhado em {item} com {user}");
        m.insert("Unshare", "Deixar de partilhar");
        m.insert("can edit", "pode editar");
        m.insert("access control", "Controlo de acesso");
        m.insert("create", "criar");
        m.insert("update", "actualizar");
        m.insert("delete", "apagar");
        m.insert("share", "partilhar");
        m.insert("Password protected", "Protegido com palavra-passe");
        m.insert("Error unsetting expiration date", "Erro ao retirar a data de expiração");
        m.insert("Error setting expiration date", "Erro ao aplicar a data de expiração");
        m.insert("Sending ...", "A Enviar...");
        m.insert("Email sent", "E-mail enviado");
        m.insert("Warning", "Aviso");
        m.insert("The object type is not specified.", "O tipo de objecto não foi especificado");
        m.insert("Delete", "Eliminar");
        m.insert("Add", "Adicionar");
        m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "A actualização falhou. Por favor reporte este incidente seguindo este link <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "A actualização foi concluída com sucesso. Vai ser redireccionado para o ownCloud agora.");
        m.insert("%s password reset", "%s reposição da password");
        m.insert("Use the following link to reset your password: {link}", "Use o seguinte endereço para repor a sua password: {link}");
        m.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "O link para fazer reset à sua password foi enviado para o seu e-mail. <br> Se não o recebeu dentro um espaço de tempo aceitável, por favor verifique a sua pasta de SPAM.<br> Se não o encontrar, por favor contacte o seu administrador.");
        m.insert("Request failed!<br>Did you make sure your email/username was right?", "O pedido falhou! <br> Tem a certeza que introduziu o seu email/username correcto?");
        m.insert("You will receive a link to reset your password via Email.", "Vai receber um endereço para repor a sua password");
        m.insert("Username", "Nome de utilizador");
        m.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Os seus ficheiros estão encriptados. Se não activou a chave de recuperação, não vai ser possível recuperar os seus dados no caso da sua password ser reinicializada. Se não tem a certeza do que precisa de fazer, por favor contacte o seu administrador antes de continuar. Tem a certeza que quer continuar?");
        m.insert("Yes, I really want to reset my password now", "Sim, tenho a certeza que pretendo redefinir a minha palavra-passe agora.");
        m.insert("Your password was reset", "A sua password foi reposta");
        m.insert("To login page", "Para a página de entrada");
        m.insert("New password", "Nova palavra-chave");
        m.insert("Reset password", "Repor password");
        m.insert("Personal", "Pessoal");
        m.insert("Users", "Utilizadores");
        m.insert("Apps", "Aplicações");
        m.insert("Admin", "Admin");
        m.insert("Help", "Ajuda");
        m.insert("Access forbidden", "Acesso interdito");
        m.insert("Cloud not found", "Cloud nao encontrada");
        m.insert("Security Warning", "Aviso de Segurança");
        m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "A sua versão do PHP é vulnerável ao ataque Byte Null (CVE-2006-7243)");
        m.insert("Please update your PHP installation to use %s securely.", "Por favor atualize a sua versão PHP instalada para usar o %s com segurança.");
        m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Não existe nenhum gerador seguro de números aleatórios, por favor, active a extensão OpenSSL no PHP.");
        m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Sem nenhum gerador seguro de números aleatórios, uma pessoa mal intencionada pode prever a sua password, reiniciar as seguranças adicionais e tomar conta da sua conta. ");
        m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "A pasta de dados do ownCloud e os respectivos ficheiros, estarão provavelmente acessíveis a partir da internet, pois o ficheiros .htaccess não funciona.");
        m.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Para obter informações de como configurar correctamente o servidor, veja em: <a href=\"%s\" target=\"_blank\">documentação</a>.");
        m.insert("Create an <strong>admin account</strong>", "Criar uma <strong>conta administrativa</strong>");
        m.insert("Advanced", "Avançado");
        m.insert("Data folder", "Pasta de dados");
        m.insert("Configure the database", "Configure a base de dados");
        m.insert("will be used", "vai ser usada");
        m.insert("Database user", "Utilizador da base de dados");
        m.insert("Database password", "Password da base de dados");
        m.insert("Database name", "Nome da base de dados");
        m.insert("Database tablespace", "Tablespace da base de dados");
        m.insert("Database host", "Anfitrião da base de dados");
        m.insert("Finish setup", "Acabar instalação");
        m.insert("Finishing …", "A terminar...");
        m.insert("%s is available. Get more information on how to update.", "%s está disponível. Tenha mais informações como actualizar.");
        m.insert("Log out", "Sair");
        m.insert("Automatic logon rejected!", "Login automático rejeitado!");
        m.insert("If you did not change your password recently, your account may be compromised!", "Se não mudou a sua palavra-passe recentemente, a sua conta pode ter sido comprometida!");
        m.insert("Please change your password to secure your account again.", "Por favor mude a sua palavra-passe para assegurar a sua conta de novo.");
        m.insert("Lost your password?", "Esqueceu-se da sua password?");
        m.insert("remember", "lembrar");
        m.insert("Log in", "Entrar");
        m.insert("Alternative Logins", "Contas de acesso alternativas");
        m.insert("Updating ownCloud to version %s, this may take a while.", "A actualizar o ownCloud para a versão %s, esta operação pode demorar.");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["%n minuto atrás", "%n minutos atrás"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["%n hora atrás", "%n horas atrás"]);
        m.insert("_%n day ago_::_%n days ago_", vec!["%n dia atrás", "%n dias atrás"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["%n mês atrás", "%n meses atrás"]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
        m
    };

    pub static ref PLURAL_RULE: &'static str = "nplurals=2; plural=(n != 1);";
}

// Función para obtener una traducción
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

// Función para obtener una traducción plural
pub fn get_plural_translation(key: &str, n: usize) -> Option<&'static str> {
    PLURAL_FORMS.get(key).and_then(|forms| {
        let index = if n != 1 { 1 } else { 0 };
        forms.get(index).copied()
    })
}