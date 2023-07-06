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
    let default_port: u16 = 4000;
    //let port_env = match env::var("LUNCHBOT_PORT") {
    //    Ok(v) => v.to_string()parse().unwrap(),
    //    Err(_) => {
    //        format!("Error loading ENV variables\n")
    //    }
    //};
    //let port: u16 = port_env.parse().unwrap();
    
    let chad: u16 = match env::var("LUNCHBOT_PORT") {
        Ok(v) => v.parse().unwrap(),
        Err(_) => {
            format!("ENV Variable missing, defaulting");
            default_port
        }
    };

    
    println!("Starting at port {}", chad);
    HttpServer::new(move ||{
        App::new()
            .service(hello_world)
    })
        .bind(("127.0.0.1", chad))?
        .run()
        .await
}
