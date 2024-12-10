use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use crate::routes::Route;
use crate::services::auth::logout;
use crate::services::room::{get_rooms, RoomsResponse, Room, RoomInfo, create_room, add_room_member};
use crate::components::room_card::RoomCard;
use web_sys::HtmlInputElement;

pub enum Msg {
    LogoutClicked,
    LogoutSuccess,
    LogoutFailure(String),
    FetchRooms,
    FetchRoomsSuccess(RoomsResponse),
    FetchRoomsFailure(String),
    UpdateRoomName(String),
    CreateRoom,
    CreateRoomSuccess(Room),
    CreateRoomFailure(String),
    AddMemberToRoomSuccess,
    AddMemberToRoomFailure(String),
}

pub struct Dashboard {
    token: Option<String>,
    rooms: Option<RoomsResponse>,
    error: Option<String>,
    loading: bool,
    room_name_input: String,
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
            room_name_input: String::new(),
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
            Msg::UpdateRoomName(name) => {
                self.room_name_input = name;
                true
            }
            Msg::CreateRoom => {
                if let Some(token) = self.token.clone() {
                    let link = ctx.link().clone();
                    let room_info = RoomInfo {
                        room_name: self.room_name_input.clone(),
                    };
                    spawn_local(async move {
                        match create_room(&token, &room_info).await {
                            Ok(room) => link.send_message(Msg::CreateRoomSuccess(room)),
                            Err(err) => link.send_message(Msg::CreateRoomFailure(err)),
                        }
                    });
                }
                false
            }
            Msg::CreateRoomSuccess(room) => {
                if let Some(ref mut rooms) = self.rooms {
                    rooms.rooms.push(room.clone());
                }
                self.room_name_input.clear(); // Clear input on success
            
                // Add the current user as a member of the newly created room
                if let Some(token) = self.token.clone() {
                    let link = ctx.link().clone();
                    let room_id = room.room_id;
                    spawn_local(async move {
                        match add_room_member(&token, room_id).await {
                            Ok(_) => link.send_message(Msg::AddMemberToRoomSuccess),
                            Err(err) => link.send_message(Msg::AddMemberToRoomFailure(err)),
                        }
                    });
                }
                true
            }
            Msg::CreateRoomFailure(err) => {
                self.error = Some(err);
                true
            }
            Msg::AddMemberToRoomSuccess => {
                // Handle success (e.g., update UI or log success)
                true
            }
            Msg::AddMemberToRoomFailure(err) => {
                // Handle failure (e.g., display an error message)
                self.error = Some(format!("Failed to join the room: {}", err));
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_logout = ctx.link().callback(|_| Msg::LogoutClicked);
        let oninput_room_name = ctx
        .link()
        .callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into(); // Now works because HtmlInputElement is imported
            Msg::UpdateRoomName(input.value())
        });
        let onclick_create_room = ctx.link().callback(|_| Msg::CreateRoom);

        html! {
            <div class="full-height">
                // Header Section
                <header class="header">
                    <nav class="nav">
                        <a class="nav-logo" href="/">{"Pika Chat"}</a>
                        <a href="#" onclick={onclick_logout} class="nav-link">{"Logout"}</a>
                    </nav>
                </header>

                // Main Section
                <main class="main">
                    <h1 class="heading">{"Dashboard"}</h1>
                    <div class="input-group">
                        <input
                            type="text"
                            value={self.room_name_input.clone()}
                            oninput={oninput_room_name}
                            placeholder="Enter room name"
                            class="input-box"
                        />
                        <button
                            onclick={onclick_create_room}
                            class="button"
                        >
                            {"Create Room"}
                        </button>
                    </div>
                    <p class="description">
                        {"Welcome to the Dashboard page! Below is the list of available rooms."}
                    </p>
                    if self.loading {
                        <p>{"Loading rooms..."}</p>
                    } else if let Some(error) = &self.error {
                        <p class="error">{format!("Error: {}", error)}</p>
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
                <footer class="footer">
                    <p class="footer-text">{"© 2024 Pika Chat. All rights reserved."}</p>
                </footer>
            </div>
        }        
    }
}
