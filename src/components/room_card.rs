use yew::prelude::*;
use crate::services::room::Room;

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub room: Room,
}

#[function_component(RoomCard)]
pub fn room_card(props: &RoomProps) -> Html {
    html! {
        <div class="room-card">
            <h3 class="room-card-title">{&props.room.room_name}</h3>
            <p class="room-card-detail">{format!("Room ID: {}", props.room.room_id)}</p>
            <p class="room-card-detail">{format!("Owner ID: {}", props.room.user_id)}</p>
        </div>
    }
}
