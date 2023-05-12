use dotenv::dotenv;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Url;
use std::collections::HashSet;
use std::env;

const API_ENDPOINT: &str = "https://osu.ppy.sh/api/v2";

fn fetch_access_token(client_id: String, client_secret: String) -> String {
    // Create URL
    let url =
        Url::parse("https://osu.ppy.sh/oauth/token").expect("Something went wrong parsing URL");

    // Create header map
    let mut header_map = HeaderMap::new();
    header_map.insert(ACCEPT, HeaderValue::from_static("application/json"));
    header_map.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    // Create request
    let client = Client::new();
    let res = client
        .post(url)
        .headers(header_map)
        .body(format!(
            "client_id={}&client_secret={}&grant_type=client_credentials&scope=public",
            client_id, client_secret
        ))
        .send()
        .expect("Something went wrong sending request");

    let parsed_json: serde_json::Value =
        serde_json::from_slice(res.bytes().unwrap().as_ref()).unwrap();

    parsed_json["access_token"]
        .as_str()
        .expect("Something went wrong parsing Bearer token")
        .to_string()
}

fn fetch_favourite_beatmaps(token: &String, user_id: u32) -> HashSet<String> {
    let client = Client::new();
    let mut offset = 1;

    // Create hashset to hold unique beatmapset ids
    let mut favorited_beatmap_ids: HashSet<String> = HashSet::new();

    // Create header map
    let mut header_map = HeaderMap::new();
    header_map.insert(ACCEPT, HeaderValue::from_static("application/json"));
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    header_map.insert(
        AUTHORIZATION,
        HeaderValue::from_str(("Bearer ".to_owned() + token).as_str()).unwrap(),
    );


    loop {
        // Create URL
        let api_endpoint_url = Url::parse(
            (API_ENDPOINT.to_owned()
                + format!("/users/{}/beatmapsets/favourite?limit=100&offset={}", user_id.to_string(), offset.to_string()).as_str())
            .as_str(),
        )
        .expect("Something went wrong parsing URL");

        // Send request
        let res_text = client
            .get(api_endpoint_url)
            .headers(header_map.clone())
            .send()
            .expect("Something went wrong sending request")
            .text()
            .unwrap();

        let re = Regex::new(r#""beatmapset_id":\s*(\d+)"#).unwrap();

        for captures in re.captures_iter(res_text.as_str()) {
            favorited_beatmap_ids.insert(captures[1].to_string());
        }

        // Break if response doesn't return 100 beatmap ids ( done )
        // Can just check if total / 100 doesn't has a remainder
        if favorited_beatmap_ids.len() % 100 != 0 {
            break;
        }

        offset += 100;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("{}", favorited_beatmap_ids.len());

    favorited_beatmap_ids
}

fn main() {
    // Load env variables
    dotenv().ok();

    // Set variables from env
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");

    // Obtain access token using OAuth
    let access_token = fetch_access_token(client_id, client_secret);

    // Get favorited beatmaps
    let favourite_beatmap_ids = fetch_favourite_beatmaps(&access_token, 14852499);

    println!("{}", favourite_beatmap_ids.len());
    //for id in favourite_beatmap_ids.iter() {
    //    println!("{}", id);
    //}
}
