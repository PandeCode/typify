pub(crate) mod fetch;
pub(crate) mod local;

use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, Token};
use yew::prelude::*;

use local::{get_local, set_local};

pub fn set_href(url: String) -> Result<(), wasm_bindgen::JsValue> {
    web_sys::window()
        .expect("Missing Window")
        .location()
        .set_href(url.as_str())
}

pub fn svg_asset(f: &str) -> String {
    format!("/assets/svgs/{}.svg", f)
}

pub fn remove_params_from_url() {
    let window = web_sys::window().expect("Missing Window");
    window.history().unwrap().replace_state_with_url(
        &wasm_bindgen::JsValue::null(),
        "",
        window.location().pathname().ok().as_deref(),
    );
}
pub fn reload_page() {
    set_href(web_sys::window().unwrap().location().href().unwrap());
}
pub fn get_href() -> String {
    web_sys::window()
        .expect("Missing Window")
        .location()
        .href()
        .expect("Couldn't get url")
}

pub fn get_state() -> String {
    match get_local("spotify-state") {
        Ok(state) => state,
        Err(_) => {
            let state = generate_random_uuid(16);
            set_local("spotify-state", &state);
            state
        }
    }
}
pub fn generate_random_uuid(length: usize) -> String {
    let alphanum: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let mut buf = vec![0u8; length];
    getrandom::getrandom(&mut buf).unwrap();
    let range = alphanum.len();

    buf.iter()
        .map(|byte| alphanum[*byte as usize % range] as char)
        .collect()
}

const CLIENT_ID: &str = std::env!("RSPOTIFY_CLIENT_ID");
const CLIENT_SECRET: &str = std::env!("RSPOTIFY_CLIENT_SECRET");

pub fn get_spotify() -> AuthCodeSpotify {
    let oauth = OAuth {
        scopes: scopes!(
            "user-read-email",
            "user-read-private",
            "user-read-playback-state",
            "user-read-currently-playing"
        ),
        state: get_state(),
        redirect_uri: "http://localhost:8080".to_owned(),
        ..Default::default()
    };

    let creds = Credentials::new(CLIENT_ID, CLIENT_SECRET);
    AuthCodeSpotify::new(creds, oauth)
}
