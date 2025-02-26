use std::collections::HashMap;
use rust_i18n::t;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Could not move %s - File with this name already exists", "Impossível mover %s - Um arquivo com este nome já existe");
    translations.insert("Could not move %s", "Impossível mover %s");
    translations.insert("File name cannot be empty.", "O nome do arquivo não pode estar vazio.");
    translations.insert("File name must not contain \"/\". Please choose a different name.", "O nome do arquivo não deve conter \"/\". Por favor, escolha um nome diferente.");
    translations.insert("The name %s is already used in the folder %s. Please choose a different name.", "O nome %s já é usado na pasta %s. Por favor, escolha um nome diferente.");
    translations.insert("Not a valid source", "Não é uma fonte válida");
    translations.insert("Error while downloading %s to %s", "Erro ao baixar %s para %s");
    translations.insert("Error when creating the file", "Erro ao criar o arquivo");
    translations.insert("Folder name cannot be empty.", "O nome da pasta não pode estar vazio.");
    translations.insert("Folder name must not contain \"/\". Please choose a different name.", "O nome da pasta não pode conter \"/\". Por favor, escolha um nome diferente.");
    translations.insert("Error when creating the folder", "Erro ao criar a pasta");
    translations.insert("Unable to set upload directory.", "Impossível configurar o diretório de upload");
    translations.insert("Invalid Token", "Token inválido");
    translations.insert("No file was uploaded. Unknown error", "Nenhum arquivo foi enviado. Erro desconhecido");
    translations.insert("There is no error, the file uploaded with success", "Sem erros, o arquivo foi enviado com sucesso");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "O arquivo enviado excede a diretiva upload_max_filesize no php.ini: ");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "O arquivo carregado excede o argumento MAX_FILE_SIZE especificado no formulário HTML");
    translations.insert("The uploaded file was only partially uploaded", "O arquivo foi parcialmente enviado");
    translations.insert("No file was uploaded", "Nenhum arquivo enviado");
    translations.insert("Missing a temporary folder", "Pasta temporária não encontrada");
    translations.insert("Failed to write to disk", "Falha ao escrever no disco");
    translations.insert("Not enough storage available", "Espaço de armazenamento insuficiente");
    translations.insert("Upload failed. Could not get file info.", "Falha no envio. Não foi possível obter informações do arquivo.");
    translations.insert("Upload failed. Could not find uploaded file", "Falha no envio. Não foi possível encontrar o arquivo enviado");
    translations.insert("Invalid directory.", "Diretório inválido.");
    translations.insert("Files", "Arquivos");
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Incapaz de fazer o envio de {filename}, pois é um diretório ou tem 0 bytes");
    translations.insert("Not enough space available", "Espaço de armazenamento insuficiente");
    translations.insert("Upload cancelled.", "Envio cancelado.");
    translations.insert("Could not get result from server.", "Não foi possível obter o resultado do servidor.");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Upload em andamento. Sair da página agora resultará no cancelamento do envio.");
    translations.insert("URL cannot be empty", "URL não pode estar vazia");
    translations.insert("In the home folder 'Shared' is a reserved filename", "Na pasta home 'Shared- Compartilhada' é um nome reservado");
    translations.insert("{new_name} already exists", "{new_name} já existe");
    translations.insert("Could not create file", "Não foi possível criar o arquivo");
    translations.insert("Could not create folder", "Não foi possível criar a pasta");
    translations.insert("Share", "Compartilhar");
    translations.insert("Delete permanently", "Excluir permanentemente");
    translations.insert("Rename", "Renomear");
    translations.insert("Pending", "Pendente");
    translations.insert("Could not rename file", "Não foi possível renomear o arquivo");
    translations.insert("replaced {new_name} with {old_name}", "Substituído {old_name} por {new_name} ");
    translations.insert("undo", "desfazer");
    translations.insert("_%n folder_::_%n folders_", "");  // Handled separately with plurals
    translations.insert("_%n file_::_%n files_", "");      // Handled separately with plurals
    translations.insert("{dirs} and {files}", "{dirs} e {files}");
    translations.insert("_Uploading %n file_::_Uploading %n files_", ""); // Handled separately with plurals
    translations.insert("'.' is an invalid file name.", "'.' é um nome de arquivo inválido.");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nome inválido, '\\', '/', '<', '>', ':', '\"', '|', '?' e '*' não são permitidos.");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "Seu armazenamento está cheio, arquivos não podem mais ser atualizados ou sincronizados!");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "Seu armazenamento está quase cheio ({usedSpacePercent}%)");
    translations.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "App de encriptação está ativado, mas as chaves não estão inicializadas, por favor log-out e faça login novamente");
    translations.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Chave do App de Encriptação é inválida. Por favor, atualize sua senha de chave privada em suas configurações pessoais para recuperar o acesso a seus arquivos criptografados.");
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Encriptação foi desabilitada mas seus arquivos continuam encriptados. Por favor vá a suas configurações pessoais para descriptar seus arquivos.");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "Seu download está sendo preparado. Isto pode levar algum tempo se os arquivos forem grandes.");
    translations.insert("Error moving file", "Erro movendo o arquivo");
    translations.insert("Error", "Erro");
    translations.insert("Name", "Nome");
    translations.insert("Size", "Tamanho");
    translations.insert("Modified", "Modificado");
    translations.insert("Invalid folder name. Usage of 'Shared' is reserved.", "Nome da pasta inválido. Uso de 'Shared' é reservado.");
    translations.insert("%s could not be renamed", "%s não pode ser renomeado");
    translations.insert("Upload", "Upload");
    translations.insert("File handling", "Tratamento de Arquivo");
    translations.insert("Maximum upload size", "Tamanho máximo para carregar");
    translations.insert("max. possible: ", "max. possível:");
    translations.insert("Needed for multi-file and folder downloads.", "Necessário para download de múltiplos arquivos e diretórios.");
    translations.insert("Enable ZIP-download", "Habilitar ZIP-download");
    translations.insert("0 is unlimited", "0 para ilimitado");
    translations.insert("Maximum input size for ZIP files", "Tamanho máximo para arquivo ZIP");
    translations.insert("Save", "Guardar");
    translations.insert("New", "Novo");
    translations.insert("Text file", "Arquivo texto");
    translations.insert("Folder", "Pasta");
    translations.insert("From link", "Do link");
    translations.insert("Deleted files", "Arquivos apagados");
    translations.insert("Cancel upload", "Cancelar upload");
    translations.insert("You don't have permission to upload or create files here", "Você não tem permissão para carregar ou criar arquivos aqui");
    translations.insert("Nothing in here. Upload something!", "Nada aqui.Carrege alguma coisa!");
    translations.insert("Download", "Baixar");
    translations.insert("Unshare", "Descompartilhar");
    translations.insert("Delete", "Excluir");
    translations.insert("Upload too large", "Upload muito grande");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Os arquivos que você está tentando carregar excedeu o tamanho máximo para arquivos no servidor.");
    translations.insert("Files are being scanned, please wait.", "Arquivos sendo escaneados, por favor aguarde.");
    translations.insert("Current scanning", "Scanning atual");
    translations.insert("Upgrading filesystem cache...", "Atualizando cache do sistema de arquivos...");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}

pub fn translate_plural(key: &str, count: i64) -> String {
    match key {
        "_%n folder_::_%n folders_" => {
            if count > 1 {
                format!("{} pastas", count)
            } else {
                format!("{} pasta", count)
            }
        },
        "_%n file_::_%n files_" => {
            if count > 1 {
                format!("{} arquivos", count)
            } else {
                format!("{} arquivo", count)
            }
        },
        "_Uploading %n file_::_Uploading %n files_" => {
            if count > 1 {
                format!("Enviando {} arquivos", count)
            } else {
                format!("Enviando {} arquivo", count)
            }
        },
        _ => format!("Unknown plural key: {}", key),
    }
}