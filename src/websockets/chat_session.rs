use actix::{Actor, StreamHandler, Context, Addr, Message, Handler, AsyncContext};
use actix_web_actors::ws;
use std::collections::{HashMap, HashSet};
use crate::models::presence::{GetRoomPresence, UserPresence};

// Define RoomId and UserId types for better readability
pub type RoomId = i64;
pub type UserId = i64;

// Message type for broadcasting a message within a room.
pub struct BroadcastMessage {
    pub room_id: RoomId,
    pub message: String,
}

impl Message for BroadcastMessage {
    type Result = ();
}

// Message type for adding a user with session address to a room
pub struct AddUser {
    pub room_id: RoomId,
    pub user_id: UserId,
    pub username: String,
    pub addr: Addr<ChatSession>,
}

impl Message for AddUser {
    type Result = ();
}

// Message type for removing a user from a room
pub struct RemoveUser {
    pub room_id: RoomId,
    pub user_id: UserId,
}

impl Message for RemoveUser {
    type Result = ();
}

// RoomServer is an Actix actor responsible for managing chat rooms and users within them.
pub struct RoomServer {
    rooms: HashMap<RoomId, HashSet<UserId>>,               // Tracks user IDs in each room
    user_sessions: HashMap<UserId, Addr<ChatSession>>,      // Tracks active sessions by user ID
    user_presence: HashMap<UserId, bool>,                   // Tracks online/offline status of each user
    user_names: HashMap<UserId, String>,                   // Maps user IDs to usernames
}

impl RoomServer {
    // Constructor to create a new RoomServer instance.
    pub fn new() -> Self {
        RoomServer {
            rooms: HashMap::new(),
            user_sessions: HashMap::new(),
            user_presence: HashMap::new(),
            user_names: HashMap::new(),
        }
    }

    // Adds a user to a specified room.
    pub fn add_user(&mut self, room_id: RoomId, user_id: UserId, addr: Addr<ChatSession>) {
        self.rooms.entry(room_id).or_insert_with(HashSet::new).insert(user_id);
        self.user_sessions.insert(user_id, addr);
        self.user_presence.insert(user_id, true);                                // Set user as online
    }

    // Removes a user from a specified room, marking them as offline
    pub fn remove_user(&mut self, room_id: RoomId, user_id: UserId) {
        if let Some(users) = self.rooms.get_mut(&room_id) {
            users.remove(&user_id);                   // Remove user from room set
            self.user_presence.insert(user_id, false); // Mark user as offline
            if users.is_empty() {                     // If no users left, remove the room
                self.rooms.remove(&room_id);
            }
        }
        self.user_sessions.remove(&user_id);          // Remove user's active session
    }    
    
    // Broadcasts a message to all users in a specified room.
    fn broadcast_to_room(&self, room_id: RoomId, message: &str) {
        if let Some(user_ids) = self.rooms.get(&room_id) {
            for &user_id in user_ids {
                if let Some(addr) = self.user_sessions.get(&user_id) {
                    if let Some(username) = self.user_names.get(&user_id) {
                        addr.do_send(ChatMessage {
                            message: format!("{}: {}", username, message),
                        });
                    }
                }
            }
        }
    }

    // Sets a user's presence to online.
    pub fn set_user_online(&mut self, user_id: UserId) {
        self.user_presence.insert(user_id, true);
    }

    // Sets a user's presence to offline.
    pub fn set_user_offline(&mut self, user_id: UserId) {
        self.user_presence.insert(user_id, false);
    }

    // Retrieves the presence status of all users in a room, including offline users
    pub fn get_room_presence(&self, room_id: RoomId) -> Vec<UserPresence> {
        let mut presence_list = Vec::new();
    
        if let Some(user_ids) = self.rooms.get(&room_id) {
            // Track presence of users who are currently in the room (online)
            for &user_id in user_ids {
                let is_online = *self.user_presence.get(&user_id).unwrap_or(&false);
                presence_list.push(UserPresence::new(user_id, is_online));
            }
        }
    
        // Add users who were in the room but are now offline, based on user_presence
        for (&user_id, &is_online) in &self.user_presence {
            if !is_online && self.user_presence.contains_key(&user_id) {
                presence_list.push(UserPresence::new(user_id, is_online));
            }
        }
    
        presence_list
    }    
}

impl Actor for RoomServer {
    type Context = Context<Self>;
}

// Handler for BroadcastMessage to send a message to all users in a room.
impl Handler<BroadcastMessage> for RoomServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Self::Context) {
        println!("Broadcasting to room {}: {}", msg.room_id, msg.message);
        self.broadcast_to_room(msg.room_id, &msg.message);
    }
}

// Handler for AddUser to add a user to a room.
impl Handler<AddUser> for RoomServer {
    type Result = ();

    fn handle(&mut self, msg: AddUser, _: &mut Self::Context) {
        self.add_user(msg.room_id, msg.user_id, msg.addr.clone());
        self.user_names.insert(msg.user_id, msg.username.clone());
        self.set_user_online(msg.user_id);
        println!("User {} ({}) added to room {}", msg.user_id, msg.username, msg.room_id);
    }
}

// Handler for RemoveUser to remove a user from a room.
impl Handler<RemoveUser> for RoomServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveUser, _: &mut Self::Context) {
        self.remove_user(msg.room_id, msg.user_id);
        self.set_user_offline(msg.user_id); // Set user as offline when removed
        println!("User {} removed from room {}", msg.user_id, msg.room_id);
    }
}

// Handler for GetRoomPresence to get the presence status of all users in a room.
impl Handler<GetRoomPresence> for RoomServer {
    type Result = Vec<UserPresence>;

    fn handle(&mut self, msg: GetRoomPresence, _: &mut Self::Context) -> Self::Result {
        self.get_room_presence(msg.room_id)
    }
}

// ChatMessage represents a message sent from RoomServer to a ChatSession.
pub struct ChatMessage {
    pub message: String,
}

impl Message for ChatMessage {
    type Result = ();
}

// ChatSession represents an individual WebSocket connection for a user in a room.
pub struct ChatSession {
    pub room_id: RoomId,
    pub user_id: UserId,
    pub username: String,
    pub room_server: Addr<RoomServer>,
}

impl ChatSession {
    pub fn new(room_id: RoomId, user_id: UserId, username: String, room_server: Addr<RoomServer>) -> Self {
        ChatSession { room_id, user_id, username, room_server }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Send AddUser message to RoomServer to track this user
        self.room_server.do_send(AddUser {
            room_id: self.room_id,
            user_id: self.user_id,
            username: self.username.clone(),
            addr: ctx.address(),
        });

        // Announce that the user has joined the room
        self.room_server.do_send(BroadcastMessage {
            room_id: self.room_id,
            message: format!("⚡ Pika Pi! Welcome to the chat, {}!", self.username),
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // Send RemoveUser message to RoomServer to stop tracking this user
        self.room_server.do_send(RemoveUser {
            room_id: self.room_id,
            user_id: self.user_id,
        });

        // Announce that the user has left the room
        self.room_server.do_send(BroadcastMessage {
            room_id: self.room_id,
            message: format!("Pika-pika... Goodbye, {}!", self.username),
        });
    }
}

// ChatSession handler for receiving ChatMessage from RoomServer
impl Handler<ChatMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.message);
    }
}

// Implement StreamHandler to handle incoming WebSocket messages from the client.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        // Handle text messages received over the WebSocket connection
        if let Ok(ws::Message::Text(text)) = msg {
            // Send the received message to the RoomServer for broadcasting
            self.room_server.do_send(BroadcastMessage {
                room_id: self.room_id,
                message: format!("{}", text),
            });

            // Celebrate a great message with Easter egg
            let lower_text = text.to_lowercase();
            if lower_text.contains("great") 
                || lower_text.contains("awesome") 
                || lower_text.contains("amazing") 
                || lower_text.contains("ginny") 
            {
                self.room_server.do_send(BroadcastMessage {
                    room_id: self.room_id,
                    message: format!("⚡ Pikachuuu~! Great message from {}!", self.username),
                });
            }
        }
    }
}
