use yew::prelude::*;
use crate::services::room::{RoomMember, UserPresence};

#[derive(Properties, PartialEq)]
pub struct RoomMembersListProps {
    pub members: Vec<RoomMember>,
    pub user_presence: Vec<UserPresence>,
}

#[function_component(RoomMembersList)]
pub fn room_members_list(props: &RoomMembersListProps) -> Html {
    html! {
        <ul style="list-style-type: none; padding: 0; margin: 0;">
            {
                for props.members.iter().map(|member| {
                    let presence = props.user_presence.iter().find(|presence| presence.user_id == member.user_id);
                    let status = match presence {
                        Some(p) if p.is_online => ("Online", "#10B981"), // Green for online
                        _ => ("Offline", "#EF4444"), // Red for offline or not found
                    };

                    html! {
                        <li style="
                            padding: 0.75rem 0; 
                            border-bottom: 1px solid #e5e7eb; 
                            display: flex; 
                            justify-content: space-between; 
                            align-items: center;
                        ">
                            <span style="
                                font-size: 1rem; 
                                font-weight: 500; 
                                color: #374151;
                            ">
                                { &member.username }
                            </span>
                            <span style={format!("font-size: 0.875rem; color: {};", status.1)}>
                                { status.0 }
                            </span>
                        </li>
                    }
                })
            }
        </ul>
    }
}
