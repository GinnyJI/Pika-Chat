use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use crate::components::form_input::FormInput;
use crate::services::auth::{Credentials, login};
use crate::routes::Route;

pub struct Login {
    credentials: Credentials,
    error: Option<String>,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit,
    LoginSuccess(String),
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
            },
            error: None,
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

                // Spawn an async task to handle login.
                wasm_bindgen_futures::spawn_local(async move {
                    match login(&credentials).await {
                        Ok(response) => link.send_message(Msg::LoginSuccess(response.token)),
                        Err(error) => link.send_message(Msg::LoginFailure(error)),
                    }
                });
                false
            }
            Msg::LoginSuccess(token) => {
                // Save the token to local storage using the same key as home.rs
                LocalStorage::set("jwtToken", token).expect("Failed to save token");

                // Redirect to the "dashboard" page
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
                // Header Section
                <header class="header">
                    <nav class="nav">
                        <a href="/" class="nav-logo">{"Pika Chat"}</a>
                        <div class="nav-links">
                            <a href="/register" class="nav-link">{"Register"}</a>
                            <a href="/login" class="nav-link">{"Login"}</a>
                        </div>
                    </nav>
                </header>

                // Main Section
                <main class="main">
                    <div class="login-card">
                        <h1 class="login-heading">{"Login"}</h1>
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

                // Footer Section
                <footer class="footer">
                    <p class="footer-text">
                        {"© 2024 Pika Chat. All rights reserved."}
                    </p>
                </footer>
            </div>
        }
    }
}
