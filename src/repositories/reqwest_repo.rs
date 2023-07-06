// Crates
use log::info;
use reqwest::{Client, Response};
use serde_json::json;

// Repo Modules

// Local Functions
use crate::models::fbu::FbuResponse;


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

    // FBU functions
    pub async fn get_fbu(&self) {
        let url = "https://snaroyveien30-gg.issfoodservices.no/api/articles/article/200147073/p200011994--c200029061/200004464/200005204".to_string();

        let response = self.client.get(url)
        .send()
        .await
        .unwrap();
        println!("{:?}", response);
    }
}