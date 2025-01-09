use wasm_bindgen::JsValue;
use web_sys::window;
use weblog::console_error;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Theme {
    Dark,
    Light,
}

fn get_theme_match_media() -> Result<Theme, JsValue> {
    if let Some(window) = window() {
        if let Some(media_matches) = window.match_media("(prefers-color-scheme: dark)")? {
            Ok(if media_matches.matches() {
                Theme::Dark
            } else {
                Theme::Light
            })
        } else {
            console_error!("Unable to get media matches.");
            Err(JsValue::from("Unable to get media matches."))
        }
    } else {
        console_error!("Unable to get window.");
        Err(JsValue::from("Unable to get window."))
    }
}

pub fn setup_theme() -> Result<(), JsValue> {
    if let Some(window) = window() {
        if let Some(local_storage) = window.local_storage()? {
            let theme = local_storage.get_item("color-theme");
            let theme = if let Ok(theme) = theme {
                if let Some(theme) = theme {
                    match (theme).as_str() {
                        "dark" => Some(Theme::Dark),
                        "light" => Some(Theme::Light),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if theme.is_none() {
                local_storage.set_item(
                    "color-theme",
                    match get_theme_match_media() {
                        Ok(Theme::Dark) => "dark",
                        Ok(Theme::Light) => "light",
                        _ => {
                            console_error!("Failed to get match_media.");
                            return Err(JsValue::from("Failed to geta match_media"));
                        }
                    },
                )?;
            }

            if let Some(document) = window.document() {
                if let Some(document_element) = document.document_element() {
                    let cl = document_element.class_list();
                    match theme {
                        Some(Theme::Dark) => {
                            cl.add_1("dark")?;
                        }
                        Some(Theme::Light) => {
                            cl.remove_1("dark")?;
                        }
                        None => console_error!("Unable to get match media for 'color-theme'."),
                    }
                } else {
                    console_error!("Unable to get document element.")
                }
            } else {
                console_error!("Unable to get document.")
            }
        } else {
            console_error!("Unable to get window.")
        }
    }

    Ok(())
}

pub fn get_theme() -> Result<Theme, JsValue> {
    if let Some(window) = window() {
        if let Some(local_storage) = window.local_storage()? {
            let theme = local_storage.get_item("color-theme");
            if let Ok(theme) = theme {
                if let Some(theme) = theme {
                    match theme.as_str() {
                        "dark" => Ok(Theme::Dark),
                        "light" => Ok(Theme::Light),
                        _ => Err(JsValue::from(format!(
                            "Unable to match local_storage with value '{}'.",
                            theme
                        ))),
                    }
                } else {
                    setup_theme()?;
                    Err(JsValue::from("Unable to access local_storage."))
                }
            } else {
                setup_theme()?;
                console_error!(theme.err());
                Err(JsValue::from("Unable to access local_storage."))
            }
        } else {
            console_error!("Unable to get window.");
            Err(JsValue::from("Unable to get window."))
        }
    } else {
        console_error!("Unable to get document.");
        Err(JsValue::from("Unable to get window."))
    }
}

pub fn set_theme(theme: Theme) -> Result<(), JsValue> {
    if let Some(window) = window() {
        if let Some(ls) = window.local_storage()? {
            ls.set_item(
                "color-theme",
                match theme {
                    Theme::Dark => "dark",
                    Theme::Light => "light",
                },
            )?;
        } else {
            console_error!("Unable to get window.")
        }

        if let Some(document) = window.document() {
            if let Some(document_element) = document.document_element() {
                let cl = document_element.class_list();
                match theme {
                    Theme::Dark => {
                        cl.add_1("dark")?;
                    }
                    Theme::Light => {
                        cl.remove_1("dark")?;
                    }
                }
            } else {
                console_error!("Unable to get document element.")
            }
        }
    } else {
        console_error!("Unable to get document.")
    }
    Ok(())
}

pub fn toggle_theme() -> Result<(), JsValue> {
    set_theme(match get_theme()? {
        Theme::Dark => Theme::Light,
        Theme::Light => Theme::Dark,
    })?;

    Ok(())
}
