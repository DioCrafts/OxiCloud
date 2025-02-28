use std::collections::HashMap;
use askama::Template;

#[derive(Template)]
#[template(path = "part.wizard-server.html")]
pub struct WizardServerTemplate<'a> {
    server_configuration_prefixes: Vec<String>,
    server_configuration_hosts: HashMap<String, String>,
    wizard_controls: String,
    l: &'a dyn Translator,
}

pub trait Translator {
    fn t(&self, text: &str) -> String;
}

impl<'a> WizardServerTemplate<'a> {
    pub fn new(
        server_configuration_prefixes: Vec<String>,
        server_configuration_hosts: HashMap<String, String>,
        wizard_controls: String,
        l: &'a dyn Translator,
    ) -> Self {
        Self {
            server_configuration_prefixes,
            server_configuration_hosts,
            wizard_controls,
            l,
        }
    }
}

Y el archivo de plantilla correspondiente en `templates/part.wizard-server.html`:

```html
<fieldset id="ldapWizard1">
    <p>
    <select id="ldap_serverconfig_chooser" name="ldap_serverconfig_chooser">
    {% if server_configuration_prefixes.is_empty() %}
        <option value="" selected>1. Server</option>
    {% else %}
        {% for prefix in &server_configuration_prefixes %}
            <option value="{{ prefix }}"{% if loop.first %} selected{% endif %}>{{ loop.index }}. Server: {{ server_configuration_hosts.get(prefix).unwrap_or(&String::new()) }}</option>
        {% endfor %}
    {% endif %}
    <option value="NEW">{{ l.t("Add Server Configuration") }}</option>
    </select>
    <button id="ldap_action_delete_configuration"
        name="ldap_action_delete_configuration">Delete Configuration</button>
    </p>

    <div class="hostPortCombinator">
        <div class="tablerow">
            <div class="tablecell">
                <div class="table">
                    <input type="text" class="host tablecell lwautosave" id="ldap_host"
                        name="ldap_host"
                        placeholder="{{ l.t("Host") }}"
                        title="{{ l.t("You can omit the protocol, except you require SSL. Then start with ldaps://") }}"
                        />
                    <span>
                        <input type="number" id="ldap_port" name="ldap_port"
                            class="invisible lwautosave"
                            placeholder="{{ l.t("Port") }}" />
                    </span>
                </div>
            </div>
        </div>
        <div class="tablerow">
            <input type="text" id="ldap_dn" name="ldap_dn"
            class="tablecell lwautosave"
            placeholder="{{ l.t("User DN") }}"
            title="{{ l.t("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.") }}"
            />
        </div>

        <div class="tablerow">
            <input type="password" id="ldap_agent_password"
            class="tablecell lwautosave" name="ldap_agent_password"
            placeholder="{{ l.t("Password") }}"
            title="{{ l.t("For anonymous access, leave DN and Password empty.") }}"
            />
        </div>

        <div class="tablerow">
            <textarea id="ldap_base" name="ldap_base"
                class="tablecell invisible lwautosave"
                placeholder="{{ l.t("One Base DN per line") }}"
                title="{{ l.t("You can specify Base DN for users and groups in the Advanced tab") }}">
            </textarea>
        </div>

        <div class="tablerow">
            <div class="tablecell ldapWizardInfo invisible">&nbsp;
            </div>
        </div>
    </div>
    {{ wizard_controls|safe }}
</fieldset>