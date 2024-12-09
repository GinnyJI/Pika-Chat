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
            <div style="min-height: 100vh; display: flex; flex-direction: column; background-color: #f9fafb;">
                // Header Section
                <header style="background-color: #facc15; padding: 1rem; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); position: static; width: 100%;">
                    <nav style="max-width: 1200px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;">
                        <div style="font-size: 1.5rem; font-weight: bold; color: #1f2937;">
                            <a href="/">{"Pika Chat"}</a>
                        </div>
                        <div style="display: flex; gap: 1rem;">
                            <a href="/register" style="color: #1f2937; text-decoration: none; font-weight: 500; transition: color 0.2s; hover: color: #4b5563;">
                                {"Register"}
                            </a>
                            <a href="/login" style="color: #1f2937; text-decoration: none; font-weight: 500; transition: color 0.2s; hover: color: #4b5563;">
                                {"Login"}
                            </a>
                        </div>
                    </nav>
                </header>

                // Main Section
                <main style="flex: 1; padding: 2rem; display: flex; flex-direction: column; align-items: center; justify-content: center;">
                    <div style="max-width: 400px; width: 100%; background-color: #ffffff; border-radius: 0.5rem; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); padding: 2rem;">
                        <h1 style="font-size: 1.5rem; font-weight: bold; color: #1f2937; text-align: center;">
                            {"Register"}
                        </h1>
                        <form
                            style="margin-top: 1.5rem; display: flex; flex-direction: column; gap: 1rem;"
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
                                <p style="color: #dc2626; font-size: 0.875rem;">{ error.clone() }</p>
                            }
                            if self.success {
                                <p style="color: #16a34a; font-size: 0.875rem;">{ "Registration successful! Please log in." }</p>
                            }
                            <button
                                type="submit"
                                style="padding: 0.75rem; background-color: #4f46e5; color: #ffffff; border-radius: 0.375rem; text-align: center; font-weight: bold; transition: background-color 0.2s; hover: background-color: #4338ca;"
                            >
                                { "Register" }
                            </button>
                        </form>
                    </div>
                </main>

                // Footer Section
                <footer style="background-color: #1f2937; color: #e5e7eb; text-align: center; padding: 1rem; width: 100%;">
                    <p style="font-size: 0.875rem;">
                        {"© 2024 Pika Chat. All rights reserved."}
                    </p>
                </footer>
            </div>
        }
    }
}