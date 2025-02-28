use std::collections::HashMap;
use askama::Template;

#[derive(Template)]
#[template(path = "settings-personal.html")]
pub struct SettingsPersonalTemplate {
    private_key_set: bool,
    initialized: bool,
    recovery_enabled: bool,
    recovery_enabled_for_user: i32,
    l: Localization,
}

pub struct Localization {
    translations: HashMap<String, String>,
}

impl Localization {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        // Inicializar traducciones
        translations.insert("Encryption".to_string(), "Encryption".to_string());
        translations.insert("Your private key password no longer match your log-in password:".to_string(), 
                          "Your private key password no longer match your log-in password:".to_string());
        translations.insert("Set your old private key password to your current log-in password.".to_string(),
                          "Set your old private key password to your current log-in password.".to_string());
        translations.insert(" If you don't remember your old password you can ask your administrator to recover your files.".to_string(),
                          " If you don't remember your old password you can ask your administrator to recover your files.".to_string());
        translations.insert("Old log-in password".to_string(), "Old log-in password".to_string());
        translations.insert("Current log-in password".to_string(), "Current log-in password".to_string());
        translations.insert("Update Private Key Password".to_string(), "Update Private Key Password".to_string());
        translations.insert("Enable password recovery:".to_string(), "Enable password recovery:".to_string());
        translations.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss".to_string(),
                          "Enabling this option will allow you to reobtain access to your encrypted files in case of password loss".to_string());
        translations.insert("Enabled".to_string(), "Enabled".to_string());
        translations.insert("Disabled".to_string(), "Disabled".to_string());
        translations.insert("File recovery settings updated".to_string(), "File recovery settings updated".to_string());
        translations.insert("Could not update file recovery".to_string(), "Could not update file recovery".to_string());
        
        Self { translations }
    }
    
    pub fn t(&self, key: &str) -> &str {
        self.translations.get(key).map_or(key, |s| s.as_str())
    }
}

// Contenido HTML de la plantilla (en un archivo separado: templates/settings-personal.html)
/*
<form id="encryption">
    <fieldset class="personalblock">
        <h2>{{ l.t("Encryption") }}</h2>

        {% if !private_key_set && initialized %}
            <p>
                <a name="changePKPasswd" />
                <label for="changePrivateKeyPasswd">
                    {{ l.t("Your private key password no longer match your log-in password:") }}
                </label>
                <br />
                <em>{{ l.t("Set your old private key password to your current log-in password.") }}
                {% if recovery_enabled_for_user == 1 %}
                    {{ l.t(" If you don't remember your old password you can ask your administrator to recover your files.") }}
                {% endif %}
                </em>
                <br />
                <input
                    type="password"
                    name="changePrivateKeyPassword"
                    id="oldPrivateKeyPassword" />
                <label for="oldPrivateKeyPassword">{{ l.t("Old log-in password") }}</label>
                <br />
                <input
                    type="password"
                    name="changePrivateKeyPassword"
                    id="newPrivateKeyPassword" />
                <label for="newRecoveryPassword">{{ l.t("Current log-in password") }}</label>
                <br />
                <button
                    type="button"
                    name="submitChangePrivateKeyPassword"
                    disabled>{{ l.t("Update Private Key Password") }}
                </button>
                <span class="msg"></span>
            </p>
        {% endif %}

        {% if recovery_enabled && private_key_set %}
            <br />
            <p>
                <label for="userEnableRecovery">{{ l.t("Enable password recovery:") }}</label>
                <br />
                <em>{{ l.t("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss") }}</em>
                <br />
                <input
                type='radio'
                name='userEnableRecovery'
                value='1'
                {% if recovery_enabled_for_user == 1 %}checked="checked"{% endif %} />
                {{ l.t("Enabled") }}
                <br />

                <input
                type='radio'
                name='userEnableRecovery'
                value='0'
                {% if recovery_enabled_for_user == 0 %}checked="checked"{% endif %} />
                {{ l.t("Disabled") }}
                <div id="recoveryEnabledSuccess">{{ l.t("File recovery settings updated") }}</div>
                <div id="recoveryEnabledError">{{ l.t("Could not update file recovery") }}</div>
            </p>
        {% endif %}

    </fieldset>
</form>
*/

pub fn render_settings_personal(
    private_key_set: bool,
    initialized: bool,
    recovery_enabled: bool,
    recovery_enabled_for_user: i32,
) -> Result<String, askama::Error> {
    let template = SettingsPersonalTemplate {
        private_key_set,
        initialized,
        recovery_enabled,
        recovery_enabled_for_user,
        l: Localization::new(),
    };
    
    template.render()
}