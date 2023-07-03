// Crates
use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn hello_world() -> HttpResponse {
    println!("Hello World");
    HttpResponse::Ok().json("Hello World")
}