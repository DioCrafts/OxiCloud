use std::collections::HashMap;
use rust_i18n::i18n;

// Portuguese (Portugal) translations
pub fn get_pt_pt_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists".to_string(), "Não foi possível mover o ficheiro %s - Já existe um ficheiro com esse nome".to_string());
    translations.insert("Could not move %s".to_string(), "Não foi possível move o ficheiro %s".to_string());
    translations.insert("File name cannot be empty.".to_string(), "O nome do ficheiro não pode estar vazio.".to_string());
    translations.insert("Unable to set upload directory.".to_string(), "Não foi possível criar o diretório de upload".to_string());
    translations.insert("Invalid Token".to_string(), "Token inválido".to_string());
    translations.insert("No file was uploaded. Unknown error".to_string(), "Nenhum ficheiro foi carregado. Erro desconhecido".to_string());
    translations.insert("There is no error, the file uploaded with success".to_string(), "Não ocorreram erros, o ficheiro foi submetido com sucesso".to_string());
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(), "O ficheiro enviado excede o limite permitido na directiva do php.ini upload_max_filesize".to_string());
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(), "O tamanho do ficheiro carregado ultrapassa o valor MAX_FILE_SIZE definido no formulário HTML".to_string());
    translations.insert("The uploaded file was only partially uploaded".to_string(), "O ficheiro seleccionado foi apenas carregado parcialmente".to_string());
    translations.insert("No file was uploaded".to_string(), "Nenhum ficheiro foi submetido".to_string());
    translations.insert("Missing a temporary folder".to_string(), "Está a faltar a pasta temporária".to_string());
    translations.insert("Failed to write to disk".to_string(), "Falhou a escrita no disco".to_string());
    translations.insert("Not enough storage available".to_string(), "Não há espaço suficiente em disco".to_string());
    translations.insert("Upload failed. Could not get file info.".to_string(), "O carregamento falhou. Não foi possível obter a informação do ficheiro.".to_string());
    translations.insert("Invalid directory.".to_string(), "Directório Inválido".to_string());
    translations.insert("Files".to_string(), "Ficheiros".to_string());
    translations.insert("Not enough space available".to_string(), "Espaço em disco insuficiente!".to_string());
    translations.insert("Upload cancelled.".to_string(), "Envio cancelado.".to_string());
    translations.insert("Could not get result from server.".to_string(), "Não foi possível obter o resultado do servidor.".to_string());
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.".to_string(), "Envio de ficheiro em progresso. Irá cancelar o envio se sair da página agora.".to_string());
    translations.insert("{new_name} already exists".to_string(), "O nome {new_name} já existe".to_string());
    translations.insert("Share".to_string(), "Partilhar".to_string());
    translations.insert("Delete permanently".to_string(), "Eliminar permanentemente".to_string());
    translations.insert("Rename".to_string(), "Renomear".to_string());
    translations.insert("Pending".to_string(), "Pendente".to_string());
    translations.insert("replaced {new_name} with {old_name}".to_string(), "substituido {new_name} por {old_name}".to_string());
    translations.insert("undo".to_string(), "desfazer".to_string());
    translations.insert("{dirs} and {files}".to_string(), "{dirs} e {files}".to_string());
    translations.insert("'.' is an invalid file name.".to_string(), "'.' não é um nome de ficheiro válido!".to_string());
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(), "Nome Inválido, os caracteres '\\', '/', '<', '>', ':', '\"', '|', '?' e '*' não são permitidos.".to_string());
    translations.insert("Your storage is full, files can not be updated or synced anymore!".to_string(), "O seu armazenamento está cheio, os ficheiros não podem ser sincronizados.".to_string());
    translations.insert("Your storage is almost full ({usedSpacePercent}%)".to_string(), "O seu espaço de armazenamento está quase cheiro ({usedSpacePercent}%)".to_string());
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.".to_string(), "A encriptação foi desactivada mas os seus ficheiros continuam encriptados.  Por favor consulte as suas definições pessoais para desencriptar os ficheiros.".to_string());
    translations.insert("Your download is being prepared. This might take some time if the files are big.".to_string(), "O seu download está a ser preparado. Este processo pode demorar algum tempo se os ficheiros forem grandes.".to_string());
    translations.insert("Error moving file".to_string(), "Erro ao mover o ficheiro".to_string());
    translations.insert("Error".to_string(), "Erro".to_string());
    translations.insert("Name".to_string(), "Nome".to_string());
    translations.insert("Size".to_string(), "Tamanho".to_string());
    translations.insert("Modified".to_string(), "Modificado".to_string());
    translations.insert("%s could not be renamed".to_string(), "%s não pode ser renomeada".to_string());
    translations.insert("Upload".to_string(), "Carregar".to_string());
    translations.insert("File handling".to_string(), "Manuseamento de ficheiros".to_string());
    translations.insert("Maximum upload size".to_string(), "Tamanho máximo de envio".to_string());
    translations.insert("max. possible: ".to_string(), "max. possivel: ".to_string());
    translations.insert("Needed for multi-file and folder downloads.".to_string(), "Necessário para multi download de ficheiros e pastas".to_string());
    translations.insert("Enable ZIP-download".to_string(), "Permitir descarregar em ficheiro ZIP".to_string());
    translations.insert("0 is unlimited".to_string(), "0 é ilimitado".to_string());
    translations.insert("Maximum input size for ZIP files".to_string(), "Tamanho máximo para ficheiros ZIP".to_string());
    translations.insert("Save".to_string(), "Guardar".to_string());
    translations.insert("New".to_string(), "Novo".to_string());
    translations.insert("Text file".to_string(), "Ficheiro de texto".to_string());
    translations.insert("Folder".to_string(), "Pasta".to_string());
    translations.insert("From link".to_string(), "Da ligação".to_string());
    translations.insert("Deleted files".to_string(), "Ficheiros eliminados".to_string());
    translations.insert("Cancel upload".to_string(), "Cancelar envio".to_string());
    translations.insert("Nothing in here. Upload something!".to_string(), "Vazio. Envie alguma coisa!".to_string());
    translations.insert("Download".to_string(), "Transferir".to_string());
    translations.insert("Unshare".to_string(), "Deixar de partilhar".to_string());
    translations.insert("Delete".to_string(), "Eliminar".to_string());
    translations.insert("Upload too large".to_string(), "Upload muito grande".to_string());
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(), "Os ficheiro que está a tentar enviar excedem o tamanho máximo de envio neste servidor.".to_string());
    translations.insert("Files are being scanned, please wait.".to_string(), "Os ficheiros estão a ser analisados, por favor aguarde.".to_string());
    translations.insert("Current scanning".to_string(), "Análise actual".to_string());
    translations.insert("Upgrading filesystem cache...".to_string(), "Atualizar cache do sistema de ficheiros...".to_string());
    
    // Plural forms
    translations.insert("_%n folder_::_%n folders_".to_string(), "%n pasta|%n pastas".to_string());
    translations.insert("_%n file_::_%n files_".to_string(), "%n ficheiro|%n ficheiros".to_string());
    translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), "A carregar %n ficheiro|A carregar %n ficheiros".to_string());
    
    translations
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_translations() {
        let translations = get_pt_pt_translations();
        assert!(translations.contains_key("Files"));
        assert_eq!(translations.get("Files").unwrap(), "Ficheiros");
    }

    #[test]
    fn test_get_plural_form() {
        assert_eq!(get_plural_form(), "nplurals=2; plural=(n != 1);");
    }
}