/**
 * Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 */
use actix_web::{web, HttpResponse};
use serde::Serialize;
use tera::{Context, Tera};
use std::collections::HashMap;

struct L10n {
    // Simplificación del sistema de traducción
    translations: HashMap<String, String>,
}

impl L10n {
    fn t(&self, text: &str) -> String {
        self.translations.get(text)
            .cloned()
            .unwrap_or_else(|| text.to_string())
    }
}

struct Theme {
    name: String,
    title: String,
}

impl Theme {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_title(&self) -> &str {
        &self.title
    }
    
    fn get_short_footer(&self) -> String {
        format!("© {} {} Inc.", chrono::Local::now().year(), self.name)
    }
}

#[derive(Debug, Serialize)]
struct LogEntry {
    level: u8,
    app: String,
    message: String,
    time: i64,
}

struct OCUtil {
    // Utilidades estáticas
}

impl OCUtil {
    fn format_date(timestamp: i64) -> String {
        let dt = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .unwrap_or_else(|| chrono::Local::now().naive_local());
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    
    fn get_version_string() -> String {
        "10.0.0".to_string() // Ejemplo
    }
    
    fn get_channel() -> String {
        "stable".to_string() // Ejemplo
    }
    
    fn get_edition_string() -> String {
        "".to_string() // Ejemplo
    }
}

fn link_to_docs(section: &str) -> String {
    format!("https://docs.example.com/{}", section)
}

pub async fn render_admin_page(
    tera: web::Data<Tera>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let mut context = Context::new();
    let l = L10n { translations: HashMap::new() }; // Simplificado
    let theme = Theme { 
        name: "Nextcloud".to_string(), 
        title: "Nextcloud".to_string() 
    };
    
    let levels = vec!["Debug", "Info", "Warning", "Error", "Fatal"];
    let level_labels = vec![
        l.t("Everything (fatal issues, errors, warnings, info, debug)"),
        l.t("Info, warnings, errors and fatal issues"),
        l.t("Warnings, errors and fatal issues"),
        l.t("Errors and fatal issues"),
        l.t("Fatal issues only"),
    ];
    
    // Configuración del estado actual
    let htaccess_working = true;
    let is_webdav_working = true;
    let has_fileinfo = true;
    let is_locale_working = true;
    let internet_connection_working = true;
    let forms: Vec<String> = vec![];
    let backgroundjobs_mode = "cron".to_string();
    let share_api_enabled = "yes".to_string();
    let allow_links = "yes".to_string();
    let allow_public_upload = "yes".to_string();
    let allow_resharing = "yes".to_string();
    let share_policy = "global".to_string();
    let allow_mail_notification = "yes".to_string();
    let enforce_https_enabled = false;
    let is_connected_via_https = false;
    let loglevel = 2;
    
    // Entradas de registro simuladas
    let entries = vec![
        LogEntry {
            level: 1,
            app: "core".to_string(),
            message: "Log entry example".to_string(),
            time: chrono::Local::now().timestamp(),
        },
    ];
    let entries_remain = true;
    
    // Agregar todo al contexto
    context.insert("levels", &levels);
    context.insert("levelLabels", &level_labels);
    context.insert("htaccessworking", &htaccess_working);
    context.insert("isWebDavWorking", &is_webdav_working);
    context.insert("has_fileinfo", &has_fileinfo);
    context.insert("islocaleworking", &is_locale_working);
    context.insert("internetconnectionworking", &internet_connection_working);
    context.insert("forms", &forms);
    context.insert("backgroundjobs_mode", &backgroundjobs_mode);
    context.insert("shareAPIEnabled", &share_api_enabled);
    context.insert("allowLinks", &allow_links);
    context.insert("allowPublicUpload", &allow_public_upload);
    context.insert("allowResharing", &allow_resharing);
    context.insert("sharePolicy", &share_policy);
    context.insert("allowMailNotification", &allow_mail_notification);
    context.insert("enforceHTTPSEnabled", &enforce_https_enabled);
    context.insert("isConnectedViaHTTPS", &is_connected_via_https);
    context.insert("loglevel", &loglevel);
    context.insert("entries", &entries);
    context.insert("entriesremain", &entries_remain);
    context.insert("theme", &theme);
    context.insert("l", &l);
    
    // Funciones auxiliares para la plantilla
    context.insert("OCUtil", &OCUtil::get_version_string());
    context.insert("link_to_docs", &link_to_docs("admin-install"));
    
    // Renderizar plantilla
    match tera.render("admin.html", &context) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            HttpResponse::InternalServerError().body("Template rendering error")
        }
    }
}

// Template en Tera (admin.html)
// Normalmente esto estaría en un archivo separado, pero lo incluyo aquí como referencia:
/*
{% if not htaccessworking %}
<fieldset class="personalblock">
    <h2>{{ l.t("Security Warning") }}</h2>
    <span class="securitywarning">
        {{ l.t("Your data directory and your files are probably accessible from the internet. The .htaccess file is not working. We strongly suggest that you configure your webserver in a way that the data directory is no longer accessible or you move the data directory outside the webserver document root.") }}
    </span>
</fieldset>
{% endif %}

{% if not isWebDavWorking %}
<fieldset class="personalblock">
    <h2>{{ l.t("Setup Warning") }}</h2>
    <span class="securitywarning">
        {{ l.t("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.") }}
        {{ l.t("Please double check the <a href=\"%s\">installation guides</a>.", link_to_docs) | safe }}
    </span>
</fieldset>
{% endif %}

{% if not has_fileinfo %}
<fieldset class="personalblock">
    <h2>{{ l.t("Module 'fileinfo' missing") }}</h2>
    <span class="connectionwarning">
        {{ l.t("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.") }}
    </span>
</fieldset>
{% endif %}

{% if not islocaleworking %}
<fieldset class="personalblock">
    <h2>{{ l.t("Locale not working") }}</h2>
    <span class="connectionwarning">
        {{ l.t("System locale can't be set to %s. This means that there might be problems with certain characters in file names. We strongly suggest to install the required packages on your system to support %s.", ["en_US.UTF-8/en_US.UTF8", "en_US.UTF-8/en_US.UTF8"]) }}
    </span>
</fieldset>
{% endif %}

{% if not internetconnectionworking %}
<fieldset class="personalblock">
    <h2>{{ l.t("Internet connection not working") }}</h2>
    <span class="connectionwarning">
        {{ l.t("This server has no working internet connection. This means that some of the features like mounting of external storage, notifications about updates or installation of 3rd party apps don´t work. Accessing files from remote and sending of notification emails might also not work. We suggest to enable internet connection for this server if you want to have all features.") }}
    </span>
</fieldset>
{% endif %}

{% for form in forms %}
    {{ form | safe }}
{% endfor %}

<fieldset class="personalblock" id="backgroundjobs">
    <h2>{{ l.t("Cron") }}</h2>
    <p>
        <input type="radio" name="mode" value="ajax"
               id="backgroundjobs_ajax" {% if backgroundjobs_mode == "ajax" %}checked="checked"{% endif %}>
        <label for="backgroundjobs_ajax">AJAX</label><br/>
        <em>{{ l.t("Execute one task with each page loaded") }}</em>
    </p>
    <p>
        <input type="radio" name="mode" value="webcron"
               id="backgroundjobs_webcron" {% if backgroundjobs_mode == "webcron" %}checked="checked"{% endif %}>
        <label for="backgroundjobs_webcron">Webcron</label><br/>
        <em>{{ l.t("cron.php is registered at a webcron service to call cron.php every 15 minutes over http.") }}</em>
    </p>
    <p>
        <input type="radio" name="mode" value="cron"
               id="backgroundjobs_cron" {% if backgroundjobs_mode == "cron" %}checked="checked"{% endif %}>
        <label for="backgroundjobs_cron">Cron</label><br/>
        <em>{{ l.t("Use systems cron service to call the cron.php file every 15 minutes.") }}</em>
    </p>
</fieldset>

<fieldset class="personalblock" id="shareAPI">
    <h2>{{ l.t("Sharing") }}</h2>
    <table class="shareAPI">
        <tr>
            <td id="enable">
                <input type="checkbox" name="shareapi_enabled" id="shareAPIEnabled"
                       value="1" {% if shareAPIEnabled == "yes" %}checked="checked"{% endif %} />
                <label for="shareAPIEnabled">{{ l.t("Enable Share API") }}</label><br/>
                <em>{{ l.t("Allow apps to use the Share API") }}</em>
            </td>
        </tr>
        <tr>
            <td {% if shareAPIEnabled == "no" %}class="hidden"{% endif %}>
                <input type="checkbox" name="shareapi_allow_links" id="allowLinks"
                       value="1" {% if allowLinks == "yes" %}checked="checked"{% endif %} />
                <label for="allowLinks">{{ l.t("Allow links") }}</label><br/>
                <em>{{ l.t("Allow users to share items to the public with links") }}</em>
            </td>
        </tr>
        {% if app_not_enabled("files_encryption") %}
        <tr>
            <td {% if shareAPIEnabled == "no" %}class="hidden"{% endif %}>
                <input type="checkbox" name="shareapi_allow_public_upload" id="allowPublicUpload"
                       value="1" {% if allowPublicUpload == "yes" %}checked="checked"{% endif %} />
                <label for="allowPublicUpload">{{ l.t("Allow public uploads") }}</label><br/>
                <em>{{ l.t("Allow users to enable others to upload into their publicly shared folders") }}</em>
            </td>
        </tr>
        {% endif %}
        <tr>
            <td {% if shareAPIEnabled == "no" %}class="hidden"{% endif %}>
                <input type="checkbox" name="shareapi_allow_resharing" id="allowResharing"
                       value="1" {% if allowResharing == "yes" %}checked="checked"{% endif %} />
                <label for="allowResharing">{{ l.t("Allow resharing") }}</label><br/>
                <em>{{ l.t("Allow users to share items shared with them again") }}</em>
            </td>
        </tr>
        <tr>
            <td {% if shareAPIEnabled == "no" %}class="hidden"{% endif %}>
                <input type="radio" name="shareapi_share_policy" id="sharePolicyGlobal"
                       value="global" {% if sharePolicy == "global" %}checked="checked"{% endif %} />
                <label for="sharePolicyGlobal">{{ l.t("Allow users to share with anyone") }}</label><br/>
                <input type="radio" name="shareapi_share_policy" id="sharePolicyGroupsOnly"
                       value="groups_only" {% if sharePolicy == "groups_only" %}checked="checked"{% endif %} />
                <label for="sharePolicyGroupsOnly">{{ l.t("Allow users to only share with users in their groups") }}</label><br/>
            </td>
        </tr>
        <tr>
            <td {% if shareAPIEnabled == "no" %}class="hidden"{% endif %}>
                <input type="checkbox" name="shareapi_allow_mail_notification" id="allowMailNotification"
                       value="1" {% if allowMailNotification == "yes" %}checked="checked"{% endif %} />
                <label for="allowMailNotification">{{ l.t("Allow mail notification") }}</label><br/>
                <em>{{ l.t("Allow user to send mail notification for shared files") }}</em>
            </td>
        </tr>
    </table>
</fieldset>

<fieldset class="personalblock" id="security">
    <h2>{{ l.t("Security") }}</h2>
    <table>
        <tr>
            <td id="enable">
                <input type="checkbox" name="forcessl"  id="forcessl"
                    {% if enforceHTTPSEnabled %}
                        checked="checked" value="false"
                    {% else %}
                        value="true"
                    {% endif %}
                    {% if not isConnectedViaHTTPS %}disabled{% endif %} />
                <label for="forcessl">{{ l.t("Enforce HTTPS") }}</label><br/>
                <em>{{ l.t("Forces the clients to connect to %s via an encrypted connection.", theme.get_name()) }}</em>
                {% if not isConnectedViaHTTPS %}
                <br/><em>{{ l.t("Please connect to your %s via HTTPS to enable or disable the SSL enforcement.", theme.get_name()) }}</em>
                {% endif %}
            </td>
        </tr>
    </table>
</fieldset>

<fieldset class="personalblock">
    <h2>{{ l.t("Log") }}</h2>
    {{ l.t("Log level") }} <select name='loglevel' id='loglevel'>
    {% for i in range(0, 5) %}
        <option value='{{ i }}' {% if i == loglevel %}selected="selected"{% endif %}>{{ levelLabels[i] }}</option>
    {% endfor %}
    </select>
    <table id="log" class="grid">
        {% for entry in entries %}
        <tr>
            <td>
                {{ levels[entry.level] }}
            </td>
            <td>
                {{ entry.app }}
            </td>
            <td>
                {{ entry.message }}
            </td>
            <td class="date">
                {% if entry.time is number %}
                    {{ OCUtil::format_date(entry.time) }}
                {% else %}
                    {{ entry.time }}
                {% endif %}
            </td>
        </tr>
        {% endfor %}
    </table>
    {% if entriesremain %}
    <input id="moreLog" type="button" value="{{ l.t('More') }}...">
    <input id="lessLog" type="button" value="{{ l.t('Less') }}...">
    {% endif %}
</fieldset>

<fieldset class="personalblock">
    <h2>{{ l.t("Version") }}</h2>
    <strong>{{ theme.get_title() }}</strong> {{ OCUtil::get_version_string() }} ({{ OCUtil::get_channel() }})
    {% if OCUtil::get_edition_string() == "" %}
    <p>
        {{ l.t("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.") | safe }}
    </p>
    {% endif %}
</fieldset>

<fieldset class="personalblock credits-footer">
<p>
    {{ theme.get_short_footer() | safe }}
</p>
</fieldset>
*/