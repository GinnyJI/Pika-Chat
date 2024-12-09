use yew::prelude::*;
use yew::{function_component, html, use_effect_with_deps, Html};
// use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
// use crate::routes::Route;
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
                        info!("JWT Token: {}", token); // Log the token
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
        <div style="min-height: 100vh; display: flex; flex-direction: column; background-color: #f9fafb;">
            // Header Section
            <header style="background-color: #facc15; padding: 1rem; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); position: static; width: 100%;">
                <nav style="max-width: 1200px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;">
                    <div style="font-size: 1.5rem; font-weight: bold; color: #1f2937;">
                        <a href="/">{"Pika Chat"}</a>
                    </div>
                    <div style="display: flex; gap: 1rem;">
                    {
                        if !(*username).is_empty() {
                            html! {
                                <span>{format!("Welcome, {}", *username)}</span>
                            }
                        } else {
                            html! {
                                <>
                                    <a href="/register" style="color: #1f2937; text-decoration: none; font-weight: 500; transition: color 0.2s; hover: color: #4b5563;">
                                        {"Register"}
                                    </a>
                                    <a href="/login" style="color: #1f2937; text-decoration: none; font-weight: 500; transition: color 0.2s; hover: color: #4b5563;">
                                        {"Login"}
                                    </a>
                                </>
                            }
                        }
                    }
                    
                    </div>
                </nav>
            </header>

            // Main Section
            <main style="flex: 1; padding: 2rem; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center;">
                <h1 style="font-size: 2.5rem; font-weight: bold; color: #1f2937; margin-bottom: 1.5rem;">
                    {"Welcome to Pika Chat"}
                </h1>
                <p style="font-size: 1.125rem; color: #4b5563; margin-bottom: 2rem; max-width: 600px;">
                    {"Connect with your friends, chat in real-time, and enjoy the best social experience online. Join now to explore!"}
                </p>
                <img src="static/pikachu.png" alt="Pikachu image" style="width: 10rem; height: 10rem; margin-bottom: 2rem;" />
                {
                    if !(*username).is_empty() {
                        html! {
                            <a href="/dashboard" style="background-color: #facc15; padding: 1rem 2rem; border-radius: 0.5rem; font-weight: 600; color: #1f2937; text-decoration: none; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); transition: transform 0.2s; hover: transform: scale(1.05);">
                                {"Go to Dashboard"}
                            </a>
                        }
                    } else {
                        html! {
                            <a href="/login" style="background-color: #facc15; padding: 1rem 2rem; border-radius: 0.5rem; font-weight: 600; color: #1f2937; text-decoration: none; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); transition: transform 0.2s; hover: transform: scale(1.05);">
                                {"Login First"}
                            </a>
                        }
                    }
                }
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
