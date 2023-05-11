use reqwest::Url;
use reqwest::blocking::Client;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, AUTHORIZATION};
use std::env;

const API_ENDPOINT: &str = "https://osu.ppy.sh/api/v2";


fn fetch_access_token(client_id: String, client_secret: String) -> String {
    // Create URL
    let url = Url::parse("https://osu.ppy.sh/oauth/token").expect("Something went wrong parsing URL");

    // Create header map
    let mut header_map = HeaderMap::new();
    header_map.insert(ACCEPT, HeaderValue::from_static("application/json"));
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));

    // Create request
    let client = Client::new();
    let res = client.post(url)
        .headers(header_map)
        .body(format!("client_id={}&client_secret={}&grant_type=client_credentials&scope=public", client_id, client_secret))
        .send().expect("Something went wrong sending request");

    let parsed_json: serde_json::Value = serde_json::from_str(res.text().unwrap().as_str()).unwrap();
    
    parsed_json["access_token"].as_str().expect("Something went wrong parsing Bearer token").to_string()
}

fn fetch_favourite_beatmaps(api_endpoint_url: &Url, token: &String, user_id: i32) {
    let client = Client::new();

    // Create header map
    let mut header_map = HeaderMap::new();
    header_map.insert(ACCEPT, HeaderValue::from_static("application/json"));
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    header_map.insert(AUTHORIZATION, HeaderValue::from_str(("Bearer ".to_owned() + token).as_str()).unwrap());

    println!("{}", ("Bearer ".to_owned() + token).as_str());

    // Send request
    let res = client.get(api_endpoint_url.clone())
        .headers(header_map)
        .send().expect("Something went wrong sending request");

    println!("{}", res.status());
}

fn main() {
    // Load env variables
    dotenv().ok();

    // Set variables from env 
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    
    // Create URL
    let api_endpoint_url = Url::parse(API_ENDPOINT).expect("Something went wrong parsing URL");
    
    // Obtain access token using OAuth
    let access_token = fetch_access_token(client_id, client_secret);
    println!("{}", access_token);

    // Get favorited beatmaps
    //fetch_favourite_beatmaps(&api_endpoint_url, &access_token, 1093872);
}
