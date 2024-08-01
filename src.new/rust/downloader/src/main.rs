mod fake_useragent;
use std::{collections::HashMap, error::Error};

async fn total(url: String) -> Result<(i64, String), Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok((1, "".to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
