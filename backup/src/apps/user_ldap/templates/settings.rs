use askama::Template;
use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "ldap_settings.html")]
struct LdapSettingsTemplate {
    toc: HashMap<String, String>,
    tabs: String,
    ldap_configuration_active_default: String,
    ldap_login_filter_default: String,
    ldap_backup_host_default: String,
    ldap_backup_port_default: String,
    ldap_override_main_server_default: String,
    ldap_nocase_default: String,
    ldap_turn_off_cert_check_default: String,
    ldap_cache_ttl_default: String,
    ldap_display_name_default: String,
    ldap_base_users_default: String,
    ldap_attributes_for_user_search_default: String,
    ldap_group_display_name_default: String,
    ldap_base_groups_default: String,
    ldap_attributes_for_group_search_default: String,
    ldap_group_member_assoc_attribute_default: String,
    ldap_quota_attr_default: String,
    ldap_quota_def_default: String,
    ldap_email_attr_default: String,
    home_folder_naming_rule_default: String,
    settingControls: String,
    ldap_expert_username_attr_default: String,
    ldap_expert_uuid_user_attr_default: String,
    ldap_expert_uuid_group_attr_default: String,
    ldap_nocase: Option<bool>,
    ldap_group_member_assoc_attribute: Option<String>,
    theme_name: String,
    translatable: TranslatableStrings,
    webdavauth_enabled: bool,
    ldap_extension_loaded: bool,
}

#[derive(Serialize)]
struct TranslatableStrings {
    webdavauth_incompatible_warning: String,
    ldap_module_not_installed_warning: String,
    connection_settings: String,
    configuration_active: String,
    configuration_active_description: String,
    user_login_filter: String,
    user_login_filter_description: String,
    backup_replica_host: String,
    backup_replica_host_description: String,
    backup_replica_port: String,
    disable_main_server: String,
    disable_main_server_description: String,
    case_insensitive_ldap_server: String,
    turn_off_ssl_cert_validation: String,
    turn_off_ssl_cert_validation_description: String,
    cache_ttl: String,
    cache_ttl_description: String,
    directory_settings: String,
    user_display_name_field: String,
    user_display_name_field_description: String,
    base_user_tree: String,
    one_user_base_dn_per_line: String,
    user_search_attributes: String,
    optional_one_attribute_per_line: String,
    group_display_name_field: String,
    group_display_name_field_description: String,
    base_group_tree: String,
    one_group_base_dn_per_line: String,
    group_search_attributes: String,
    group_member_association: String,
    special_attributes: String,
    quota_field: String,
    quota_default: String,
    quota_default_description: String,
    email_field: String,
    user_home_folder_naming_rule: String,
    user_home_folder_naming_rule_description: String,
    internal_username: String,
    internal_username_description: String,
    internal_username_attribute: String,
    override_uuid_detection: String,
    override_uuid_detection_description: String,
    uuid_attribute_for_users: String,
    uuid_attribute_for_groups: String,
    username_ldap_user_mapping: String,
    username_ldap_user_mapping_description: String,
    clear_username_ldap_user_mapping: String,
    clear_groupname_ldap_group_mapping: String,
}

struct AppState {
    translator: Translator,
    app_manager: AppManager,
    theme: Theme,
}

struct Translator {
    // Traducción simplificada para este ejemplo
    translations: HashMap<String, String>,
}

impl Translator {
    fn t(&self, key: &str) -> String {
        self.translations.get(key).cloned().unwrap_or_else(|| key.to_string())
    }
}

struct AppManager {
    enabled_apps: Vec<String>,
}

impl AppManager {
    fn is_enabled(&self, app_name: &str) -> bool {
        self.enabled_apps.contains(&app_name.to_string())
    }
}

struct Theme {
    name: String,
}

impl Theme {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

async fn ldap_settings(State(state): State<AppState>) -> Html<String> {
    // Verificar si se ha cargado la extensión LDAP de PHP
    let ldap_extension_loaded = cfg!(feature = "ldap");
    
    let mut toc = HashMap::new();
    toc.insert("ldapSettings-0".to_string(), "Server".to_string());
    toc.insert("ldapSettings-1".to_string(), "Advanced".to_string());
    toc.insert("ldapSettings-2".to_string(), "Expert".to_string());

    let template = LdapSettingsTemplate {
        toc,
        tabs: "<div>Tabs content here</div>".to_string(),
        ldap_configuration_active_default: "1".to_string(),
        ldap_login_filter_default: "uid=%uid".to_string(),
        ldap_backup_host_default: "".to_string(),
        ldap_backup_port_default: "389".to_string(),
        ldap_override_main_server_default: "0".to_string(),
        ldap_nocase_default: "0".to_string(),
        ldap_turn_off_cert_check_default: "0".to_string(),
        ldap_cache_ttl_default: "600".to_string(),
        ldap_display_name_default: "displayName".to_string(),
        ldap_base_users_default: "ou=users,dc=example,dc=com".to_string(),
        ldap_attributes_for_user_search_default: "".to_string(),
        ldap_group_display_name_default: "cn".to_string(),
        ldap_base_groups_default: "ou=groups,dc=example,dc=com".to_string(),
        ldap_attributes_for_group_search_default: "".to_string(),
        ldap_group_member_assoc_attribute_default: "uniqueMember".to_string(),
        ldap_quota_attr_default: "".to_string(),
        ldap_quota_def_default: "".to_string(),
        ldap_email_attr_default: "mail".to_string(),
        home_folder_naming_rule_default: "".to_string(),
        settingControls: "<div class='settingControls'><button type='button'>Save</button></div>".to_string(),
        ldap_expert_username_attr_default: "".to_string(),
        ldap_expert_uuid_user_attr_default: "".to_string(),
        ldap_expert_uuid_group_attr_default: "".to_string(),
        ldap_nocase: Some(false),
        ldap_group_member_assoc_attribute: Some("member".to_string()),
        theme_name: state.theme.get_name(),
        translatable: get_translatable_strings(&state.translator),
        webdavauth_enabled: state.app_manager.is_enabled("user_webdavauth"),
        ldap_extension_loaded,
    };

    Html(template.render().unwrap_or_else(|err| {
        format!("Error rendering template: {}", err)
    }))
}

fn get_translatable_strings(translator: &Translator) -> TranslatableStrings {
    TranslatableStrings {
        webdavauth_incompatible_warning: translator.t("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them."),
        ldap_module_not_installed_warning: translator.t("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it."),
        connection_settings: translator.t("Connection Settings"),
        configuration_active: translator.t("Configuration Active"),
        configuration_active_description: translator.t("When unchecked, this configuration will be skipped."),
        user_login_filter: translator.t("User Login Filter"),
        user_login_filter_description: translator.t("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\""),
        backup_replica_host: translator.t("Backup (Replica) Host"),
        backup_replica_host_description: translator.t("Give an optional backup host. It must be a replica of the main LDAP/AD server."),
        backup_replica_port: translator.t("Backup (Replica) Port"),
        disable_main_server: translator.t("Disable Main Server"),
        disable_main_server_description: translator.t("Only connect to the replica server."),
        case_insensitive_ldap_server: translator.t("Case insensitve LDAP server (Windows)"),
        turn_off_ssl_cert_validation: translator.t("Turn off SSL certificate validation."),
        turn_off_ssl_cert_validation_description: translator.t("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server."),
        cache_ttl: translator.t("Cache Time-To-Live"),
        cache_ttl_description: translator.t("in seconds. A change empties the cache."),
        directory_settings: translator.t("Directory Settings"),
        user_display_name_field: translator.t("User Display Name Field"),
        user_display_name_field_description: translator.t("The LDAP attribute to use to generate the user's display name."),
        base_user_tree: translator.t("Base User Tree"),
        one_user_base_dn_per_line: translator.t("One User Base DN per line"),
        user_search_attributes: translator.t("User Search Attributes"),
        optional_one_attribute_per_line: translator.t("Optional; one attribute per line"),
        group_display_name_field: translator.t("Group Display Name Field"),
        group_display_name_field_description: translator.t("The LDAP attribute to use to generate the groups's display name."),
        base_group_tree: translator.t("Base Group Tree"),
        one_group_base_dn_per_line: translator.t("One Group Base DN per line"),
        group_search_attributes: translator.t("Group Search Attributes"),
        group_member_association: translator.t("Group-Member association"),
        special_attributes: translator.t("Special Attributes"),
        quota_field: translator.t("Quota Field"),
        quota_default: translator.t("Quota Default"),
        quota_default_description: translator.t("in bytes"),
        email_field: translator.t("Email Field"),
        user_home_folder_naming_rule: translator.t("User Home Folder Naming Rule"),
        user_home_folder_naming_rule_description: translator.t("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute."),
        internal_username: translator.t("Internal Username"),
        internal_username_description: translator.t("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users."),
        internal_username_attribute: translator.t("Internal Username Attribute:"),
        override_uuid_detection: translator.t("Override UUID detection"),
        override_uuid_detection_description: translator.t("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups."),
        uuid_attribute_for_users: translator.t("UUID Attribute for Users:"),
        uuid_attribute_for_groups: translator.t("UUID Attribute for Groups:"),
        username_ldap_user_mapping: translator.t("Username-LDAP User Mapping"),
        username_ldap_user_mapping_description: translator.t("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage."),
        clear_username_ldap_user_mapping: translator.t("Clear Username-LDAP User Mapping"),
        clear_groupname_ldap_group_mapping: translator.t("Clear Groupname-LDAP Group Mapping"),
    }
}

#[tokio::main]
async fn main() {
    // Configurar el estado de la aplicación
    let app_state = AppState {
        translator: Translator {
            translations: HashMap::new(), // En una implementación real, cargaría traducciones
        },
        app_manager: AppManager {
            enabled_apps: vec!["user_ldap".to_string()], // Ejemplo de apps habilitadas
        },
        theme: Theme {
            name: "Nextcloud".to_string(),
        },
    };

    // Crear el router con las rutas
    let app = Router::new()
        .route("/settings/user/ldap", get(ldap_settings))
        .with_state(app_state);

    // Iniciar el servidor
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}