// Definir un template de Askama para la página de instalación
use askama::Template;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Template)]
#[template(path = "installation.html")]
pub struct InstallationTemplate {
    has_mysql: bool,
    has_sqlite: bool,
    has_postgresql: bool,
    has_oracle: bool,
    has_mssql: bool,
    errors: Vec<ErrorInfo>,
    vulnerable_to_null_byte: bool,
    secure_rng: bool,
    htaccess_working: bool,
    directory_is_set: bool,
    db_is_set: bool,
    directory: String,
    theme_name: String,
    server_root: String,
    admin_login: String,
    admin_pass: String,
    dbuser: String,
    dbpass: String,
    dbname: String,
    dbtablespace: String,
    dbhost: String,
    dbtype: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorInfo {
    error: String,
    hint: Option<String>,
}

// Implementación para crear un nuevo template
impl InstallationTemplate {
    pub fn new(
        has_mysql: bool,
        has_sqlite: bool,
        has_postgresql: bool,
        has_oracle: bool,
        has_mssql: bool,
        errors: Vec<ErrorInfo>,
        vulnerable_to_null_byte: bool,
        secure_rng: bool,
        htaccess_working: bool,
        directory_is_set: bool,
        db_is_set: bool,
        directory: String,
        theme_name: String,
        form_data: &HashMap<String, String>,
    ) -> Self {
        let server_root = std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        // Obtener valores del formulario o usar valores por defecto
        let admin_login = form_data.get("adminlogin").cloned().unwrap_or_default();
        let admin_pass = form_data.get("adminpass").cloned().unwrap_or_default();
        let dbuser = form_data.get("dbuser").cloned().unwrap_or_default();
        let dbpass = form_data.get("dbpass").cloned().unwrap_or_default();
        let dbname = form_data.get("dbname").cloned().unwrap_or_default();
        let dbtablespace = form_data.get("dbtablespace").cloned().unwrap_or_default();
        let dbhost = form_data.get("dbhost").cloned().unwrap_or_default();
        let dbtype = form_data.get("dbtype").cloned().unwrap_or_else(|| {
            if has_sqlite { "sqlite" } 
            else if has_mysql { "mysql" } 
            else if has_postgresql { "pgsql" } 
            else if has_oracle { "oci" } 
            else if has_mssql { "mssql" } 
            else { "sqlite" }.to_string()
        });

        Self {
            has_mysql,
            has_sqlite,
            has_postgresql,
            has_oracle,
            has_mssql,
            errors,
            vulnerable_to_null_byte,
            secure_rng,
            htaccess_working,
            directory_is_set,
            db_is_set,
            directory,
            theme_name,
            server_root,
            admin_login,
            admin_pass,
            dbuser,
            dbpass,
            dbname,
            dbtablespace,
            dbhost,
            dbtype,
        }
    }
}

// El template sería guardado en templates/installation.html
// Contenido de templates/installation.html:
/*
<input type='hidden' id='hasMySQL' value='{{ has_mysql }}'>
<input type='hidden' id='hasSQLite' value='{{ has_sqlite }}'>
<input type='hidden' id='hasPostgreSQL' value='{{ has_postgresql }}'>
<input type='hidden' id='hasOracle' value='{{ has_oracle }}'>
<input type='hidden' id='hasMSSQL' value='{{ has_mssql }}'>
<form action="index.php" method="post">
<input type="hidden" name="install" value="true" />
    {% if errors.len() > 0 %}
    <ul class="errors">
        {% for err in errors %}
        <li>
            {% if err.hint.is_some() %}
                {{ err.error|safe }}
                <p class='hint'>{{ err.hint.as_ref().unwrap()|safe }}</p>
            {% else %}
                {{ err.error|safe }}
            {% endif %}
        </li>
        {% endfor %}
    </ul>
    {% endif %}
    {% if vulnerable_to_null_byte %}
    <fieldset class="warning">
        <legend><strong>{{ t("Security Warning") }}</strong></legend>
        <p>{{ t("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)") }}<br/>
        {{ t("Please update your PHP installation to use %s securely.", theme_name) }}</p>
    </fieldset>
    {% endif %}
    {% if !secure_rng %}
    <fieldset class="warning">
        <legend><strong>{{ t("Security Warning") }}</strong></legend>
        <p>{{ t("No secure random number generator is available, please enable the PHP OpenSSL extension.") }}<br/>
        {{ t("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.") }}</p>
    </fieldset>
    {% endif %}
    {% if !htaccess_working %}
    <fieldset class="warning">
        <legend><strong>{{ t("Security Warning") }}</strong></legend>
        <p>{{ t("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.") }}<br>
        {{ t_safe("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", link_to_docs("admin-install")) }}</p>
    </fieldset>
    {% endif %}
    <fieldset id="adminaccount">
        <legend>{{ t_safe("Create an <strong>admin account</strong>") }}</legend>
        <p class="infield grouptop">
            <input type="text" name="adminlogin" id="adminlogin" placeholder=""
                value="{{ admin_login }}" autocomplete="off" autofocus required />
            <label for="adminlogin" class="infield">{{ t("Username") }}</label>
            <img class="svg" src="{{ image_path('', 'actions/user.svg') }}" alt="" />
        </p>
        <p class="infield groupbottom">
            <input type="password" name="adminpass" data-typetoggle="#show" id="adminpass" placeholder=""
                value="{{ admin_pass }}" />
            <label for="adminpass" class="infield">{{ t("Password") }}</label>
            <img class="svg" id="adminpass-icon" src="{{ image_path('', 'actions/password.svg')|safe }}" alt="" />
            <input type="checkbox" id="show" name="show" />
            <label for="show"></label>
        </p>
    </fieldset>

    {% if !directory_is_set || !db_is_set || errors.len() > 0 %}
    <fieldset id="advancedHeader">
        <legend><a id="showAdvanced">{{ t("Advanced") }} <img class="svg" src="{{ image_path('', 'actions/caret.svg')|safe }}" /></a></legend>
    </fieldset>
    {% endif %}

    {% if !directory_is_set || errors.len() > 0 %}
    <fieldset id="datadirField">
        <div id="datadirContent">
            <label for="directory">{{ t("Data folder") }}</label>
            <input type="text" name="directory" id="directory"
                placeholder="{{ server_root }}/data"
                value="{{ directory }}" />
        </div>
    </fieldset>
    {% endif %}

    {% if !db_is_set || errors.len() > 0 %}
    <fieldset id='databaseField'>
        {% set has_other_db = has_mysql || has_postgresql || has_oracle || has_mssql %}
        <legend>{{ t("Configure the database") }}</legend>
        <div id="selectDbType">
        {% if has_sqlite %}
        <input type='hidden' id='hasSQLite' value="true" />
        {% if !has_other_db %}
        <p>SQLite {{ t("will be used") }}.</p>
        <input type="hidden" id="dbtype" name="dbtype" value="sqlite" />
        {% else %}
        <input type="radio" name="dbtype" value="sqlite" id="sqlite"
            {% if dbtype == "sqlite" %}checked="checked"{% endif %} />
        <label class="sqlite" for="sqlite">SQLite</label>
        {% endif %}
        {% endif %}

        {% if has_mysql %}
        <input type='hidden' id='hasMySQL' value='true'/>
        {% if !has_sqlite && !has_postgresql && !has_oracle && !has_mssql %}
        <p>MySQL {{ t("will be used") }}.</p>
        <input type="hidden" id="dbtype" name="dbtype" value="mysql" />
        {% else %}
        <input type="radio" name="dbtype" value="mysql" id="mysql"
            {% if dbtype == "mysql" %}checked="checked"{% endif %} />
        <label class="mysql" for="mysql">MySQL</label>
        {% endif %}
        {% endif %}

        {% if has_postgresql %}
        {% if !has_sqlite && !has_mysql && !has_oracle && !has_mssql %}
        <p>PostgreSQL {{ t("will be used") }}.</p>
        <input type="hidden" id="dbtype" name="dbtype" value="pgsql" />
        {% else %}
        <label class="pgsql" for="pgsql">PostgreSQL</label>
        <input type="radio" name="dbtype" value='pgsql' id="pgsql"
            {% if dbtype == "pgsql" %}checked="checked"{% endif %} />
        {% endif %}
        {% endif %}

        {% if has_oracle %}
        {% if !has_sqlite && !has_mysql && !has_postgresql && !has_mssql %}
        <p>Oracle {{ t("will be used") }}.</p>
        <input type="hidden" id="dbtype" name="dbtype" value="oci" />
        {% else %}
        <label class="oci" for="oci">Oracle</label>
        <input type="radio" name="dbtype" value='oci' id="oci"
            {% if dbtype == "oci" %}checked="checked"{% endif %} />
        {% endif %}
        {% endif %}

        {% if has_mssql %}
        <input type='hidden' id='hasMSSQL' value='true'/>
        {% if !has_sqlite && !has_mysql && !has_postgresql && !has_oracle %}
        <p>MS SQL {{ t("will be used") }}.</p>
        <input type="hidden" id="dbtype" name="dbtype" value="mssql" />
        {% else %}
        <label class="mssql" for="mssql">MS SQL</label>
        <input type="radio" name="dbtype" value='mssql' id="mssql" 
            {% if dbtype == "mssql" %}checked="checked"{% endif %} />
        {% endif %}
        {% endif %}
        </div>

        {% if has_other_db %}
        <div id="use_other_db">
            <p class="infield grouptop">
                <label for="dbuser" class="infield">{{ t("Database user") }}</label>
                <input type="text" name="dbuser" id="dbuser" placeholder=""
                    value="{{ dbuser }}" autocomplete="off" />
            </p>
            <p class="infield groupmiddle">
                <input type="password" name="dbpass" id="dbpass" placeholder="" data-typetoggle="#dbpassword" 
                    value="{{ dbpass }}" />
                <label for="dbpass" class="infield">{{ t("Database password") }}</label>
                <input type="checkbox" id="dbpassword" name="dbpassword" />
                <label for="dbpassword"></label>
            </p>
            <p class="infield groupmiddle">
                <label for="dbname" class="infield">{{ t("Database name") }}</label>
                <input type="text" name="dbname" id="dbname" placeholder=""
                    value="{{ dbname }}"
                    autocomplete="off" pattern="[0-9a-zA-Z$_-]+" />
            </p>
            {% if has_oracle %}
            <div id="use_oracle_db">
                <p class="infield groupmiddle">
                    <label for="dbtablespace" class="infield">{{ t("Database tablespace") }}</label>
                    <input type="text" name="dbtablespace" id="dbtablespace" placeholder=""
                        value="{{ dbtablespace }}" autocomplete="off" />
                </p>
            </div>
            {% endif %}
            <p class="infield groupbottom">
                <label for="dbhost" class="infield">{{ t("Database host") }}</label>
                <input type="text" name="dbhost" id="dbhost" placeholder=""
                    value="{{ dbhost }}" />
            </p>
        </div>
        {% endif %}
    </fieldset>
    {% endif %}

    <div class="buttons"><input type="submit" class="primary" value="{{ t("Finish setup") }}" data-finishing="{{ t("Finishing …") }}" /></div>
</form>
*/

// Funciones helper para la traducción y manipulación de imágenes
fn t(text: &str) -> String {
    // Implementación de la función de traducción
    text.to_string()
}

fn t_safe(text: &str, arg: &str) -> String {
    // Implementación de la función de traducción con parámetros
    text.replace("%s", arg)
}

fn image_path(app: &str, file: &str) -> String {
    // Implementación para obtener la ruta de una imagen
    format!("/core/img/{}/{}", app, file)
}

fn link_to_docs(page: &str) -> String {
    // Implementación para generar un enlace a la documentación
    format!("https://docs.example.com/{}", page)
}

// Route handler para mostrar la página de instalación
pub async fn show_installation_page(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let form_data = req.get_form_data()?;
    
    // Obtener configuración del sistema
    let db_config = state.get_db_config();
    let security_config = state.get_security_config();
    
    let template = InstallationTemplate::new(
        db_config.has_mysql,
        db_config.has_sqlite,
        db_config.has_postgresql,
        db_config.has_oracle,
        db_config.has_mssql,
        security_config.errors,
        security_config.vulnerable_to_null_byte,
        security_config.secure_rng,
        security_config.htaccess_working,
        security_config.directory_is_set,
        security_config.db_is_set,
        security_config.directory.clone(),
        state.get_theme_name(),
        &form_data,
    );
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render()?))
}