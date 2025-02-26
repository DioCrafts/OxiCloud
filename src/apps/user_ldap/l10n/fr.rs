use std::collections::HashMap;
use once_cell::sync::Lazy;

// Definición de las traducciones
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Failed to clear the mappings.", "Erreur lors de la suppression des associations.");
    translations.insert("Failed to delete the server configuration", "Échec de la suppression de la configuration du serveur");
    translations.insert("The configuration is valid and the connection could be established!", "La configuration est valide et la connexion peut être établie !");
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "La configuration est valide, mais le lien ne peut être établi. Veuillez vérifier les paramètres du serveur ainsi que vos identifiants de connexion.");
    translations.insert("The configuration is invalid. Please have a look at the logs for further details.", "La configuration est invalide. Veuillez consulter les logs pour plus de détails.");
    translations.insert("No action specified", "Aucune action spécifiée");
    translations.insert("No configuration specified", "Aucune configuration spécifiée");
    translations.insert("No data specified", "Aucune donnée spécifiée");
    translations.insert(" Could not set configuration %s", "Impossible de spécifier la configuration %s");
    translations.insert("Deletion failed", "La suppression a échoué");
    translations.insert("Take over settings from recent server configuration?", "Récupérer les paramètres depuis une configuration récente du serveur ?");
    translations.insert("Keep settings?", "Garder ces paramètres ?");
    translations.insert("Cannot add server configuration", "Impossible d'ajouter la configuration du serveur");
    translations.insert("mappings cleared", "associations supprimées");
    translations.insert("Success", "Succès");
    translations.insert("Error", "Erreur");
    translations.insert("Select groups", "Sélectionnez les groupes");
    translations.insert("Select object classes", "Sélectionner les classes d'objet");
    translations.insert("Select attributes", "Sélectionner les attributs");
    translations.insert("Connection test succeeded", "Test de connexion réussi");
    translations.insert("Connection test failed", "Test de connexion échoué");
    translations.insert("Do you really want to delete the current Server Configuration?", "Êtes-vous vraiment sûr de vouloir effacer la configuration actuelle du serveur ?");
    translations.insert("Confirm Deletion", "Confirmer la suppression");
    translations.insert("Invalid Host", "Hôte invalide");
    translations.insert("Could not find the desired feature", "Impossible de trouver la fonction souhaitée");
    translations.insert("Save", "Sauvegarder");
    translations.insert("Test Configuration", "Tester la configuration");
    translations.insert("Help", "Aide");
    translations.insert("only those object classes:", "seulement ces classes d'objet :");
    translations.insert("only from those groups:", "seulement de ces groupes :");
    translations.insert("Edit raw filter instead", "Éditer le filtre raw à la place");
    translations.insert("Raw LDAP filter", "Filtre Raw LDAP");
    translations.insert("groups found", "groupes trouvés");
    translations.insert("What attribute shall be used as login name:", "Quel attribut doit être utilisé comme nom de login:");
    translations.insert("LDAP Username:", "Nom d'utilisateur LDAP :");
    translations.insert("LDAP Email Address:", "Adresse email LDAP :");
    translations.insert("Other Attributes:", "Autres attributs :");
    translations.insert("Add Server Configuration", "Ajouter une configuration du serveur");
    translations.insert("Host", "Hôte");
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Vous pouvez omettre le protocole, sauf si vous avez besoin de SSL. Dans ce cas préfixez avec ldaps://");
    translations.insert("Port", "Port");
    translations.insert("User DN", "DN Utilisateur (Autorisé à consulter l'annuaire)");
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN de l'utilisateur client pour lequel la liaison doit se faire, par exemple uid=agent,dc=example,dc=com. Pour un accès anonyme, laisser le DN et le mot de passe vides.");
    translations.insert("Password", "Mot de passe");
    translations.insert("For anonymous access, leave DN and Password empty.", "Pour un accès anonyme, laisser le DN utilisateur et le mot de passe vides.");
    translations.insert("One Base DN per line", "Un DN racine par ligne");
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", "Vous pouvez spécifier les DN Racines de vos utilisateurs et groupes via l'onglet Avancé");
    translations.insert("users found", "utilisateurs trouvés");
    translations.insert("Back", "Retour");
    translations.insert("Continue", "Poursuivre");
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Avertissement :</b> Les applications user_ldap et user_webdavauth sont incompatibles. Des dysfonctionnements peuvent survenir. Contactez votre administrateur système pour qu'il désactive l'une d'elles.");
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Attention :</b> Le module php LDAP n'est pas installé, par conséquent cette extension ne pourra fonctionner. Veuillez contacter votre administrateur système afin qu'il l'installe.");
    translations.insert("Connection Settings", "Paramètres de connexion");
    translations.insert("Configuration Active", "Configuration active");
    translations.insert("When unchecked, this configuration will be skipped.", "Lorsque non cochée, la configuration sera ignorée.");
    translations.insert("User Login Filter", "Modèle d'authentification utilisateurs");
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Définit le filtre à appliquer lors d'une tentative de connexion. %%uid remplace le nom d'utilisateur lors de la connexion. Exemple : \"uid=%%uid\"");
    translations.insert("Backup (Replica) Host", "Serveur de backup (réplique)");
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Fournir un serveur de backup optionnel.  Il doit s'agir d'une réplique du serveur LDAP/AD principal.");
    translations.insert("Backup (Replica) Port", "Port du serveur de backup (réplique)");
    translations.insert("Disable Main Server", "Désactiver le serveur principal");
    translations.insert("Only connect to the replica server.", "Se connecter uniquement au serveur de replica.");
    translations.insert("Case insensitve LDAP server (Windows)", "Serveur LDAP insensible à la casse (Windows)");
    translations.insert("Turn off SSL certificate validation.", "Désactiver la validation du certificat SSL.");
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Non recommandé, à utiliser à des fins de tests uniquement. Si la connexion ne fonctionne qu'avec cette option, importez le certificat SSL du serveur LDAP dans le serveur %s.");
    translations.insert("Cache Time-To-Live", "Durée de vie du cache");
    translations.insert("in seconds. A change empties the cache.", "en secondes. Tout changement vide le cache.");
    translations.insert("Directory Settings", "Paramètres du répertoire");
    translations.insert("User Display Name Field", "Champ \"nom d'affichage\" de l'utilisateur");
    translations.insert("The LDAP attribute to use to generate the user's display name.", "L'attribut LDAP utilisé pour générer le nom d'utilisateur affiché.");
    translations.insert("Base User Tree", "DN racine de l'arbre utilisateurs");
    translations.insert("One User Base DN per line", "Un DN racine utilisateur par ligne");
    translations.insert("User Search Attributes", "Recherche des attributs utilisateur");
    translations.insert("Optional; one attribute per line", "Optionnel, un attribut par ligne");
    translations.insert("Group Display Name Field", "Champ \"nom d'affichage\" du groupe");
    translations.insert("The LDAP attribute to use to generate the groups's display name.", "L'attribut LDAP utilisé pour générer le nom de groupe affiché.");
    translations.insert("Base Group Tree", "DN racine de l'arbre groupes");
    translations.insert("One Group Base DN per line", "Un DN racine groupe par ligne");
    translations.insert("Group Search Attributes", "Recherche des attributs du groupe");
    translations.insert("Group-Member association", "Association groupe-membre");
    translations.insert("Special Attributes", "Attributs spéciaux");
    translations.insert("Quota Field", "Champ du quota");
    translations.insert("Quota Default", "Quota par défaut");
    translations.insert("in bytes", "en bytes");
    translations.insert("Email Field", "Champ Email");
    translations.insert("User Home Folder Naming Rule", "Convention de nommage du répertoire utilisateur");
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Laisser vide ");
    translations.insert("Internal Username", "Nom d'utilisateur interne");
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Par défaut le nom d'utilisateur interne sera créé à partir de l'attribut UUID. Ceci permet d'assurer que le nom d'utilisateur est unique et que les caractères ne nécessitent pas de conversion. Le nom d'utilisateur interne doit contenir uniquement les caractères suivants : [ a-zA-Z0-9_.@- ]. Les autres caractères sont remplacés par leur correspondance ASCII ou simplement omis. En cas de collision, un nombre est incrémenté/décrémenté. Le nom d'utilisateur interne est utilisé pour identifier l'utilisateur au sein du système. C'est aussi le nom par défaut du répertoire utilisateur dans ownCloud. C'est aussi le port d'URLs distants, par exemple pour tous les services *DAV. Le comportement par défaut peut être modifié à l'aide de ce paramètre. Pour obtenir un comportement similaire aux versions précédentes à ownCloud 5, saisir le nom d'utilisateur à afficher dans le champ suivant. Laissez à blanc pour le comportement par défaut. Les modifications prendront effet seulement pour les nouveaux (ajoutés) utilisateurs LDAP.");
    translations.insert("Internal Username Attribute:", "Nom d'utilisateur interne:");
    translations.insert("Override UUID detection", "Surcharger la détection d'UUID");
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Par défaut, l'attribut UUID est automatiquement  détecté. Cet attribut est utilisé pour identifier les utilisateurs et groupes de façon fiable. Un nom d'utilisateur interne basé sur l'UUID sera automatiquement créé, sauf s'il est spécifié autrement ci-dessus. Vous pouvez modifier ce comportement et définir l'attribut de votre choix. Vous devez alors vous assurer que l'attribut de votre choix peut être récupéré pour les utilisateurs ainsi que pour les groupes et qu'il soit unique. Laisser à blanc pour le comportement par défaut. Les modifications seront effectives uniquement pour les nouveaux (ajoutés) utilisateurs et groupes LDAP.");
    translations.insert("UUID Attribute for Users:", "Attribut UUID pour les utilisateurs :");
    translations.insert("UUID Attribute for Groups:", "Attribut UUID pour les groupes :");
    translations.insert("Username-LDAP User Mapping", "Association Nom d'utilisateur-Utilisateur LDAP");
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "Les noms d'utilisateurs sont utilisés pour le stockage et l'assignation de (meta) données. Pour identifier et reconnaitre précisément les utilisateurs, chaque utilisateur LDAP aura un nom interne spécifique. Cela requiert l'association d'un nom d'utilisateur ownCloud à un nom d'utilisateur LDAP. Le nom d'utilisateur créé est associé à l'attribut UUID de l'utilisateur LDAP. Par ailleurs, le DN est mémorisé en cache pour limiter les interactions LDAP mais il n'est pas utilisé pour l'identification. Si le DN est modifié, ces modifications seront retrouvées. Seul le nom interne à ownCloud est utilisé au sein du produit. Supprimer les associations créera des orphelins et l'action affectera toutes les configurations LDAP. NE JAMAIS SUPPRIMER LES ASSOCIATIONS EN ENVIRONNEMENT DE PRODUCTION, mais uniquement sur des environnements de tests et d'expérimentation.");
    translations.insert("Clear Username-LDAP User Mapping", "Supprimer l'association utilisateur interne-utilisateur LDAP");
    translations.insert("Clear Groupname-LDAP Group Mapping", "Supprimer l'association nom de groupe-groupe LDAP");
    
    // Agregar traducciones con patrones
    translations.insert("Limit the access to %s to groups meeting this criteria:", "Limiter l'accès à %s aux groupes respectant ce critère :");
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.", "Le filtre spécifie quels groupes LDAP doivent avoir accès à l'instance %s.");
    translations.insert("Limit the access to %s to users meeting this criteria:", "Limiter l'accès à %s aux utilisateurs respectant ce critère :");
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.", "Le filtre spécifie quels utilisateurs LDAP doivent avoir accès à l'instance %s.");
    
    translations
});

// Definición de las traducciones plurales
pub static PLURAL_TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%s group found_::_%s groups found_", vec!["%s groupe trouvé", "%s groupes trouvés"]);
    plural_translations.insert("_%s user found_::_%s users found_", vec!["%s utilisateur trouvé", "%s utilisateurs trouvés"]);
    plural_translations
});

// Función de mapeo de plurales
pub fn get_plural_form(n: usize) -> usize {
    if n > 1 { 1 } else { 0 }
}

// Funciones auxiliares para traducciones
pub fn translate(text: &str) -> &'static str {
    TRANSLATIONS.get(text).copied().unwrap_or(text)
}

pub fn translate_plural(text: &str, count: usize) -> String {
    if let Some(forms) = PLURAL_TRANSLATIONS.get(text) {
        let form_index = get_plural_form(count);
        let form = forms.get(form_index).unwrap_or(&forms[0]);
        form.replace("%s", &count.to_string())
    } else {
        text.to_string()
    }
}

pub fn translate_with_params(text: &str, params: &[&str]) -> String {
    let mut result = translate(text).to_string();
    for (i, param) in params.iter().enumerate() {
        result = result.replace(&format!("%{}", i + 1), param);
        result = result.replace(&format!("%s"), param); // Solo reemplaza el primer %s
    }
    result
}