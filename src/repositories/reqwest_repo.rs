// Crates
use log::info;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;

// Local Functions
use crate::models::fbu::FbuResponse;
use crate::repositories::help_functions::get_url;

pub struct ReqwestRepo{
    client: Client
}

impl ReqwestRepo {
    pub async fn init() -> Self {
        info!("Initating Reqwest Client");

        let client = Client::new();

        info!("Client created, returning client");
        
        ReqwestRepo{ client }
    }

    // FBU Functions
    pub async fn get_fbu(&self, day: String) -> (Result<FbuResponse, reqwest::Error>,Result<FbuResponse, reqwest::Error>,Result<FbuResponse, reqwest::Error>) {
        info!("Requesting FBU lunch menu");
        
        let mut fresh = String::new();
        let mut street = String::new();
        let mut flow = String::new();

        match day.as_str() {
            "monday" => {
                fresh = get_url("MON_FRESH".to_string());
                street = get_url("MON_STREET".to_string());
                flow = get_url("MON_FLOW".to_string());
            },
            "tuesday" => {
                fresh = get_url("TUE_FRESH".to_string());
                street = get_url("TUE_STREET".to_string());
                flow = get_url("TUE_FLOW".to_string());
            },
            "wednesday" => {
                fresh = get_url("WED_FRESH".to_string());
                street = get_url("WED_STREET".to_string());
                flow = get_url("WED_FLOW".to_string());
            },
            "thursday" => {
                fresh = get_url("THU_FRESH".to_string());
                street = get_url("THU_STREET".to_string());
                flow = get_url("THU_FLOW".to_string());
            },
            "friday" => {
                fresh = get_url("FRI_FRESH".to_string());
                street = get_url("FRI_STREET".to_string());
                flow = get_url("FRI_FLOW".to_string());
            },
            _ => {
                fresh = "".to_string();
                street = "".to_string();
                flow = "".to_string();
            }
        };

        info!("Requesting fresh lunch menu");
        let fresh_response: Result<FbuResponse, reqwest::Error> = self.client.get(fresh)
        .send()
        .await
        .unwrap()
        .json()
        .await;

        info!("Requesting street lunch menu");
        let street_response: Result<FbuResponse, reqwest::Error> = self.client.get(street)
        .send()
        .await
        .unwrap()
        .json()
        .await;

        info!("Requesting flow lunch menu");
        let flow_response: Result<FbuResponse, reqwest::Error> = self.client.get(flow)
        .send()
        .await
        .unwrap()
        .json()
        .await;
        
        info!("Returning FBU lunch menu");
        (fresh_response, street_response, flow_response)
    }


    // Webex Functions
    pub async fn post_to_webex(&self, message: String) -> Result<reqwest::Response, reqwest::Error> {
        let url = get_url("WEBEX_URL".to_string());

        let body = json!({
            "markdown": message
        });

        let webex_response = self.client.post(url)
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await;

    webex_response
    }
}