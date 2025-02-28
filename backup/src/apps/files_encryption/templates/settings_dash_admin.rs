use askama::Template;

struct L {
    translations: std::collections::HashMap<String, String>,
}

impl L {
    fn t(&self, key: &str) -> &str {
        self.translations.get(key).map_or(key, |s| s.as_str())
    }
}

#[derive(Template)]
#[template(path = "settings-admin.html")]
struct SettingsAdminTemplate<'a> {
    l: &'a L,
    recovery_enabled: &'a str,
}

// HTML template (settings-admin.html):
/*
<form id="encryption">
    <fieldset class="personalblock">
        <h2>{{ l.t("Encryption") }}</h2>

        <p>
            {{ l.t("Enable recovery key (allow to recover users files in case of password loss):") }}
            <br/>
            <br/>
            <input type="password" name="encryptionRecoveryPassword" id="encryptionRecoveryPassword"/>
            <label for="recoveryPassword">{{ l.t("Recovery key password") }}</label>
            <br/>
            <input type="password" name="encryptionRecoveryPassword" id="repeatEncryptionRecoveryPassword"/>
            <label for="repeatEncryptionRecoveryPassword">{{ l.t("Repeat Recovery key password") }}</label>
            <br/>
            <input
                type='radio'
                name='adminEnableRecovery'
                value='1'
                {% if recovery_enabled == "1" %}checked="checked"{% else %}disabled{% endif %} />
            {{ l.t("Enabled") }}
            <br/>

            <input
                type='radio'
                name='adminEnableRecovery'
                value='0'
                {% if recovery_enabled == "0" %}checked="checked"{% else %}disabled{% endif %} />
            {{ l.t("Disabled") }}
        </p>
        <br/><br/>

        <p name="changeRecoveryPasswordBlock" {% if recovery_enabled == "0" %}class="hidden"{% endif %}>
            <strong>{{ l.t("Change recovery key password:") }}</strong>
            <br/><br/>
            <input
                type="password"
                name="changeRecoveryPassword"
                id="oldEncryptionRecoveryPassword" />
            <label for="oldEncryptionRecoveryPassword">{{ l.t("Old Recovery key password") }}</label>
            <br/>
            <br/>
            <input
                type="password"
                name="changeRecoveryPassword"
                id="newEncryptionRecoveryPassword" />
            <label for="newEncryptionRecoveryPassword">{{ l.t("New Recovery key password") }}</label>
            <br/>
            <input
                type="password"
                name="changeRecoveryPassword"
                id="repeatedNewEncryptionRecoveryPassword" />
            <label for="repeatEncryptionRecoveryPassword">{{ l.t("Repeat New Recovery key password") }}</label>
            <br/>
            <button
                type="button"
                name="submitChangeRecoveryKey"
                disabled>{{ l.t("Change Password") }}
            </button>
            <span class="msg"></span>
        </p>
    </fieldset>
</form>
*/

fn render_settings_admin(l: &L, recovery_enabled: &str) -> Result<String, askama::Error> {
    let template = SettingsAdminTemplate {
        l,
        recovery_enabled,
    };
    template.render()
}