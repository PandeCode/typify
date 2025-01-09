use std::{collections::HashMap, time::Duration};

use gloo_net::http::Request;
use gloo_timers::future::sleep;
use web_sys::js_sys::Date;

const CACHE_EXPIRY_MS: f64 = 24.0 * 60.0 * 60.0 * 1000.0; // 1 day in milliseconds
const CACHE_DELAY_MS: u64 = 0; // To simulate loading, not necessay for now

pub async fn request_get_cache(url: &str) -> Option<String> {
    if let Some(window) = web_sys::window() {
        if let Ok(local_storage) = window.local_storage() {
            if let Some(local_storage) = local_storage {
                // Retrieve cached data
                if let Ok(Some(item)) = local_storage.get_item(url) {
                    sleep(Duration::from_millis(CACHE_DELAY_MS)).await; // adjust duration as needed

                    // Split the timestamp and data
                    if let Some((timestamp_str, data)) = item.split_once('|') {
                        if let Ok(timestamp) = timestamp_str.parse::<f64>() {
                            // Check if the data is still valid
                            let current_time = Date::now();
                            if current_time - timestamp < CACHE_EXPIRY_MS {
                                return Some(data.to_string());
                            } else {
                                // Remove the old data and proceed
                                if let Err(err) = local_storage.remove_item(url) {
                                    log::warn!("Failed to remove old data from the cache");
                                }
                            }
                        }
                    }
                }

                // If cache is expired or doesn't exist, fetch fresh data
                if let Ok(response) = Request::get(&url).send().await {
                    if let Ok(text) = response.text().await {
                        // Prepend the current timestamp to the data
                        let timestamped_data = format!("{}|{}", Date::now(), text);
                        // Store the new data in local storage
                        if let Err(err) = local_storage.set_item(url, &timestamped_data) {
                            log::warn!(
                                "Failed to store in local storage: {}",
                                err.as_string().unwrap()
                            );
                        }
                        return Some(text);
                    }
                }
            } else {
                log::warn!("LocalStorage not available");
            }
        } else {
            log::warn!("LocalStorage not found");
        }
    } else {
        log::warn!("Window not found");
    }

    None
}
