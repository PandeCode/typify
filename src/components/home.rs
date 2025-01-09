use std::ops::Deref;

use crate::{
    components::{lyrics::Lyrics, now_playing::NowPlaying},
    utils::{
        fetch::request_get_cache,
        get_href, get_spotify, get_state,
        local::{get_local, remove_local, set_local},
        reload_page, remove_params_from_url, set_href,
    },
};

use gloo_timers::callback::Interval;
use rspotify::{model::TimeRange, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, Token};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_interval;

const CLIENT_ID: &str = std::env!("RSPOTIFY_CLIENT_ID");
const CLIENT_SECRET: &str = std::env!("RSPOTIFY_CLIENT_SECRET");

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;

    let spotify = use_state(|| get_spotify());
    let got_token = use_state(|| false);
    let now_playing = use_state(|| None);
    let user = use_state(|| None);

    // Handle Callback URI
    {
        let spotify = spotify.clone();
        let user = user.clone();
        let got_token = got_token.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(code) = (*spotify).parse_response_code(&get_href()) {
                    log::info!("Got code: {}", code.clone());
                    if let Err(err) = (*spotify).request_token(&code).await {
                        log::error!("Failed to get user token: {:?}", err.to_string());
                    } else {
                        got_token.set(true);
                        spotify.set((*spotify).clone());

                        let token = (*spotify).token.lock().await.unwrap();
                        if let Ok(token_str) = serde_json::to_string::<Token>(
                            &(<Option<Token> as Clone>::clone(&(*token)).unwrap()),
                        ) {
                            set_local("spotify-token", &token_str);
                        } else {
                            log::error!("Failed to convert token to string");
                        }

                        log::info!("Requested user token successfully.");

                        let window = web_sys::window().expect("Missing Window");
                        window.history().unwrap().replace_state_with_url(
                            &wasm_bindgen::JsValue::null(),
                            "",
                            window.location().pathname().ok().as_deref(),
                        );
                    }
                }
            });
        });
    }

    // Refresh Token TODO: Test
    {
        let spotify = spotify.clone();
        let got_token = got_token.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(token) = get_local("spotify-token") {
                    if let Ok(mut sp_token) = (*spotify).token.lock().await {
                        let token = serde_json::from_str::<Token>(&token).unwrap();
                        if !token.is_expired() {
                            log::info!("Load cache");
                            *sp_token = Some(token);
                            spotify.set((*spotify).clone());
                            got_token.set(true);
                        } else {
                            log::info!("Refresh");
                            match (*spotify).refresh_token().await {
                                Ok(_) => {
                                    got_token.set(true);
                                }
                                Err(err) => {
                                    log::error!("Failed to refresh token: {}", err);
                                }
                            }
                        }
                    }
                }
            });
        });
    }

    // Now playgin interval
    let has_interval = use_state(|| false);
    {
        let has_interval = has_interval.clone();
        let spotify = spotify.clone();
        let now_playing = now_playing.clone();
        let user = user.clone();
        let got_token = got_token.clone();

        use_effect_with(got_token, move |got_token| {
            if **got_token {
                wasm_bindgen_futures::spawn_local(async move {
                    match spotify.current_user().await {
                        Ok(_user) => {
                            user.set(Some(_user));
                        }
                        Err(_err) => {
                            log::error!("Failed to get user: {}", _err.to_string());
                        }
                    }

                    if !*has_interval {
                        let spotify = spotify.clone();
                        let now_playing = now_playing.clone();

                        Interval::new(2000, move || {
                            let spotify = spotify.clone();
                            let now_playing = now_playing.clone();

                            spawn_local(async move {
                                if let Ok(sp_np) = (*spotify).current_user_playing_item().await {
                                    now_playing.set(sp_np);
                                }
                            });
                        })
                        .forget();

                        has_interval.set(true);
                    }
                });
            }
        });
    }

    let get_user = {
        let spotify = spotify.clone();
        let user = user.clone();

        move |_| {
            let spotify = spotify.clone();
            let user = user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match (*spotify).current_user().await {
                    Ok(_user) => {
                        user.set(Some(_user));
                    }
                    Err(_err) => {
                        log::error!("Failed to get user: {}", _err.to_string());
                    }
                }
            });
        }
    };

    let login = {
        let spotify = spotify.clone();

        move |_| {
            let auth_url = (*spotify).get_authorize_url(true);
            set_href(auth_url.unwrap());
        }
    };

    html! {
        <div
            class="flex flex-col justify-center items-center min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100 space-y-6 p-4"
        >
            { if *got_token {
                if let Some(user) = &(*user) {
                    html! {
                        <div class="flex flex-row w-full max-w-5xl space-x-4">
                            <div class="flex flex-col space-y-4 w-1/3">
                                <p class="text-lg font-semibold">{format!("Welcome, {}", user.display_name.clone().unwrap_or("{{error}}".to_string()))}</p>
                                <NowPlaying now_playing={now_playing.clone()}/>
                            </div>

                            <div class="flex-1">
                                <Lyrics now_playing={now_playing.clone()}/>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-center space-y-4">
                            <p class="text-sm text-gray-600 dark:text-gray-400">{"Fetching user data..."}</p>
                            <button
                                onclick={get_user}
                                class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-300 dark:focus:ring-blue-700">
                                {"Get User"}
                            </button>
                        </div>
                    }
                }
            } else {
                html! {
                    <button
                        onclick={login}
                        class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-300 dark:focus:ring-green-700">
                        {"Login"}
                    </button>
                }
            } }
        </div>
    }
}
