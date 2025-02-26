use std::collections::HashMap;
use rust_i18n::locale_file;

locale_file!("fr.rs");

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Could not move %s - File with this name already exists", "Impossible de déplacer %s - Un fichier possédant ce nom existe déjà");
    translations.insert("Could not move %s", "Impossible de déplacer %s");
    translations.insert("File name cannot be empty.", "Le nom de fichier ne peut être vide.");
    translations.insert("File name must not contain \"/\". Please choose a different name.", "Le nom de fichier ne doit pas contenir \"/\". Merci de choisir un nom différent.");
    translations.insert("The name %s is already used in the folder %s. Please choose a different name.", "Le nom %s est déjà utilisé dans le dossier %s. Merci de choisir un nom différent.");
    translations.insert("Not a valid source", "La source n'est pas valide");
    translations.insert("Error while downloading %s to %s", "Erreur pendant le téléchargement de %s à %s");
    translations.insert("Error when creating the file", "Erreur pendant la création du fichier");
    translations.insert("Folder name cannot be empty.", "Le nom de dossier ne peux pas être vide.");
    translations.insert("Folder name must not contain \"/\". Please choose a different name.", "Le nom de dossier ne doit pas contenir \"/\". Merci de choisir un nom différent.");
    translations.insert("Error when creating the folder", "Erreur pendant la création du dossier");
    translations.insert("Unable to set upload directory.", "Impossible de définir le dossier pour l'upload, charger.");
    translations.insert("Invalid Token", "Jeton non valide");
    translations.insert("No file was uploaded. Unknown error", "Aucun fichier n'a été envoyé. Erreur inconnue");
    translations.insert("There is no error, the file uploaded with success", "Aucune erreur, le fichier a été envoyé avec succès.");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Le fichier envoyé dépasse l'instruction upload_max_filesize située dans le fichier php.ini:");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Le fichier envoyé dépasse l'instruction MAX_FILE_SIZE qui est spécifiée dans le formulaire HTML.");
    translations.insert("The uploaded file was only partially uploaded", "Le fichier n'a été que partiellement envoyé.");
    translations.insert("No file was uploaded", "Pas de fichier envoyé.");
    translations.insert("Missing a temporary folder", "Absence de dossier temporaire.");
    translations.insert("Failed to write to disk", "Erreur d'écriture sur le disque");
    translations.insert("Not enough storage available", "Plus assez d'espace de stockage disponible");
    translations.insert("Upload failed. Could not get file info.", "L'envoi a échoué. Impossible d'obtenir les informations du fichier.");
    translations.insert("Upload failed. Could not find uploaded file", "L'envoi a échoué. Impossible de trouver le fichier envoyé.");
    translations.insert("Invalid directory.", "Dossier invalide.");
    translations.insert("Files", "Fichiers");
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Impossible d'envoyer {filename} car il s'agit d'un répertoire ou d'un fichier de taille nulle");
    translations.insert("Not enough space available", "Espace disponible insuffisant");
    translations.insert("Upload cancelled.", "Envoi annulé.");
    translations.insert("Could not get result from server.", "Ne peut recevoir les résultats du serveur.");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "L'envoi du fichier est en cours. Quitter cette page maintenant annulera l'envoi du fichier.");
    translations.insert("URL cannot be empty", "L'URL ne peut pas être vide");
    translations.insert("In the home folder 'Shared' is a reserved filename", "Dans le dossier home, 'Partagé' est un nom de fichier réservé");
    translations.insert("{new_name} already exists", "{new_name} existe déjà");
    translations.insert("Could not create file", "Impossible de créer le fichier");
    translations.insert("Could not create folder", "Impossible de créer le dossier");
    translations.insert("Share", "Partager");
    translations.insert("Delete permanently", "Supprimer de façon définitive");
    translations.insert("Rename", "Renommer");
    translations.insert("Pending", "En attente");
    translations.insert("Could not rename file", "Impossible de renommer le fichier");
    translations.insert("replaced {new_name} with {old_name}", "{new_name} a été remplacé par {old_name}");
    translations.insert("undo", "annuler");
    translations.insert("'.' is an invalid file name.", "'.' n'est pas un nom de fichier valide.");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nom invalide, les caractères '\\', '/', '<', '>', ':', '\"', '|', '?' et '*' ne sont pas autorisés.");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "Votre espage de stockage est plein, les fichiers ne peuvent plus être téléversés ou synchronisés !");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "Votre espace de stockage est presque plein ({usedSpacePercent}%)");
    translations.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "L'application de chiffrement est activée mais vos clés ne sont pas initialisées, veuillez vous déconnecter et ensuite vous reconnecter.");
    translations.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Votre clef privée pour l'application de chiffrement est invalide ! Veuillez mettre à jour le mot de passe de votre clef privée dans vos paramètres personnels pour récupérer l'accès à vos fichiers chiffrés.");
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Le chiffrement était désactivé mais vos fichiers sont toujours chiffrés. Veuillez vous rendre sur vos Paramètres personnels pour déchiffrer vos fichiers.");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "Votre téléchargement est cours de préparation. Ceci peut nécessiter un certain temps si les fichiers sont volumineux.");
    translations.insert("Error moving file", "Erreur lors du déplacement du fichier");
    translations.insert("Error", "Erreur");
    translations.insert("Name", "Nom");
    translations.insert("Size", "Taille");
    translations.insert("Modified", "Modifié");
    translations.insert("Invalid folder name. Usage of 'Shared' is reserved.", "Nom de dossier invalide. L'utilisation du mot 'Shared' est réservée.");
    translations.insert("%s could not be renamed", "%s ne peut être renommé");
    translations.insert("Upload", "Envoyer");
    translations.insert("File handling", "Gestion des fichiers");
    translations.insert("Maximum upload size", "Taille max. d'envoi");
    translations.insert("max. possible: ", "Max. possible :");
    translations.insert("Needed for multi-file and folder downloads.", "Nécessaire pour le téléchargement de plusieurs fichiers et de dossiers.");
    translations.insert("Enable ZIP-download", "Activer le téléchargement ZIP");
    translations.insert("0 is unlimited", "0 est illimité");
    translations.insert("Maximum input size for ZIP files", "Taille maximale pour les fichiers ZIP");
    translations.insert("Save", "Sauvegarder");
    translations.insert("New", "Nouveau");
    translations.insert("Text file", "Fichier texte");
    translations.insert("Folder", "Dossier");
    translations.insert("From link", "Depuis le lien");
    translations.insert("Deleted files", "Fichiers supprimés");
    translations.insert("Cancel upload", "Annuler l'envoi");
    translations.insert("You don't have permission to upload or create files here", "Vous n'avez pas la permission de téléverser ou de créer des fichiers ici");
    translations.insert("Nothing in here. Upload something!", "Il n'y a rien ici ! Envoyez donc quelque chose :)");
    translations.insert("Download", "Télécharger");
    translations.insert("Unshare", "Ne plus partager");
    translations.insert("Delete", "Supprimer");
    translations.insert("Upload too large", "Téléversement trop volumineux");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Les fichiers que vous essayez d'envoyer dépassent la taille maximale permise par ce serveur.");
    translations.insert("Files are being scanned, please wait.", "Les fichiers sont en cours d'analyse, veuillez patienter.");
    translations.insert("Current scanning", "Analyse en cours");
    translations.insert("Upgrading filesystem cache...", "Mise à niveau du cache du système de fichier");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}

pub fn translate_plural(singular: &str, plural: &str, count: i64) -> String {
    if count > 1 {
        match singular {
            "_%n folder_::_%n folders_" => format!("{} dossiers", count),
            "_%n file_::_%n files_" => format!("{} fichiers", count),
            "_Uploading %n file_::_Uploading %n files_" => format!("Téléversement de {} fichiers", count),
            _ => plural.to_string(),
        }
    } else {
        match singular {
            "_%n folder_::_%n folders_" => format!("{} dossier", count),
            "_%n file_::_%n files_" => format!("{} fichier", count),
            "_Uploading %n file_::_Uploading %n files_" => format!("Téléversement de {} fichier", count),
            _ => singular.to_string(),
        }
    }
}

pub fn translate_text_with_vars(key: &str, vars: &HashMap<&str, &str>) -> String {
    let translations = get_translations();
    let text = translations.get(key).unwrap_or(&key);
    
    let mut result = text.to_string();
    for (var_name, var_value) in vars {
        result = result.replace(&format!("{{{}}}", var_name), var_value);
    }
    
    result
}