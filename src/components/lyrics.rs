use std::ops::Deref;

use crate::components::typing::Typing;
use rspotify::model::CurrentlyPlayingContext;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{types::LyricResponses, utils::fetch::request_get_cache};

#[derive(PartialEq, Properties)]
pub struct LyricsProps {
    pub now_playing: UseStateHandle<Option<CurrentlyPlayingContext>>,
}

#[function_component]
pub fn Lyrics(props: &LyricsProps) -> Html {
    let LyricsProps { now_playing } = props;

    let np = now_playing.clone();

    let lyrics = use_state(|| None);
    let progress = use_state(|| 0);

    {
        let lyrics = lyrics.clone();
        let progress = progress.clone();
        use_effect_with(np.clone(), move |np| {
            let np = (*(np.clone())).clone();

            spawn_local(async move {
                if let Some(np) = np {
                    if let Some(item) = &np.item {
                        progress.set(np.progress.unwrap_or(Default::default()).num_seconds());

                        match item {
                            rspotify::model::PlayableItem::Track(track) => {
                                if let Some(data) = request_get_cache(
                                    format!(
                                        "https://lrclib.net/api/search?q={}%20{}",
                                        track.name,
                                        track.artists.iter().fold(String::new(), |mut acc, o| {
                                            acc.push_str(o.name.as_str());
                                            acc.push_str("%20");
                                            acc
                                        })
                                    )
                                    .as_str(),
                                )
                                .await
                                {
                                    if let Some(data) =
                                        serde_json::from_str::<LyricResponses>(&data).ok()
                                    {
                                        lyrics.set(Some(data));
                                    } else {
                                        log::error!("Failed to parse json.");
                                    }
                                } else {
                                    log::error!("Failed to get lyrics");
                                }
                            }

                            _ => {}
                        }
                    }
                }
            });
        });
    }

    let lyrics = lyrics.clone();
    let progress = progress.clone();

    if let Some(lyrics) = lyrics.deref() {
        if let Some(lyric) = lyrics.first() {
            html! {
                <div
                    class="container mx-auto p-4 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                >
                    <div class="text-center">
                        <h1 class="text-2xl font-bold">{ &lyric.track_name }</h1>
                    </div>
                    <Typing
                        synced_lyrics={lyric.synced_lyrics.clone()}
                        plain_lyrics={lyric.plain_lyrics.clone()}
                        progress={*progress}
                    />
                </div>
            }
        } else {
            html! {
                <div
                    class="container mx-auto p-4 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100 text-center"
                >
                    { "no lyrics found" }
                </div>
            }
        }
    } else {
        html! {
            <div
                class="container mx-auto p-4 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100 text-center"
            >
                { "loading lyrics" }
            </div>
        }
    }
}
