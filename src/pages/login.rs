use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use crate::components::form_input::FormInput;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::services::auth::{Credentials, login};
use crate::routes::Route;

pub struct Login {
    credentials: Credentials,
    error: Option<String>,
    avatar_url: Option<String>,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit,
    LoginSuccess { token: String, username: String, avatar_url: Option<String> },
    LoginFailure(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            credentials: Credentials {
                username: String::new(),
                password: String::new(),
                avatar_url: None, // Only used by register
            },
            error: None,
            avatar_url: None, // Initialize avatar_url as None
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
            Msg::Submit => {
                let credentials = self.credentials.clone();
                let link = ctx.link().clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match login(&credentials).await {
                        Ok(response) => link.send_message(Msg::LoginSuccess {
                            token: response.token,
                            username: credentials.username.clone(),
                            avatar_url: response.avatar_url,
                        }),
                        Err(error) => link.send_message(Msg::LoginFailure(error)),
                    }
                });                
                false
            }
            Msg::LoginSuccess { token, username, avatar_url } => {
                // Save the JWT token to local storage
                LocalStorage::set("jwtToken", token).expect("Failed to save token");
                
                // Save avatar_url to local storage (if available)
                if let Some(url) = avatar_url.clone() {
                    LocalStorage::set("avatarUrl", url).expect("Failed to save avatar URL");
                } else {
                    // Clear any previously stored avatar URL if it's not provided
                    LocalStorage::delete("avatarUrl");
                }
            
                // Update avatar_url and username in component state
                self.credentials.username = username;
                self.avatar_url = avatar_url;
            
                // Redirect to the dashboard
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Dashboard);
                false
            }            
            Msg::LoginFailure(error) => {
                self.error = Some(error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="full-height">
                <Header />
                <main class="main">
                    <div class="login-card">
                        <h1 class="login-heading">{ "Login" }</h1>
                        <form
                            class="login-form"
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
                            if let Some(error) = &self.error {
                                <p class="error-message">{ error.clone() }</p>
                            }
                            <button type="submit" class="button-primary">
                                { "Login" }
                            </button>
                        </form>
                        <p class="register-link">
                            <a href="/register" class="link">
                                { "Don't have an account? Register" }
                            </a>
                        </p>
                    </div>
                </main>
                <Footer />
            </div>
        }
    }
}
