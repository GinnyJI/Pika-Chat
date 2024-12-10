use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::form_input::FormInput;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::services::auth::{Credentials, register};
use crate::routes::Route;

pub struct Register {
    credentials: Credentials,
    error: Option<String>,
    success: bool,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    SelectAvatar(String),
    Submit,
    RegisterSuccess,
    RegisterFailure(String),
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            credentials: Credentials {
                username: String::new(),
                password: String::new(),
                avatar_url: None, // Initialize avatar_url as None
            },
            error: None,
            success: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(username) => {
                self.credentials.username = username;
                true
            }
            Msg::UpdatePassword(password) => {
                self.credentials.password = password;
                true
            }
            Msg::SelectAvatar(avatar) => {
                self.credentials.avatar_url = Some(avatar); // Directly update avatar_url in credentials
                true
            }
            Msg::Submit => {
                let credentials = self.credentials.clone();
                let link = ctx.link().clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match register(&credentials).await {
                        Ok(_) => {
                            link.send_message(Msg::RegisterSuccess);
                        }
                        Err(error) => {
                            link.send_message(Msg::RegisterFailure(error));
                        }
                    }
                });
                false
            }
            Msg::RegisterSuccess => {
                self.success = true;
                self.error = None;
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Login);
                true
            }
            Msg::RegisterFailure(error) => {
                self.error = Some(error);
                self.success = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let avatar_options = vec![
            "static/avatar1.png".to_string(),
            "static/avatar2.png".to_string(),
            "static/avatar3.png".to_string(),
            "static/avatar4.png".to_string(),
            "static/avatar5.png".to_string(),
        ];

        html! {
            <div class="full-height">
                <Header 
                    username={None::<String>} // No username to display
                    avatar_url={None::<String>}
                />
                <main class="main">
                    <div class="register-card">
                        <h1 class="register-heading">{ "Register" }</h1>
                        <form
                            class="register-form"
                            onsubmit={ctx.link().callback(|e: SubmitEvent| {
                                e.prevent_default();
                                Msg::Submit
                            })}
                        >
                            <FormInput
                                label="Username"
                                placeholder="Enter your username"
                                input_type="text"
                                value={self.credentials.username.clone()}
                                oninput={ctx.link().callback(Msg::UpdateUsername)}
                            />
                            <FormInput
                                label="Password"
                                placeholder="Enter your password"
                                input_type="password"
                                value={self.credentials.password.clone()}
                                oninput={ctx.link().callback(Msg::UpdatePassword)}
                            />

                            <div class="avatar-selection">
                                <label>{ "Select an Avatar" }</label>
                                <div class="avatar-options">
                                    {
                                        avatar_options.iter().map(|avatar| {
                                            let avatar_clone = avatar.clone();
                                            html! {
                                                <div
                                                    class={classes!(
                                                        "avatar-option",
                                                        if self.credentials.avatar_url.as_deref() == Some(avatar.as_str()) { "selected" } else { "" }
                                                    )}
                                                    onclick={ctx.link().callback(move |_| Msg::SelectAvatar(avatar_clone.clone()))}
                                                >
                                                    <img src={avatar.clone()} alt={avatar.clone()} />
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>

                            if let Some(error) = &self.error {
                                <p class="error-message">{ error.clone() }</p>
                            }
                            if self.success {
                                <p class="success-message">{ "Registration successful! Please log in." }</p>
                            }
                            <button type="submit" class="button-primary">
                                { "Register" }
                            </button>
                        </form>
                    </div>
                </main>
                <Footer />
            </div>
        }
    }
}
