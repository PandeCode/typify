use std::ops::Deref;

use rspotify::model::CurrentlyPlayingContext;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NowPlayingProps {
    pub now_playing: UseStateHandle<Option<CurrentlyPlayingContext>>,
}

#[function_component]
pub fn NowPlaying(props: &NowPlayingProps) -> Html {
    let NowPlayingProps { now_playing } = props;

    let np = now_playing.clone();
    let now_playing = np.deref();

    if let Some(np) = now_playing {
        let progress = np.progress.unwrap_or(Default::default()).num_seconds();
        let is_playing = np.is_playing;

        if let Some(item) = &np.item {
            match item {
                rspotify::model::PlayableItem::Track(track) => {
                    let name = &track.name;
                    let duration = track.duration.num_seconds();
                    let artists = track.artists.iter().fold(String::new(), |mut acc, o| {
                        acc.push_str(o.name.as_str());
                        acc.push_str(",");
                        acc
                    });
                    let image = track
                        .album
                        .images
                        // .iter()
                        .first()
                        .unwrap();
                    // .min_by_key(|img| img.height)

                    html! {
                        <div class="">
                            <img src={image.url.clone()} class="w-256 h-256 rounded shadow" />
                            <div class="flex flex-col">
                                <p class="text-sm text-gray-600">
                                    { format!("{:.0}:{:02}", progress / 60, progress % 60) }
                                    { " / " }
                                    { format!("{:.0}:{:02}", duration / 60, duration % 60) }
                                </p>
                                <p class="text-sm text-gray-600">{ artists }</p>
                                <p class="text-sm text-gray-600 font-semibold">{ name }</p>
                            </div>
                        </div>
                    }
                }
                rspotify::model::PlayableItem::Episode(_episode) => {
                    html! {
                        <>
                            <p class="text-sm text-gray-599">{ "Cannot get lyrics of podcast!" }</p>
                        </>
                    }
                }
            }
        } else {
            html! { <p class="text-sm text-gray-599">{ "Play Something..." }</p> }
        }
    } else {
        html! { <p class="text-sm text-gray-599">{ "Loading now playing data..." }</p> }
    }
}
