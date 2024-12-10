use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use crate::routes::Route;
use crate::services::message::{get_messages, send_message, Message, MessagesResponse};

pub enum Msg {
    FetchMessages,
    FetchMessagesSuccess(MessagesResponse),
    FetchMessagesFailure(String),
    SendMessage(String),
    SendMessageSuccess,
    SendMessageFailure(String),
    UpdateMessageContent(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct ChatroomProps {
    pub room_id: i64, // Room ID passed as a prop
}

pub struct Chatroom {
    token: Option<String>,
    messages: Vec<Message>,
    error: Option<String>,
    message_content: String,
    loading: bool,
}

impl Component for Chatroom {
    type Message = Msg;
    type Properties = ChatroomProps;

    fn create(ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get::<String>("jwtToken").ok();
        let room_id = ctx.props().room_id;
        let link = ctx.link().clone();

        if let Some(token) = token.clone() {
            link.send_message(Msg::FetchMessages);
            spawn_local(async move {
                match get_messages(&token, room_id).await {
                    Ok(response) => link.send_message(Msg::FetchMessagesSuccess(response)),
                    Err(err) => link.send_message(Msg::FetchMessagesFailure(err)),
                }
            });
        }

        Self {
            token,
            messages: vec![],
            error: None,
            message_content: String::new(),
            loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let room_id = ctx.props().room_id;

        match msg {
            Msg::FetchMessages => {
                self.loading = true;
                self.error = None;
                true
            }
            Msg::FetchMessagesSuccess(response) => {
                self.loading = false;
                self.messages = response.messages;
                true
            }
            Msg::FetchMessagesFailure(err) => {
                self.loading = false;
                self.error = Some(err);
                true
            }
            Msg::SendMessage(content) => {
                if let Some(token) = self.token.clone() {
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        match send_message(&token, room_id, &content).await {
                            Ok(_) => link.send_message(Msg::SendMessageSuccess),
                            Err(err) => link.send_message(Msg::SendMessageFailure(err)),
                        }
                    });
                }
                false
            }
            Msg::SendMessageSuccess => {
                self.message_content.clear();
                ctx.link().send_message(Msg::FetchMessages); // Refresh messages
                true
            }
            Msg::SendMessageFailure(err) => {
                self.error = Some(err);
                true
            }
            Msg::UpdateMessageContent(content) => {
                self.message_content = content;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput_message = ctx
            .link()
            .callback(|e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                Msg::UpdateMessageContent(input.value())
            });

        let onclick_send = {
            let content = self.message_content.clone();
            ctx.link().callback(move |_| Msg::SendMessage(content.clone()))
        };

        html! {
            <div style="min-height: 100vh; display: flex; flex-direction: column; background-color: #f9fafb;">
                // Header
                <header style="background-color: #facc15; padding: 1rem; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); position: static; width: 100%;">
                    <nav style="max-width: 1200px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;">
                        <div style="font-size: 1.5rem; font-weight: bold; color: #1f2937;">
                            <a href="/">{"Pika Chat"}</a>
                        </div>
                    </nav>
                </header>

                // Chatroom Messages
                <main style="flex: 1; padding: 2rem; display: flex; flex-direction: column; align-items: center; text-align: center;">
                    <h1 style="font-size: 2.5rem; font-weight: bold; color: #1f2937; margin-bottom: 1.5rem;">
                        {"Chatroom"}
                    </h1>
                    { if self.loading {
                        html! { <p>{"Loading messages..."}</p> }
                    } else if let Some(error) = &self.error {
                        html! { <p style="color: red;">{format!("Error: {}", error)}</p> }
                    } else {
                        html! {
                            <div>
                                { for self.messages.iter().map(|message| {
                                    html! {
                                        <div style="margin-bottom: 1rem; text-align: left; border-bottom: 1px solid #e5e7eb; padding: 0.5rem;">
                                            <p style="font-weight: bold; color: #1f2937;">{&message.username}</p>
                                            <p>{&message.content}</p>
                                            <p style="font-size: 0.875rem; color: #9ca3af;">{&message.timestamp}</p>
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    }}
                    </main>

                // Message Input
                <footer style="padding: 1rem; background-color: #facc15; width: 100%; display: flex; justify-content: center;">
                    <input
                        type="text"
                        placeholder="Type your message..."
                        value={self.message_content.clone()}
                        oninput={oninput_message}
                        style="flex: 1; padding: 0.5rem; margin-right: 0.5rem; border: 1px solid #e5e7eb; border-radius: 0.375rem;" />
                    <button onclick={onclick_send} style="background-color: #1f2937; color: #fff; padding: 0.5rem 1rem; border-radius: 0.375rem; font-weight: bold;">
                        {"Send"}
                    </button>
                </footer>
            </div>
        }
    }
}
