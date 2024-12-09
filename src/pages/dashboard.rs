use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use crate::routes::Route;
use crate::services::auth::logout;
use crate::services::room::{get_rooms, RoomsResponse};
use crate::components::room_card::RoomCard;

pub enum Msg {
    LogoutClicked,
    LogoutSuccess,
    LogoutFailure(String),
    FetchRooms,
    FetchRoomsSuccess(RoomsResponse),
    FetchRoomsFailure(String),
}

pub struct Dashboard {
    token: Option<String>,
    rooms: Option<RoomsResponse>,
    error: Option<String>,
    loading: bool,
}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get::<String>("jwtToken").ok();
        let link = ctx.link().clone();

        // Fetch rooms if token exists
        if let Some(token) = token.clone() {
            link.send_message(Msg::FetchRooms);
            spawn_local(async move {
                match get_rooms(&token).await {
                    Ok(rooms) => link.send_message(Msg::FetchRoomsSuccess(rooms)),
                    Err(err) => link.send_message(Msg::FetchRoomsFailure(err)),
                }
            });
        }

        Self {
            token,
            rooms: None,
            error: None,
            loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let navigator = ctx.link().navigator().expect("No navigator available");

        match msg {
            Msg::LogoutClicked => {
                if let Some(token) = self.token.clone() {
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        let result = logout(&token).await;
                        match result {
                            Ok(_) => link.send_message(Msg::LogoutSuccess),
                            Err(err) => link.send_message(Msg::LogoutFailure(err)),
                        }
                    });
                } else {
                    navigator.push(&Route::Home);
                }
                false
            }
            Msg::LogoutSuccess => {
                LocalStorage::delete("jwtToken");
                navigator.push(&Route::Home);
                true
            }
            Msg::LogoutFailure(_error) => {
                LocalStorage::delete("jwtToken");
                navigator.push(&Route::Home);
                true
            }
            Msg::FetchRooms => {
                self.loading = true;
                self.error = None;
                true
            }
            Msg::FetchRoomsSuccess(rooms) => {
                self.loading = false;
                self.rooms = Some(rooms);
                self.error = None;
                true
            }
            Msg::FetchRoomsFailure(err) => {
                self.loading = false;
                self.rooms = None;
                self.error = Some(err);
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
                        {"Welcome to the Dashboard page! Below is the list of available rooms."}
                    </p>
                    if self.loading {
                        <p>{"Loading rooms..."}</p>
                    } else if let Some(error) = &self.error {
                        <p style="color: red;">{format!("Error: {}", error)}</p>
                    } else if let Some(rooms) = &self.rooms {
                        <div class="room-card-list">
                            {
                                for rooms.rooms.iter().map(|room| {
                                    html! {
                                        <RoomCard room={room.clone()} />
                                    }
                                })
                            }
                        </div>
                    } else {
                        <p>{"No rooms available."}</p>
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
}
