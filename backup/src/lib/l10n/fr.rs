use phf::phf_map;
use rust_fluent::types::FluentNumber;
use std::collections::HashMap;

pub const FR_TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "App \"%s\" can't be installed because it is not compatible with this version of ownCloud." => "L'application \"%s\" ne peut être installée car elle n'est pas compatible avec cette version de ownCloud.",
    "No app name specified" => "Aucun nom d'application spécifié",
    "Help" => "Aide",
    "Personal" => "Personnel",
    "Settings" => "Paramètres",
    "Users" => "Utilisateurs",
    "Admin" => "Administration",
    "Failed to upgrade \"%s\"." => "Echec de la mise à niveau \"%s\".",
    "Unknown filetype" => "Type de fichier inconnu",
    "Invalid image" => "Image invalide",
    "web services under your control" => "services web sous votre contrôle",
    "cannot open \"%s\"" => "impossible d'ouvrir \"%s\"",
    "ZIP download is turned off." => "Téléchargement ZIP désactivé.",
    "Files need to be downloaded one by one." => "Les fichiers nécessitent d'être téléchargés un par un.",
    "Back to Files" => "Retour aux Fichiers",
    "Selected files too large to generate zip file." => "Les fichiers sélectionnés sont trop volumineux pour être compressés.",
    "Download the files in smaller chunks, seperately or kindly ask your administrator." => "Télécharger les fichiers en parties plus petites, séparément ou demander avec bienveillance à votre administrateur.",
    "No source specified when installing app" => "Aucune source spécifiée pour installer l'application",
    "No href specified when installing app from http" => "Aucun href spécifié pour installer l'application par http",
    "No path specified when installing app from local file" => "Aucun chemin spécifié pour installer l'application depuis un fichier local",
    "Archives of type %s are not supported" => "Les archives de type %s ne sont pas supportées",
    "Failed to open archive when installing app" => "Échec de l'ouverture de l'archive lors de l'installation de l'application",
    "App does not provide an info.xml file" => "L'application ne fournit pas de fichier info.xml",
    "App can't be installed because of not allowed code in the App" => "L'application ne peut être installée car elle contient du code non-autorisé",
    "App can't be installed because it is not compatible with this version of ownCloud" => "L'application ne peut être installée car elle n'est pas compatible avec cette version de ownCloud",
    "App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps" => "L'application ne peut être installée car elle contient la balise <shipped>true</shipped> qui n'est pas autorisée pour les applications non-diffusées",
    "App can't be installed because the version in info.xml/version is not the same as the version reported from the app store" => "L'application ne peut être installée car la version de info.xml/version n'est identique à celle indiquée sur l'app store",
    "App directory already exists" => "Le dossier de l'application existe déjà",
    "Can't create app folder. Please fix permissions. %s" => "Impossible de créer le dossier de l'application. Corrigez les droits d'accès. %s",
    "Application is not enabled" => "L'application n'est pas activée",
    "Authentication error" => "Erreur d'authentification",
    "Token expired. Please reload page." => "La session a expiré. Veuillez recharger la page.",
    "Files" => "Fichiers",
    "Text" => "Texte",
    "Images" => "Images",
    "%s enter the database username." => "%s entrez le nom d'utilisateur de la base de données.",
    "%s enter the database name." => "%s entrez le nom de la base de données.",
    "%s you may not use dots in the database name" => "%s vous nez pouvez pas utiliser de points dans le nom de la base de données",
    "MS SQL username and/or password not valid: %s" => "Le nom d'utilisateur et/ou le mot de passe de la base MS SQL est invalide : %s",
    "You need to enter either an existing account or the administrator." => "Vous devez spécifier soit le nom d'un compte existant, soit celui de l'administrateur.",
    "MySQL username and/or password not valid" => "Nom d'utilisateur et/ou mot de passe de la base MySQL invalide",
    "DB Error: \"%s\"" => "Erreur de la base de données : \"%s\"",
    "Offending command was: \"%s\"" => "La requête en cause est : \"%s\"",
    "MySQL user '%s'@'localhost' exists already." => "L'utilisateur MySQL '%s'@'localhost' existe déjà.",
    "Drop this user from MySQL" => "Retirer cet utilisateur de la base MySQL",
    "MySQL user '%s'@'%%' already exists" => "L'utilisateur MySQL '%s'@'%%' existe déjà",
    "Drop this user from MySQL." => "Retirer cet utilisateur de la base MySQL.",
    "Oracle connection could not be established" => "La connexion Oracle ne peut pas être établie",
    "Oracle username and/or password not valid" => "Nom d'utilisateur et/ou mot de passe de la base Oracle invalide",
    "Offending command was: \"%s\", name: %s, password: %s" => "La requête en cause est : \"%s\", nom : %s, mot de passe : %s",
    "PostgreSQL username and/or password not valid" => "Nom d'utilisateur et/ou mot de passe de la base PostgreSQL invalide",
    "Set an admin username." => "Spécifiez un nom d'utilisateur pour l'administrateur.",
    "Set an admin password." => "Spécifiez un mot de passe administrateur.",
    "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken." => "Votre serveur web, n'est pas correctement configuré pour permettre la synchronisation des fichiers, car l'interface WebDav ne fonctionne pas comme il faut.",
    "Please double check the <a href='%s'>installation guides</a>." => "Veuillez vous référer au <a href='%s'>guide d'installation</a>.",
    "Could not find category \"%s\"" => "Impossible de trouver la catégorie \"%s\"",
    "seconds ago" => "il y a quelques secondes",
    "today" => "aujourd'hui",
    "yesterday" => "hier",
    "last month" => "le mois dernier",
    "last year" => "l'année dernière",
    "years ago" => "il y a plusieurs années",
    "Caused by:" => "Causé par :",
};

pub fn get_plural_form(n: FluentNumber) -> usize {
    if n.value > 1.0 {
        1
    } else {
        0
    }
}

lazy_static::lazy_static! {
    pub static ref FR_PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("_%n minute ago_::_%n minutes ago_", vec!["", "il y a %n minutes"]);
        map.insert("_%n hour ago_::_%n hours ago_", vec!["", "Il y a %n heures"]);
        map.insert("_%n day go_::_%n days ago_", vec!["", "il y a %n jours"]);
        map.insert("_%n month ago_::_%n months ago_", vec!["", "Il y a %n mois"]);
        map
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    FR_TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, n: FluentNumber) -> Option<&'static str> {
    let form = get_plural_form(n);
    FR_PLURAL_TRANSLATIONS.get(key).and_then(|forms| forms.get(form).copied())
}