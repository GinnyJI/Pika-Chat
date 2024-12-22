use yew::prelude::*;
use yew::{function_component, html, use_effect_with_deps, Callback, Html};
use gloo::storage::{LocalStorage, Storage};
use log::info;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::routes::Route;
use yew_router::prelude::use_navigator;
use crate::services::utils::decode_username;

#[function_component(Home)]
pub fn home() -> Html {
    let username = use_state(|| String::new());
    let navigator = use_navigator().expect("Navigator not available");
    let avatar_url = use_state(|| String::new());

    {
        let username = username.clone();
        let avatar_url = avatar_url.clone();
        use_effect_with_deps(
            move |_| {
                info!("I got rendered, yay!");
                if let Ok(token) = LocalStorage::get::<String>("jwtToken") {
                    if !token.is_empty() {
                        info!("JWT Token: {}", token);
                        if let Some(decoded_username) = decode_username(&token) {
                            info!("JWT Token is valid, username: {}", decoded_username);
                            username.set(decoded_username);

                            // Retrieve avatar_url from local storage
                            if let Ok(stored_avatar_url) = LocalStorage::get::<String>("avatarUrl") {
                                avatar_url.set(stored_avatar_url);
                            } else {
                                avatar_url.set("".to_string());
                            }
                        } else {
                            info!("JWT Token is invalid or expired");
                            LocalStorage::delete("jwtToken");
                            LocalStorage::delete("avatarUrl"); // Remove avatar URL on invalid token
                        }
                    } else {
                        info!("JWT Token is empty");
                    }
                } else {
                    info!("No JWT Token found in LocalStorage");
                }
            },
            (),
        );
    }

    let on_logout = {
        let navigator = navigator.clone();
        let username = username.clone();
        let avatar_url = avatar_url.clone();
        info!("Creating logout callback");
        Callback::from(move |_| {
            gloo::console::log!("Performing logout");
            LocalStorage::delete("jwtToken");
            username.set(String::new()); // Clear the username state
            avatar_url.set(String::new()); // Clear the avatar URL state
            navigator.push(&Route::Home); // Navigate back to Home
        })
    };

    html! {
        <div class="full-height">
            <Header 
                username={(*username).clone()} 
                avatar_url={(*avatar_url).clone()} 
                on_logout={on_logout} 
            />

            // Main Section
            <main class="main">
                <h1 class="heading">{"Welcome to Pika Chat"}</h1>
                <p class="description">
                    {"Connect with your friends, chat in real-time, and enjoy the best social experience online. Join now to explore!"}
                </p>
                <img src="static/pikachu.png" alt="Pikachu image" class="pikachu-img" />
                {
                    if !(*username).is_empty() {
                        html! {
                            <a href="/dashboard" class="button-primary">
                                {"Go to Dashboard"}
                            </a>
                        }
                    } else {
                        html! {} // Explicitly render nothing
                    }
                }
            </main>

            <Footer />
        </div>
    }
}
