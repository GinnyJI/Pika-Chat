pub mod auth;  // Declare the auth module

// Re-export the functions for easier access
pub use auth::{register_user, login_user, logout_user};
