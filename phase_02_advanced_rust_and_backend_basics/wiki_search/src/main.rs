use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;
use std::io;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Search On Wikipedia?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error:input");
    let input = input.trim();
    match input {
        "" => {
            println!("Please enter something!...");
            return Ok(());
        }
        input => {
            let name = &input;
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();
            headers.insert(
                USER_AGENT,
                HeaderValue::from_static("Mozilla/5.0(Rust-Bot/1.0)"),
            );
            let search_url=format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",name);
            let search_resp: Value = client
                .get(&search_url)
                .headers(headers.clone())
                .send()
                .await?
                .json()
                .await?;
            let search_list = &search_resp["query"]["search"];
            if let Some(first_result) = search_list.get(0) {
                let raw_data = first_result["title"].as_str().unwrap_or("");
                let correct_data = raw_data.replace(" ", "_");
                let info_url = format!(
                    "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
                    correct_data
                );
                let info_resp: Value = client
                    .get(&info_url)
                    .headers(headers.clone())
                    .send()
                    .await?
                    .json()
                    .await?;
                if let Some(data) = info_resp["extract"].as_str() {
                    println!("Wikipedia CLI result:  {}\nwikipedia ending...\n\n", data);
                }
            } else {
                println!("RESULT NOT FOUND!");
            }
        }
    }
    Ok(())
}
