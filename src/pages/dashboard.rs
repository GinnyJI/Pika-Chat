use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use crate::routes::Route;
use crate::services::auth::logout;
use wasm_bindgen_futures::spawn_local;

pub enum Msg {
    LogoutClicked,
    LogoutSuccess,
    LogoutFailure(String),
}

pub struct Dashboard {
    token: Option<String>,
}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Check if token is present in LocalStorage
        let token = LocalStorage::get::<String>("jwtToken").ok();
        Self { token }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let navigator = ctx.link().navigator().expect("No navigator available");

        match msg {
            Msg::LogoutClicked => {
                if let Some(token) = self.token.clone() {
                    let link = ctx.link().clone(); // Clone the link outside the async block
                    spawn_local(async move {
                        let result: Result<(), String> = logout(&token).await;
                        match result {
                            Ok(_) => {
                                link.send_message(Msg::LogoutSuccess); // Use link, not ctx.link()
                            }
                            Err(e) => {
                                link.send_message(Msg::LogoutFailure(e));
                            }
                        }
                    });
                } else {
                    // If no token is found, just navigate home
                    navigator.push(&Route::Home);
                }
                false
            }
            Msg::LogoutSuccess => {
                // Remove the token from local storage
                LocalStorage::delete("jwtToken");
                navigator.push(&Route::Home);
                true
            }
            Msg::LogoutFailure(_error) => {
                // In case of failure, you might display an error or still navigate away
                // For simplicity, we'll just remove the token and navigate home
                LocalStorage::delete("jwtToken");
                navigator.push(&Route::Home);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_logout = ctx.link().callback(|_| Msg::LogoutClicked);

        html! {
            <div style="min-height: 100vh; display: flex; flex-direction: column; background-color: #f9fafb;">
                // Header Section
                <header style="background-color: #facc15; padding: 1rem; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); position: static; width: 100%;">
                    <nav style="max-width: 1200px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;">
                        <div style="font-size: 1.5rem; font-weight: bold; color: #1f2937;">
                            <a href="/">{"Pika Chat"}</a>
                        </div>
                        <div style="display: flex; gap: 1rem;">
                            // Instead of Register and Login, we show Logout
                            <a href="#" onclick={onclick_logout} style="color: #1f2937; text-decoration: none; font-weight: 500; transition: color 0.2s;">
                                {"Logout"}
                            </a>
                        </div>
                    </nav>
                </header>

                // Main Section
                <main style="flex: 1; padding: 2rem; display: flex; flex-direction: column; align-items: center; text-align: center;">
                    <h1 style="font-size: 2.5rem; font-weight: bold; color: #1f2937; margin-bottom: 1.5rem;">
                        {"Dashboard"}
                    </h1>
                    <p style="font-size: 1.125rem; color: #4b5563; margin-bottom: 2rem; max-width: 600px;">
                        {"Welcome to the Dashboard page! This is a demo."}
                    </p>
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