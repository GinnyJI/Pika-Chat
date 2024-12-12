use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::Deserialize;
use js_sys::Date;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn decode_username(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    let payload = parts[1];
    let decoded_payload = URL_SAFE_NO_PAD.decode(payload).ok()?;
    let claims: Claims = serde_json::from_slice(&decoded_payload).ok()?;
    let current_time = Date::new_0().get_time() as usize / 1000; // current time in seconds

    if current_time < claims.exp {
        Some(claims.username)
    } else {
        None
    }
}

pub fn decode_userid(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    let payload = parts[1];
    let decoded_payload = URL_SAFE_NO_PAD.decode(payload).ok()?;
    let claims: Claims = serde_json::from_slice(&decoded_payload).ok()?;
    let current_time = Date::new_0().get_time() as usize / 1000; // current time in seconds

    if current_time < claims.exp {
        Some(claims.sub)
    } else {
        None
    }
}
