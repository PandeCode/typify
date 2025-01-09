use rspotify::model::CurrentlyPlayingContext;
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::fetch::request_get_cache;

fn parse_time_to_seconds(timestamp: &str) -> Result<f64, ()> {
    let parts: Vec<&str> = timestamp.split(':').collect();
    if parts.len() != 2 {
        return Err(());
    }
    let minutes = parts[0].parse::<f64>().map_err(|_| ())?;
    let seconds = parts[1].parse::<f64>().map_err(|_| ())?;
    Ok(minutes * 60.0 + seconds)
}

#[derive(PartialEq, Properties)]
pub struct TypingProps {
    pub progress: i64,
    pub plain_lyrics: String,
    pub synced_lyrics: Option<String>,
}

#[function_component]
pub fn Typing(props: &TypingProps) -> Html {
    let TypingProps {
        progress,
        synced_lyrics,
        plain_lyrics,
    } = props;

    let progress = *progress as f64;
    let user_input = use_state(|| String::new());
    let input_ref = use_node_ref();

    let on_input = {
        let user_input = user_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                user_input.set(input.value());
            }
        })
    };

    html! {
        <div class="mt-6">
            { if let Some(synced_lyrics) = synced_lyrics {
                let lines: Vec<&str> = synced_lyrics.lines().collect();
                let mut highlighted_lines = vec![];
                let mut next_lines = vec![];
                let mut count = 0;

                for (i, line) in lines.iter().enumerate() {
                    if let Some((time, text)) = line.split_once(']') {
                        if let Ok(time_seconds) = parse_time_to_seconds(&time[1..]) {
                            if time_seconds <= progress {
                                highlighted_lines.push((i, text.trim()));
                            } else {
                                next_lines.push(text.trim());
                                count += 1;
                                if count > 5 {
                                    break;
                                }
                            }
                        }
                    }
                }

                let main_highlight = highlighted_lines.last().map(|(_, text)| *text).unwrap_or("");
                let preceding_lines = highlighted_lines.iter().rev().skip(1).take(4).map(|(_, text)| *text).collect::<Vec<_>>();
                let next_lines = next_lines.iter().rev().collect::<Vec<_>>();

                html! {
                    <>
                        <div class="mt-4">
                            {
                                for preceding_lines.iter().rev().map(|line| {
                                    html! { <p class="text-sm text-gray-600">{ line }</p> }
                                })
                            }
                            <p class="text-lg font-bold text-blue-600">{ main_highlight }</p>
                            {
                                for next_lines.iter().rev().map(|line| {
                                    html! { <p class="text-sm text-gray-600">{ line }</p> }
                                })
                            }
                        </div>

                        <div class="mt-6">
                            <input
                                ref={input_ref.clone()}
                                class="border p-2 w-full"
                                type="text"
                                placeholder="Type here..."
                                value={(*user_input).clone()}
                                oninput={on_input.clone()}
                            />
                            <div class="mt-4">
                                {
                                    if main_highlight.starts_with(&*user_input) {
                                        html! { <p class="text-green-600">{ "Correct so far!" }</p> }
                                    } else {
                                        html! { <p class="text-red-600">{ "Keep trying!" }</p> }
                                    }
                                }
                            </div>
                        </div>
                    </>
                }
            } else {
                html! {
                    <>
                        <h2 class="text-lg font-semibold">{ "Unsynced Lyrics:" }</h2>
                        <p class="whitespace-pre-wrap mt-2">{ plain_lyrics.clone() }</p>
                    </>
                }
            } }
        </div>
    }
}
