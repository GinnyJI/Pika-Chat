use yew::prelude::*;
use crate::services::room::RoomMember;

#[derive(Properties, PartialEq)]
pub struct RoomMembersListProps {
    pub members: Vec<RoomMember>,
}

#[function_component(RoomMembersList)]
pub fn room_members_list(props: &RoomMembersListProps) -> Html {
    html! {
        <ul style="list-style-type: none; padding: 0; margin: 0;">
            {
                for props.members.iter().map(|member| {
                    html! {
                        <li style="
                            padding: 0.75rem 0; 
                            border-bottom: 1px solid #e5e7eb; 
                            display: flex; 
                            align-items: center;
                        ">
                            <span style="
                                font-size: 1rem; 
                                font-weight: 500; 
                                color: #374151;
                            ">
                                { &member.username }
                            </span>
                        </li>
                    }
                })
            }
        </ul>
    }
}
