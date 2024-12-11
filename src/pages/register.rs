use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::form_input::FormInput;
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
            Msg::Submit => {
                let credentials = self.credentials.clone();
                let link = ctx.link().clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match register(&credentials).await {
                        Ok(_) => link.send_message(Msg::RegisterSuccess),
                        Err(error) => link.send_message(Msg::RegisterFailure(error)),
                    }
                });
                false
            }
            Msg::RegisterSuccess => {
                self.success = true;
                self.error = None;
                // Redirect to the "login" page
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
                    <div class="register-card">
                        <h1 class="register-heading">{"Register"}</h1>
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
