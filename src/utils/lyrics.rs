use rspotify::{
    model::TimeRange, prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token,
};

pub fn generate_random_uuid(length: usize) -> String {
    let alphanum: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let mut buf = vec![0u8; length];
    getrandom(&mut buf).unwrap();
    let range = alphanum.len();

    buf.iter()
        .map(|byte| alphanum[*byte as usize % range] as char)
        .collect()
}

// let raw_data = use_state(|| String::new());
// {
//	 let raw_data = raw_data.clone();
//
//	 use_effect(move || {
//		 wasm_bindgen_futures::spawn_local(async move {
//			 let raw_data = raw_data.clone();
//			 if let Some(rm) =
//				 request_get_cache("https://lrclib.net/api/search?q=gloria%20kendrick").await
//			 {
//				 raw_data.set(rm);
//			 };
//		 });
//	 });
// }
