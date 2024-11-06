pub mod auth;         // Declare the auth module
pub mod test_routes;  // Include the test routes module
pub mod room;

// Re-export functions for easier access
pub use auth::{register_user, login_user, logout_user};
pub use test_routes::test_protected_route;
pub use room::{create_room, join_room, get_rooms, Room, RoomInfo};
