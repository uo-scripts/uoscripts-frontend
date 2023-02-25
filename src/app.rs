use yew::prelude::*;

use crate::header::Header;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Header/>
        </main>
    }
}
