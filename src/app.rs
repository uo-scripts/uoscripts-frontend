use yew::prelude::*;

use crate::header::Header;

// #[function_component]
// pub fn H1() -> Html {
//     let count = use_state(|| 0);
//     let onclick = {
//         let count = count.clone();
//         move |_| {
//             let value = *count + 1;
//             count.set(value);
//         }
//     };

//     html! {
//         <h1 {onclick}>{ "Hello World!" }{ *count }</h1>
//     }
// }

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Header/>
        </main>
    }
}
