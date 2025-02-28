use yew::{html, Component, Context, Html};
use i18n::translate as t;

pub struct Upgrade {}

impl Component for Upgrade {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="upgrade">
                { t("Upgrading filesystem cache...") }
                <div id="progressbar" />
            </div>
        }
    }
}