use std::borrow::Cow;
use std::path::Path;

use askama::Template;
use maud::{html, Markup, PreEscaped};

#[derive(Template)]
#[template(path = "part.wizardcontrols.html")]
pub struct WizardControlsTemplate<'a> {
    pub l: &'a dyn Translator,
    pub theme: &'a dyn Theme,
}

pub trait Translator {
    fn t(&self, text: &str) -> Cow<'_, str>;
}

pub trait Theme {
    fn get_doc_base_url(&self) -> Cow<'_, str>;
}

pub trait Util {
    fn image_path(app: &str, file: &str) -> String;
}

impl<'a> WizardControlsTemplate<'a> {
    pub fn render(&self) -> Markup {
        let back_text = self.l.t("Back");
        let continue_text = self.l.t("Continue");
        let help_text = self.l.t("Help");
        let doc_url = format!("{}/server/5.0/admin_manual/auth_ldap.html", self.theme.get_doc_base_url());
        let info_image = OcpUtil::image_path("", "actions/info.png");

        html! {
            div class="ldapWizardControls" {
                button class="ldap_action_back invisible" name="ldap_action_back" type="button" {
                    (back_text)
                }
                button class="ldap_action_continue" name="ldap_action_continue" type="button" {
                    (continue_text)
                }
                a href=(doc_url) target="_blank" {
                    img src=(info_image) style="height:1.75ex" {}
                    (help_text)
                }
            }
        }
    }
}

// Implementación simulada de OCP\Util
struct OcpUtil;

impl OcpUtil {
    pub fn image_path(app: &str, file: &str) -> String {
        // Esta sería la implementación real que busca la ruta de la imagen
        if app.is_empty() {
            format!("/core/img/{}", file)
        } else {
            format!("/apps/{}/img/{}", app, file)
        }
    }
}