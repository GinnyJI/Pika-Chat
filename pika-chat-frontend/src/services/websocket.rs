use gloo::net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use futures::{StreamExt, SinkExt};
use std::rc::Rc;
use std::cell::RefCell;
use serde::Deserialize;

// Define the structure for incoming messages
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct BroadcastMessage {
    pub room_id: i64,
    pub message: String,
    pub is_system: bool,
    pub username: String,
}

pub struct WebSocketService {
    write: Option<Rc<RefCell<futures::stream::SplitSink<WebSocket, Message>>>>,
    sender: Rc<Callback<BroadcastMessage>>,
    room_id: String,
}

impl WebSocketService {
    pub fn new(
        room_id: &str,
        userid: &str,
        sender: Callback<BroadcastMessage>,
        _on_error: Callback<String>,
        on_connect: Callback<()>,
    ) -> Self {
        let ws_url = format!(
            "ws://127.0.0.1:8080/ws/rooms/{}?user_id={}",
            room_id, userid
        );
    
        let ws = WebSocket::open(&ws_url).expect("Failed to open WebSocket");
    
        let (write, mut read) = ws.split();
    
        let sender = Rc::new(sender);
    
        // Spawn a task to handle incoming messages
        let sender_clone = sender.clone();
        spawn_local(async move {
            while let Some(Ok(Message::Text(msg))) = read.next().await {
                match serde_json::from_str::<BroadcastMessage>(&msg) {
                    Ok(parsed_msg) => sender_clone.emit(parsed_msg),
                    Err(_) => log::warn!("Failed to parse incoming message: {}", msg),
                }
            }
        });
    
        // Emit connection success
        on_connect.emit(());
    
        Self {
            write: Some(Rc::new(RefCell::new(write))),
            sender,
            room_id: room_id.to_string(),
        }
    }

    pub fn send_message(&self, message: &str) {
        if let Some(write) = &self.write {
            let msg = message.to_string();
            let write = write.clone();
            spawn_local(async move {
                let mut write = write.borrow_mut();
                if let Err(e) = write.send(Message::Text(msg)).await {
                    log::error!("Failed to send message: {:?}", e);
                }
            });
        } else {
            log::warn!("WebSocket write sink is not available. Message not sent.");
        }
    }

    pub fn close(&mut self) {
        if let Some(write) = self.write.take() {
            spawn_local(async move {
                let mut write = write.borrow_mut();
                write.close().await.ok();
            });
        }
    }
}

impl Drop for WebSocketService {
    fn drop(&mut self) {
        self.close();
    }
}
