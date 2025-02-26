use yew::{prelude::*, html};
use crate::l10n::L10n;
use crate::theme::Theme;

#[derive(Properties, Clone, PartialEq)]
pub struct UserFilterWizardProps {
    pub theme: Theme,
    pub l10n: L10n,
    pub wizard_controls: Html,
}

#[function_component(UserFilterWizard)]
pub fn user_filter_wizard(props: &UserFilterWizardProps) -> Html {
    let toggle_raw_filter = use_state(|| false);
    let user_count = use_state(|| 0);

    let on_toggle_raw_filter = {
        let toggle_raw_filter = toggle_raw_filter.clone();
        Callback::from(move |_| {
            toggle_raw_filter.set(!*toggle_raw_filter);
        })
    };

    html! {
        <fieldset id="ldapWizard2">
            <div>
                <p>
                    { props.l10n.t("Limit the access to %s to users meeting this criteria:", vec![props.theme.get_name()]) }
                </p>
                <p>
                    <label for="ldap_userfilter_objectclass">
                        { props.l10n.t("only those object classes:", vec![]) }
                    </label>

                    <select id="ldap_userfilter_objectclass" multiple="multiple"
                     name="ldap_userfilter_objectclass">
                    </select>
                </p>
                <p>
                    <label for="ldap_userfilter_groups">
                        { props.l10n.t("only from those groups:", vec![]) }
                    </label>

                    <select id="ldap_userfilter_groups" multiple="multiple"
                     name="ldap_userfilter_groups">
                    </select>
                </p>
                <p>
                    <label><a id="toggleRawUserFilter" onclick={on_toggle_raw_filter}>
                        { "↓ " }{ props.l10n.t("Edit raw filter instead", vec![]) }
                    </a></label>
                </p>
                <p id="rawUserFilterContainer" class={if *toggle_raw_filter { "" } else { "invisible" }}>
                    <input type="text" id="ldap_userlist_filter" name="ldap_userlist_filter"
                    class="lwautosave"
                    placeholder={ props.l10n.t("Raw LDAP filter", vec![]) }
                    title={ props.l10n.t("The filter specifies which LDAP users shall have access to the %s instance.", vec![props.theme.get_name()]) }
                    />
                </p>
                <p>
                    <div class="ldapWizardInfo invisible">{ "\u{00A0}" }</div>
                </p>
                <p>
                    <span id="ldap_user_count">{ format!("{} {}", *user_count, props.l10n.t("users found", vec![])) }</span>
                </p>
                { props.wizard_controls.clone() }
            </div>
        </fieldset>
    }
}