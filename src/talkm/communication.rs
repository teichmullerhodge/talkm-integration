use actix_web::http;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::Serialize;

use crate::talkm::routes;

#[derive(Serialize, Debug)]
pub struct Message {

    pub number: String,
    pub body: String, 
    pub queueId: i32,
    pub userId: i32, 

}

pub async fn send_message(message: &Message, connectionKey: String) -> Result<bool, reqwest::Error> {

    let authValue: String = format!("Bearer {}", connectionKey);

    print!("Auth headers: {}", authValue);
    print!("URL: {}", routes::MESSAGE_URL);
    print!("Body: {:?}", message);


    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&authValue).unwrap());
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    let client = Client::new();

    let response = client
        .post(routes::MESSAGE_URL)
        .headers(headers)
        .json(&message)
        .send()
        .await?;

    if response.status() == http::StatusCode::OK {
        
        print!("Message send!");
        return Ok(true)


    }    

    let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
    eprintln!("Error sending message: {}", error_text);

    Ok(false)
}
