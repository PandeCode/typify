use std::{collections::HashMap, error::Error, time::Duration};

use gloo_net::http::Request;
use gloo_timers::future::sleep;
use web_sys::js_sys::Date;

pub fn get_local(key: &str) -> Result<String, String> {
    if let Some(window) = web_sys::window() {
        if let Ok(local_storage) = window.local_storage() {
            if let Some(local_storage) = local_storage {
                if let Ok(Some(data)) = local_storage.get_item(key) {
                    Ok(data)
                } else {
                    Err("Item not in LocalStorage".to_string())
                }
            } else {
                Err("LocalStorage not available".to_string())
            }
        } else {
            Err("LocalStorage not found".to_string())
        }
    } else {
        Err("Window not found".to_string())
    }
}

pub fn set_local(key: &str, value: &str) -> Result<(), String> {
    if let Some(window) = web_sys::window() {
        if let Ok(local_storage) = window.local_storage() {
            if let Some(local_storage) = local_storage {
                Ok(local_storage.set_item(key, value).unwrap())
            } else {
                Err("LocalStorage not available".to_string())
            }
        } else {
            Err("LocalStorage not found".to_string())
        }
    } else {
        Err("Window not found".to_string())
    }
}

pub fn remove_local(key: &str) -> Result<(), String> {
    if let Some(window) = web_sys::window() {
        if let Ok(local_storage) = window.local_storage() {
            if let Some(local_storage) = local_storage {
                Ok(local_storage.remove_item(key).unwrap())
            } else {
                Err("LocalStorage not available".to_string())
            }
        } else {
            Err("LocalStorage not found".to_string())
        }
    } else {
        Err("Window not found".to_string())
    }
}
