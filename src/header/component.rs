use super::LoginRegister;
use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header id="header">
            { "UO Scripts" }
            <LoginRegister/>
        </header>
    }
}
