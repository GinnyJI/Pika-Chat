use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use crate::services::websocket::{WebSocketService, BroadcastMessage};
use crate::services::room::{get_user_presence, UserPresence};
use crate::routes::Route;
use web_sys::HtmlInputElement;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::services::utils::{decode_username, decode_userid};
use crate::services::auth::logout;
use crate::services::room::RoomMember;
use crate::services::room::get_room_members;
use crate::components::panel::Panel;
use crate::components::room_member_list::RoomMembersList;
use crate::components::message::{Message, MessageType};

pub enum Msg {
    SendMessage,
    ReceiveMessage(BroadcastMessage), // Updated to handle structured messages
    UpdateMessageInput(String),
    WebSocketConnected,
    WebSocketDisconnected,
    WebSocketError(String),
    LogoutClicked,
    LogoutSuccess,
    LogoutFailure(String),
    FetchRoomMembers,
    FetchRoomMembersSuccess(Vec<RoomMember>),
    FetchRoomMembersError(String),
    FetchUserPresence,
    FetchUserPresenceSuccess(Vec<UserPresence>),
    FetchUserPresenceError(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub room_id: i64,
    pub room_name: Option<String>,
}

pub struct ChatRoom {
    token: Option<String>,
    ws_service: Option<WebSocketService>,
    message_input: String,
    messages: Vec<BroadcastMessage>, // Store structured messages
    error: Option<String>,
    username: String,
    avatar_url: Option<String>,
    userid: String,
    room_members: Vec<RoomMember>,
    room_members_error: Option<String>,
    user_presence: Vec<UserPresence>,
    user_presence_error: Option<String>,
}

impl Component for ChatRoom {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get::<String>("jwtToken").ok();
        let username = token.as_ref().and_then(|t| decode_username(t)).unwrap_or_default();
        let userid = token.as_ref().and_then(|t| decode_userid(t)).unwrap_or_default();
        let avatar_url = LocalStorage::get::<String>("avatarUrl").ok(); // Retrieve avatar URL from local storage

        let component = Self {
            token,
            ws_service: None,
            message_input: String::new(),
            messages: vec![],
            error: None,
            username,
            avatar_url,
            userid,
            room_members: vec![],
            room_members_error: None,
            user_presence: vec![],
            user_presence_error: None,
        };

        // Fetch room members on component creation
        let link = ctx.link().clone();
        link.send_message(Msg::FetchRoomMembers);

        // Fetch user presence on component creation
        link.send_message(Msg::FetchUserPresence);

        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let navigator = ctx.link().navigator().expect("No navigator available");
        match msg {
            Msg::SendMessage => {
                if let Some(ws_service) = &mut self.ws_service {
                    ws_service.send_message(&self.message_input);
                    self.message_input.clear();
                }
                true
            }
            Msg::ReceiveMessage(message) => {
                self.messages.push(message);
                true
            }
            Msg::UpdateMessageInput(input) => {
                self.message_input = input;
                true
            }
            Msg::WebSocketConnected => {
                self.error = None;
                true
            }
            Msg::WebSocketDisconnected => {
                self.error = Some("Disconnected from the server.".to_string());
                true
            }
            Msg::WebSocketError(error) => {
                self.error = Some(error);
                true
            }
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
            Msg::FetchRoomMembers => {
                if let Some(token) = self.token.clone() {
                    let room_id: i64 = ctx.props().room_id;
                    let link = ctx.link().clone();

                    spawn_local(async move {
                        match get_room_members(&token, room_id).await {
                            Ok(members) => link.send_message(Msg::FetchRoomMembersSuccess(members)),
                            Err(err) => link.send_message(Msg::FetchRoomMembersError(err)),
                        }
                    });
                }
                false
            }
            Msg::FetchRoomMembersSuccess(members) => {
                self.room_members = members;
                self.room_members_error = None;
                true
            }
            Msg::FetchRoomMembersError(err) => {
                self.room_members_error = Some(err);
                true
            }
            Msg::FetchUserPresence => {
                if let Some(token) = self.token.clone() {
                    let room_id = ctx.props().room_id;
                    let link = ctx.link().clone();
            
                    spawn_local(async move {
                        match get_user_presence(&token, room_id).await {
                            Ok(presence) => link.send_message(Msg::FetchUserPresenceSuccess(presence)),
                            Err(err) => link.send_message(Msg::FetchUserPresenceError(err)),
                        }
                    });
                }
                false
            }
            Msg::FetchUserPresenceSuccess(presence) => {
                self.user_presence = presence;
                self.user_presence_error = None;
                true
            }
            Msg::FetchUserPresenceError(err) => {
                self.user_presence_error = Some(err);
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.ws_service.is_none() {
            let room_id = ctx.props().room_id;
            let userid = self.userid.clone();
            let link = ctx.link().clone();

            let on_message = link.callback(Msg::ReceiveMessage);
            let on_error = link.callback(Msg::WebSocketError);
            let on_connect = link.callback(|_| Msg::WebSocketConnected);

            let ws_service = WebSocketService::new(
                &room_id.to_string(),
                &userid.to_string(),
                on_message,
                on_error,
                on_connect,
            );

            self.ws_service = Some(ws_service);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // TODO: make the member list a reactive state
        // TODO: add avatar for the members
        let room_members_view = if !self.room_members.is_empty() {
            html! {
                <Panel>
                    <h2 style="
                        font-size: 1.5rem; 
                        font-weight: bold; 
                        margin-bottom: 1rem; 
                        color: #1f2937; 
                        text-align: center;
                    ">
                        {"Room Members"}
                    </h2>
                    <RoomMembersList 
                        members={self.room_members.clone()} 
                        user_presence={self.user_presence.clone()}
                    />
                </Panel>
            }
        } else if let Some(error) = &self.room_members_error {
            html! {
                <Panel>
                    <h2 style="
                        font-size: 1.5rem; 
                        font-weight: bold; 
                        margin-bottom: 1rem; 
                        color: #1f2937; 
                        text-align: center;
                    ">
                        {"Room Members"}
                    </h2>
                    <Message message={format!("Failed to load room members: {}", error)} message_type={MessageType::Error} />
                </Panel>
            }
        } else {
            html! {
                <Panel>
                    <h2 style="
                        font-size: 1.5rem; 
                        font-weight: bold; 
                        margin-bottom: 1rem; 
                        color: #1f2937; 
                        text-align: center;
                    ">
                        {"Room Members"}
                    </h2>
                    <Message message={"Loading room members...".to_string()} message_type={MessageType::Loading} />
                </Panel>
            }
        };

        html! {
            <div style="min-height: 100vh; display: flex; flex-direction: column; background-color: #f9fafb;">
                <Header
                    username={Some(self.username.clone())}
                    avatar_url={self.avatar_url.clone()}
                    on_logout={ctx.link().callback(|_| Msg::LogoutClicked)}
                />
                {room_members_view} // Left panel with room members
                <main
                    style="flex: 1; padding: 2rem; display: flex; flex-direction: column; align-items: center; text-align: center;"
                >
                    <h1 style="font-size: 2.5rem; font-weight: bold; color: #1f2937;">{ format!("Chat Room: {}", ctx.props().room_id) }</h1>
                    <div style="width: 100%; max-width: 800px; margin-bottom: 2rem; text-align: left;">
                        <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; padding: 1rem; max-height: 400px; overflow-y: auto; background-color: #ffffff;">
                            {
                                for self.messages.iter().map(|msg| {
                                    html! {
                                        <p style={
                                            if msg.is_system {
                                                "padding: 0.5rem 0; border-bottom: 1px solid #e5e7eb; color: gray;"
                                            } else {
                                                "padding: 0.5rem 0; border-bottom: 1px solid #e5e7eb;"
                                            }
                                        }>
                                            { format!("{}: {}", if msg.is_system { "System" } else { &msg.username }, msg.message) }
                                        </p>
                                    }
                                })
                            }
                        </div>
                        <div style="display: flex; margin-top: 1rem;">
                            <input
                                type="text"
                                value={self.message_input.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateMessageInput(input.value())
                                })}
                                placeholder="Type your message"
                                style="flex: 1; padding: 0.75rem; border: 1px solid #e5e7eb; border-radius: 0.5rem; margin-right: 0.5rem;"
                            />
                            <button
                                onclick={ctx.link().callback(|_| Msg::SendMessage)}
                                style="padding: 0.75rem 1.5rem; background-color: #1f2937; color: #ffffff; border: none; border-radius: 0.5rem; cursor: pointer;"
                            >
                                {"Send"}
                            </button>
                        </div>
                    </div>
                </main>
                <Footer />
            </div>
        }
    }

}
