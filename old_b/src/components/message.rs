use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageProps {
    pub message: String,
    pub message_type: MessageType,
}

#[derive(Clone, PartialEq)]
pub enum MessageType {
    Loading,
    Error,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    match props.message_type {
        MessageType::Loading => html! {
            <p style="color: #9ca3af;">{ &props.message }</p>
        },
        MessageType::Error => html! {
            <p style="color: red;">{ &props.message }</p>
        },
    }
}
