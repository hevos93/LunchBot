use log::{info, error};
use std::env;

pub fn get_url (url_id: String) -> String {
    let default = "".to_string();
    info!("Getting URL");

    let url: String = match env::var(&url_id) {
        Ok(v) => v,
        Err(_) => {
            error!("ENV: {} missing, returning empty string ", url_id);
            default
        }
    };
    info!("Returning URL");
    url
}