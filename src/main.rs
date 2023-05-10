use reqwest;
use dotenv::dotenv;
use std::env;

fn main() {
    // Load env variables
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");

    // Send request
    let res = reqwest::blocking::get("http://httpbin.org/get").unwrap();
    println!("{}", res.status());
}
