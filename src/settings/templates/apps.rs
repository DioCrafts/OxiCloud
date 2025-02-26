/**
 * Copyright (c) 2011, Robin Appelman <icewind1991@gmail.com>
 * This file is licensed under the Affero General Public License version 3 or later.
 * See the COPYING-README file.
 */

use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "apps.html")]
struct AppsTemplate {
    apps: Vec<AppInfo>,
    appid: String,
    app_custom_route: String,
    js_link: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppInfo {
    id: String,
    name: String,
    active: bool,
    internal: bool,
    ocs_id: Option<String>,
    internalclass: Option<String>,
    internallabel: Option<String>,
}

#[derive(Deserialize)]
struct AppsQuery {
    appid: Option<String>,
}

#[get("/settings/templates/apps")]
async fn apps_template(
    query: web::Query<AppsQuery>,
    app_service: web::Data<AppService>,
    translator: web::Data<Translator>,
) -> impl Responder {
    let appid = query.appid.clone().unwrap_or_default();
    
    let apps = match app_service.get_apps().await {
        Ok(apps) => apps,
        Err(_) => vec![],
    };
    
    let app_custom_route = app_service.get_app_custom_route(&appid);
    let js_link = app_service.get_js_link("settings/js", "apps.js");
    
    let template = AppsTemplate {
        apps,
        appid,
        app_custom_route,
        js_link,
    };
    
    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

struct AppService {
    // Fields for configuration, database access, etc.
}

impl AppService {
    async fn get_apps(&self) -> Result<Vec<AppInfo>, anyhow::Error> {
        // Implementation to fetch apps from database or service
        Ok(vec![])
    }
    
    fn get_app_custom_route(&self, appid: &str) -> String {
        // Equivalent to OC_Helper::linkToRoute('apps_custom')
        format!("/apps_custom?appid={}", escape_html(appid))
    }
    
    fn get_js_link(&self, path: &str, file: &str) -> String {
        // Equivalent to OC_Helper::linkTo('settings/js', 'apps.js')
        format!("/{}/{}", escape_html(path), escape_html(file))
    }
}

struct Translator {
    // Fields to store translations
}

impl Translator {
    fn translate(&self, text: &str) -> String {
        // Implementation of translation logic
        text.to_string()
    }
}

fn escape_html(text: &str) -> String {
    // HTML escaping implementation
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn sanitize_html(text: &str) -> String {
    // Implementation of OC_Util::sanitizeHTML
    // This would involve more comprehensive HTML sanitization
    text.to_string()
}

// Configuration for the template engine
#[cfg(feature = "embedded_templates")]
mod templates {
    use askama::Template;
    
    #[derive(Template)]
    #[template(source = r#"
    <script type="text/javascript" src="{{ app_custom_route }}"></script>
    <script type="text/javascript" src="{{ js_link }}"></script>

    <ul id="leftcontent" class="applist">
        <li>
            <a class="app-external" target="_blank" href="http://owncloud.org/dev">{{ tr("Add your App") }} …</a>
        </li>

        {% for app in apps %}
        <li {% if app.active %}class="active"{% endif %} data-id="{{ app.id }}"
            {% if app.ocs_id.is_some() %}data-id-ocs="{{ sanitize_html(app.ocs_id.as_ref().unwrap()) }}"{% endif %}
            data-type="{% if app.internal %}internal{% else %}external{% endif %}" data-installed="1">
            <a class="app{% if !app.internal %} externalapp{% endif %}" 
               href="?appid={{ app.id }}">{{ app.name }}</a>
            {% if !app.internal && app.internalclass.is_some() && app.internallabel.is_some() %}
            <small class="{{ sanitize_html(app.internalclass.as_ref().unwrap()) }} list">{{ sanitize_html(app.internallabel.as_ref().unwrap()) }}</small>
            {% endif %}
        </li>
        {% endfor %}

        <li>
            <a class="app-external" target="_blank" href="http://apps.owncloud.com">{{ tr("More Apps") }} …</a>
        </li>
    </ul>
    <div id="rightcontent">
        <div class="appinfo">
        <h3><strong><span class="name">{{ tr("Select an App") }}</span></strong><span
            class="version"></span><small class="externalapp" style="visibility:hidden;"></small></h3>
        <span class="score"></span>
        <p class="description"></p>
        <img src="" class="preview hidden" />
        <p class="appslink hidden"><a href="#" target="_blank">{{ tr("See application page at apps.owncloud.com") }}</a></p>
        <p class="license hidden">{{ tr_raw("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>") }}</p>
        <input class="enable hidden" type="submit" />
        <input class="update hidden" type="submit" value="{{ tr("Update") }}" />
        <div class="warning hidden"></div>
        </div>
    </div>
    "#, ext = "html")]
    struct AppsInnerTemplate {
        app_custom_route: String,
        js_link: String,
        apps: Vec<AppInfo>,
    }
}