use yew::prelude::*;

pub struct LoginRegister {
    token: Option<String>,
    show_login: bool,
    show_register: bool,
}

pub enum Msg {
    ShowLogin,
    ShowRegister,
    Logout,
    SetToken(String),
}

impl Component for LoginRegister {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            token: None,
            show_login: false,
            show_register: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Logout => {
                self.token = None;
                true
            }
            Msg::ShowLogin => {
                self.show_register = false;
                self.show_login = true;
                true
            }
            Msg::ShowRegister => {
                self.show_login = false;
                self.show_register = true;
                true
            }
            Msg::SetToken(token) => {
                self.token = Some(token);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="login-register">
                {
                    match self.token {
                        Some(_) => html! {
                            <button onclick={ctx.link().callback(|_| Msg::Logout)}>{"Logout"}</button>
                        },
                        None => html! {
                            <>
                                <button onclick={ctx.link().callback(|_| Msg::ShowLogin)}>{"Login"}</button>
                                <button onclick={ctx.link().callback(|_| Msg::ShowRegister)}>{"Register"}</button>
                            </>
                        }
                    }
                }
            </div>
        }
    }
}
