#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

//   async fn greet(name: &str) -> String {
  //     // Still inside `async fn main`...
  
  // }
  
  #[tauri::command]
async fn greet(request: Request) -> Response {
  println!("{}", request.url);
  let response = send_get_request(&request).await;

  println!("geliyor mu");

  // get response headers and status
  let response_status = response.status().as_u16();
  let mut response_headers: HashMap<String, String> = HashMap::new();

  // add headers
  for (key, value) in response.headers() {
      response_headers.insert(key.to_string(), value.to_str().unwrap().to_owned());
  }

  println!("{:?}", response);

  // get response body
  let response_text = response.text().await.unwrap_or_default();

  Response {
    body: response_text,
    status: response_status,
    headers: response_headers,
  }
}

// Struct for requests
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    url: String,
    method: String,
    body: String,
    body_type: String,
    headers: HashMap<String, String>,
}

// Struct for response
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    body: String,
    status: u16,
    headers: HashMap<String, String>,
}

async fn send_get_request(request: &Request) -> reqwest::Response {
  println!("{}", &request.url);
  // prepare get request
  let mut client = Client::new().get(&request.url);

  // add headers
  for (key, value) in &request.headers {
      client = client.header(key, value);
  }

  // send request
  client.send().await.unwrap()
}