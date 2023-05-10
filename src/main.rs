use reqwest;

fn main() {
    let res = reqwest::blocking::get("http://httpbin.org/get").unwrap();
    println!("{}", res.status());
}
