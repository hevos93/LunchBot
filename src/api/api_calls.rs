// Crates
use actix_web::{get, HttpResponse};
use log::info;

#[get("/health")]
pub async fn healtcheck() -> HttpResponse {
    info!("Healtcheck");
    HttpResponse::Ok().json("LunchBot is still alive\n")
}