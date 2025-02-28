use rocket::response::content::Html;
use rocket::get;
use rocket_dyn_templates::{Template, context};

use crate::helpers::OcHelper;
use crate::encryption::Crypt;

#[get("/invalid_private_key")]
pub fn invalid_private_key(message: &str, error_code: i32, l: &crate::i18n::Translator) -> Template {
    let location = OcHelper::link_to_route("settings_personal").unwrap_or_default() + "#changePKPasswd";
    
    let context = context! {
        message: message,
        error_code: error_code,
        location: location,
        is_private_key_error: error_code == Crypt::ENCRYPTION_PRIVATE_KEY_NOT_VALID_ERROR,
        t_personal_settings: l.t("personal settings"),
        t_go_directly: l.t("Go directly to your ")
    };
    
    Template::render("invalid_private_key", context)
}

// Template file: templates/invalid_private_key.html.tera
/*
<ul>
    <li class='error'>
        {{ message }}
        <br/>
        {% if is_private_key_error %}
            {{ t_go_directly | safe }} <a href="{{ location }}">{{ t_personal_settings }}</a>.
        {% endif %}
        <br/>
    </li>
</ul>
*/