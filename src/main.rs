use reqwest::Url;
use reqwest::blocking::Client;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use std::env;

const API_ENDPOINT: &str = "https://osu.ppy.sh/api/v2";


fn get_access_token() -> Result<String, reqwest::Error> {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    
    // Send request
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

    res.text()
}

fn main() {
    // Load env variables
    dotenv().ok();
    
    // Obtain access token using OAuth
    let access_token = get_access_token();

    println!("{}", access_token.unwrap());
}
