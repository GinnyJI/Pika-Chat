use yew::prelude::*;
use gloo::timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use crate::services::room::{RoomMember, UserPresence, get_user_presence};

#[derive(Properties, PartialEq)]
pub struct RoomMembersListProps {
    pub members: Vec<RoomMember>,
    pub room_id: i64,
    pub token: Option<String>,
}

#[allow(dead_code)]
pub enum Msg {
    FetchUserPresence,
    FetchUserPresenceSuccess(Vec<UserPresence>),
    FetchUserPresenceError(String),
}

pub struct RoomMembersList {
    user_presence: Vec<UserPresence>,
    fetch_presence_error: Option<String>,
    fetch_presence_interval: Option<Interval>,
}

impl Component for RoomMembersList {
    type Message = Msg;
    type Properties = RoomMembersListProps;

    fn create(ctx: &Context<Self>) -> Self {
        let room_id = ctx.props().room_id;
        let token = ctx.props().token.clone();
        let link = ctx.link().clone();

        let fetch_presence_interval = Some(Interval::new(1000, move || {
            if let Some(token) = token.clone() {
                let link = link.clone(); // Clone the link inside the closure
                spawn_local(async move {
                    gloo::console::log!("Fetching user presence...");
                    match get_user_presence(&token, room_id).await {
                        Ok(presence) => {
                            gloo::console::log!(format!("User presence fetched: {:?}", presence));
                            link.send_message(Msg::FetchUserPresenceSuccess(presence));
                        }
                        Err(err) => {
                            gloo::console::log!(format!("Error fetching user presence: {:?}", err));
                            link.send_message(Msg::FetchUserPresenceError(err));
                        }
                    }
                });
            } else {
                gloo::console::log!("No token available for fetching user presence.");
            }
        }));

        Self {
            user_presence: vec![],
            fetch_presence_error: None,
            fetch_presence_interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchUserPresenceSuccess(presence) => {
                self.user_presence = presence;
                true // Re-render with updated presence
            }
            Msg::FetchUserPresenceError(err) => {
                self.fetch_presence_error = Some(err);
                true // Re-render to display the error
            }
            Msg::FetchUserPresence => {
                false // No state changes to trigger a re-render
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(interval) = self.fetch_presence_interval.take() {
            interval.cancel(); // Stop the periodic fetch
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul style="list-style-type: none; padding: 0; margin: 0;">
                {
                    for ctx.props().members.iter().map(|member| {
                        let presence = self.user_presence.iter().find(|presence| presence.user_id == member.user_id);
                        let status = match presence {
                            Some(p) if p.is_online => ("Online", "#10B981"), // Green for online
                            _ => ("Offline", "#EF4444"), // Red for offline or not found
                        };
    
                        html! {
                            <li style="
                                padding: 0.75rem 0; 
                                border-bottom: 1px solid #e5e7eb; 
                                display: flex; 
                                align-items: center;
                            ">
                                // Avatar image
                                <img 
                                    src={member.avatar_url.clone()} 
                                    alt="User Avatar" 
                                    style="width: 40px; height: 40px; border-radius: 50%; margin-right: 0.75rem;" 
                                />
                                // Username and status
                                <div style="flex: 1;">
                                    <span style="
                                        font-size: 1rem; 
                                        font-weight: 500; 
                                        color: #374151;
                                    ">
                                        { &member.username }
                                    </span>
                                    <span style={format!("font-size: 0.875rem; color: {}; margin-left: 0.5rem;", status.1)}>
                                        { status.0 }
                                    </span>
                                </div>
                            </li>
                        }
                    })
                }
                {
                    if let Some(error) = &self.fetch_presence_error {
                        html! {
                            <li style="
                                padding: 0.75rem; 
                                color: #EF4444;
                                font-size: 0.875rem;
                            ">
                                { format!("Error fetching user presence: {}", error) }
                            </li>
                        }
                    } else {
                        html! {}
                    }
                }
            </ul>
        }
    }    
}
