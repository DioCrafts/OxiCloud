use askama::Template;
use std::borrow::Cow;

#[derive(Template)]
#[template(path = "settings.html")]
pub struct WebdavAuthSettingsTemplate<'a> {
    pub webdav_url: &'a str,
    pub request_token: &'a str,
    pub l: &'a dyn Translator,
}

pub trait Translator {
    fn t(&self, text: &str) -> Cow<'_, str>;
}

// Template HTML file (settings.html):
/*
<form id="webdavauth" action="#" method="post">
    <fieldset class="personalblock">
        <h2>{{ l.t("WebDAV Authentication") }}</h2>
        <p>
            <label for="webdav_url">
                {{ l.t("Address: ") }}
                <input type="url" placeholder="https://example.com/webdav" id="webdav_url" name="webdav_url" value="{{ webdav_url }}">
            </label>
            <input type="hidden" name="requesttoken" value="{{ request_token }}" id="requesttoken">
            <input type="submit" value="Save" />
            <br />
            {{ l.t("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.") }}
        </p>
    </fieldset>
</form>
*/

pub fn render_webdav_auth_settings(
    webdav_url: &str,
    request_token: &str,
    translator: &dyn Translator,
) -> Result<String, askama::Error> {
    let template = WebdavAuthSettingsTemplate {
        webdav_url,
        request_token,
        l: translator,
    };
    
    template.render()
}