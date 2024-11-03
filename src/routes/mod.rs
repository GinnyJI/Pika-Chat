pub mod auth;         // Declare the auth module
pub mod test_routes;  // Include the test routes module

// Re-export functions for easier access
pub use auth::{register_user, login_user, logout_user};
pub use test_routes::test_protected_route;
