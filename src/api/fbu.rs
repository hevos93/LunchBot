// Crates
use log::{info, error};
use actix_web::{get, HttpResponse, web::Path};
// Repo Modules
use crate::repositories::reqwest_repo::ReqwestRepo;
use crate::repositories::fbu::{format_fbu, format_fbu_json};

// Local Functions
     
#[get("/fbu/{day}")]
pub async fn fbu(path: Path<String>) -> HttpResponse {
    info!("FBU");

    let day = path.into_inner();

    let client = ReqwestRepo::init().await;

    let response = match client.get_fbu(day).await {
        (Ok(v1),Ok(v2),Ok(v3)) => (v1,v2,v3),
        _ => {
            let error_msg = "Something went wrong with lunch requests!".to_string();
            error!("{}", error_msg);
            return HttpResponse::InternalServerError().body(error_msg)
        } 
    };

    let lunch_menu = format_fbu(response);

    let response = match client.post_to_webex(lunch_menu).await {
        Ok(v) => v,
        Err(e) => {
            error!("An error occured! -> {}", e);
            return HttpResponse::InternalServerError().body("Something went wrong! Check the logs")
        }
    };
    
    let _body =  match response.error_for_status() {
        Ok(v) => {
            info!("Response code from Webex was Ok");
            v.error_for_status().is_ok()
        },
        Err(e) => {
            error!("Response code from Webex was not Ok. -> {}", e);
            return HttpResponse::InternalServerError().body("The reported response code from Webex was not Ok, check the logs")
    }
    };

    info!("Returning HttpResponse");
    HttpResponse::Ok().body("Lunch menu fetched and returned")
}

#[get("/json/fbu/{day}")]
pub async fn fbu_json(path: Path<String>) -> HttpResponse {
    info!("FBU JSON");

    let day = path.into_inner();

    let client = ReqwestRepo::init().await;

    let response = match client.get_fbu(day).await {
        (Ok(v1),Ok(v2),Ok(v3)) => (v1,v2,v3),
        _ => {
            let error_msg = "Something went wrong with lunch requests!".to_string();
            error!("{}", error_msg);
            return HttpResponse::InternalServerError().body(error_msg)
        } 
    };

    let response = format_fbu_json(response);

    info!("Returning FBU JSON");
    HttpResponse::Ok().json(response)
}

#[get("/md/fbu/{day}")]
pub async fn fbu_md(path: Path<String>) -> HttpResponse {
    info!("FBU MD");

    let day = path.into_inner();

    let client = ReqwestRepo::init().await;

    let response = match client.get_fbu(day).await {
        (Ok(v1),Ok(v2),Ok(v3)) => (v1,v2,v3),
        _ => {
            let error_msg = "Something went wrong with lunch requests!".to_string();
            error!("{}", error_msg);
            return HttpResponse::InternalServerError().body(error_msg)
        } 
    };

    let lunch_menu = format_fbu(response);

    info!("Returning FBU MD");
    HttpResponse::Ok().body(lunch_menu)
}