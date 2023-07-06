// Crates
use log::info;
use actix_web::{get, HttpResponse};
// Repo Modules
use crate::repositories::reqwest_repo::ReqwestRepo;

// Local Functions

#[get("/fbu")]
pub async fn fbu() -> HttpResponse {
    info!("FBU");

    let client = ReqwestRepo::init().await;
    let response = client.get_fbu().await;
    
    HttpResponse::UnavailableForLegalReasons().body("Get fukked fedboi")
}