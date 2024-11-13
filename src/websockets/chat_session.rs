use actix::{Actor, StreamHandler, Context, Addr, Message, Handler, AsyncContext};
use actix_web_actors::ws;
use std::collections::{HashMap, HashSet};

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
}

impl RoomServer {
    // Constructor to create a new RoomServer instance.
    pub fn new() -> Self {
        RoomServer {
            rooms: HashMap::new(),
            user_sessions: HashMap::new(),
        }
    }

    // Adds a user to a specified room.
    pub fn add_user(&mut self, room_id: RoomId, user_id: UserId, addr: Addr<ChatSession>) {
        self.rooms.entry(room_id).or_insert_with(HashSet::new).insert(user_id);
        self.user_sessions.insert(user_id, addr);
    }

    // Removes a user from a specified room.
    pub fn remove_user(&mut self, room_id: RoomId, user_id: UserId) {
        if let Some(users) = self.rooms.get_mut(&room_id) {
            users.remove(&user_id);
            if users.is_empty() {
                self.rooms.remove(&room_id);
            }
        }
        self.user_sessions.remove(&user_id);
    }

    // Broadcasts a message to all users in a specified room.
    fn broadcast_to_room(&self, room_id: RoomId, message: &str) {
        if let Some(user_ids) = self.rooms.get(&room_id) {
            for &user_id in user_ids {
                if let Some(addr) = self.user_sessions.get(&user_id) {
                    addr.do_send(ChatMessage { message: message.to_string() });
                }
            }
        }
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
        self.add_user(msg.room_id, msg.user_id, msg.addr);
        println!("User {} added to room {}", msg.user_id, msg.room_id);
    }
}

// Handler for RemoveUser to remove a user from a room.
impl Handler<RemoveUser> for RoomServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveUser, _: &mut Self::Context) {
        self.remove_user(msg.room_id, msg.user_id);
        println!("User {} removed from room {}", msg.user_id, msg.room_id);
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
    pub room_server: Addr<RoomServer>,
}

impl ChatSession {
    pub fn new(room_id: RoomId, user_id: UserId, room_server: Addr<RoomServer>) -> Self {
        ChatSession { room_id, user_id, room_server }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Send AddUser message to RoomServer to track this user
        self.room_server.do_send(AddUser {
            room_id: self.room_id,
            user_id: self.user_id,
            addr: ctx.address(),
        });

        // Announce that the user has joined the room
        self.room_server.do_send(BroadcastMessage {
            room_id: self.room_id,
            message: format!("User {} has joined the room.", self.user_id),
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
            message: format!("User {} has left the room.", self.user_id),
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
                message: format!("User {}: {}", self.user_id, text),
            });
        }
    }
}
