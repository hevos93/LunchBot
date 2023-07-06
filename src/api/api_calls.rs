// Crates
use actix_web::{get, HttpResponse};
use log::info;

#[get("/")]
pub async fn hello_world() -> HttpResponse {
    info!("Hello World()");
    println!("Hello World");
    HttpResponse::Ok().json("Hello World")
}