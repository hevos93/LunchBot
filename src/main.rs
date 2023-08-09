// Crates
use std::env;
use dotenv::dotenv;
use actix_web::{App, HttpServer};
use log::{info, warn};
use pretty_env_logger as logger;

// Repo Modules
mod api;
mod models;
mod repositories;

// Local Functions
use crate::api::api_calls::healtcheck;
use crate::api::fbu::{fbu, fbu_json};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inits
    dotenv().ok();
    logger::init();

    info!("Starting LunchBot");

    // Config variables
    let default_port: u16 = 4000;    
    let port: u16 = match env::var("LUNCHBOT_PORT") {
        Ok(v) => v.parse().unwrap(),
        Err(_) => {
            warn!{"ENV: LUNCHBOT_PORT missing, defaulting to {}", default_port};
            default_port
        }
    };

    // Starting API
    info!("LunchBot is running at {}", port);
    HttpServer::new(move ||{
        App::new()
            .service(healtcheck)
            .service(fbu)
            .service(fbu_json)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
