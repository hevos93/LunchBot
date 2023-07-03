// Crates
use std::env;
use dotenv::dotenv;
use actix_web::{App, HttpServer};

// Repo Modules
mod api;
mod models;
mod repositories;

// Local Functions
use crate::api::api_calls::hello_world;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, i am LunchBot");
    // Inits
    dotenv().ok();

    // Config variables
    let port = match env::var("LUNCHBOT_PORT") {
        Ok(v) => v.to_string(),
        Err(_) => {
            format!("Error loading ENV variables\n")
        }
    };

    
    println!("Starting at port {}", port);
    HttpServer::new(move ||{
        App::new()
            .service(hello_world)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
