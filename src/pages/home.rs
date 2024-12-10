use yew::prelude::*;
use yew::{function_component, html, use_effect_with_deps, Html};
use gloo::storage::{LocalStorage, Storage};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::Deserialize;
use js_sys::Date; 
use log::info;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Claims {
    sub: String,
    username: String,
    iat: usize,
    exp: usize,
}

// A helper function to decode the username from a JWT
fn decode_username(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    let payload = parts[1];
    let decoded_payload = URL_SAFE_NO_PAD.decode(payload).ok()?;
    let claims: Claims = serde_json::from_slice(&decoded_payload).ok()?;
    let current_time = Date::new_0().get_time() as usize / 1000; // current time in seconds

    if current_time < claims.exp {
        Some(claims.username)
    } else {
        None
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component(Home)]
pub fn home() -> Html {
    let username = use_state(|| String::new());
    {
        let username = username.clone();
        use_effect_with_deps(
            move |_| {
                info!("I got rendered, yay!");
                if let Ok(token) = LocalStorage::get::<String>("jwtToken") {
                    if !token.is_empty() {
                        info!("JWT Token: {}", token);
                        if let Some(decoded_username) = decode_username(&token) {
                            info!("JWT Token is valid, username: {}", decoded_username);
                            username.set(decoded_username);
                        } else {
                            info!("JWT Token is invalid or expired");
                            LocalStorage::delete("jwtToken");
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

    html! {
        <div class="full-height">
            // Header Section
            <header class="header">
                <nav class="nav">
                    <a href="/" class="nav-logo">{"Pika Chat"}</a>
                    <div style="display: flex; gap: 1rem;">
                        {
                            if !(*username).is_empty() {
                                html! {
                                    <span>{format!("Welcome, {}", *username)}</span>
                                }
                            } else {
                                html! {
                                    <>
                                        <a href="/register" class="nav-link">{"Register"}</a>
                                        <a href="/login" class="nav-link">{"Login"}</a>
                                    </>
                                }
                            }
                        }
                    </div>
                </nav>
            </header>

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
                        html! {
                            <a href="/login" class="button-primary">
                                {"Login First"}
                            </a>
                        }
                    }
                }
            </main>

            // Footer Section
            <footer class="footer">
                <p class="footer-text">{"© 2024 Pika Chat. All rights reserved."}</p>
            </footer>
        </div>
    }
}
