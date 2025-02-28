use std::collections::HashMap;
use askama::Template;

#[derive(Template)]
#[template(path = "wizard-loginfilter.html")]
pub struct LoginFilterTemplate<'a> {
    pub l: &'a dyn Translator,
    pub wizard_controls: &'a str,
}

pub trait Translator {
    fn t(&self, text: &str) -> String;
}

impl<'a> LoginFilterTemplate<'a> {
    pub fn new(translator: &'a dyn Translator, wizard_controls: &'a str) -> Self {
        Self {
            l: translator,
            wizard_controls,
        }
    }
}

// HTML template content in wizard-loginfilter.html:
//
// <fieldset id="ldapWizard3">
//     <div>
//         <p>
//             {{ l.t("What attribute shall be used as login name:") }}
//         </p>
//         <p>
//             <label for="ldap_loginfilter_username">
//                 {{ l.t("LDAP Username:") }}
//             </label>
// 
//             <input type="checkbox" id="ldap_loginfilter_username"
//              name="ldap_loginfilter_username" value="1" class="lwautosave" />
//         </p>
//         <p>
//             <label for="ldap_loginfilter_email">
//                 {{ l.t("LDAP Email Address:") }}
//             </label>
// 
//             <input type="checkbox" id="ldap_loginfilter_email"
//              name="ldap_loginfilter_email" value="1" class="lwautosave" />
//         </p>
//         <p>
//             <label for="ldap_loginfilter_attributes">
//                 {{ l.t("Other Attributes:") }}
//             </label>
// 
//             <select id="ldap_loginfilter_attributes" multiple="multiple"
//              name="ldap_loginfilter_attributes">
//             </select>
//         </p>
//         <p>
//             <div class="ldapWizardInfo invisible">&nbsp;</div>
//         </p>
// 
//         {{ wizard_controls|safe }}
//     </div>
// </fieldset>