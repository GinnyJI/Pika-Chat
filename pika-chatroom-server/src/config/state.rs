use std::collections::HashSet;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Define the global token blacklist
lazy_static! {
    pub static ref TOKEN_BLACKLIST: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}
