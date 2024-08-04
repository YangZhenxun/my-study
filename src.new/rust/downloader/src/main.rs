mod fake_useragent;
use std::error::Error;
use log::{trace, warn};
use regex::Regex;

use reqwest::{header::{HeaderMap, HeaderValue, USER_AGENT}, Client};

macro_rules! client_res {
    ($url: expr, $client: expr, $ua: expr) => {
        $client.get($url)
            .header(USER_AGENT, $ua.parse::<HeaderValue>()?)
            .send()
            .await?
    };
}

async fn total(url: &str, ua: String) -> Result<(Option<u64>, Option<String>), Box<dyn Error>> {
    trace!("Using total function.");
    let client = Client::new();
    let res = client_res!(url, client, ua);
    match get_filename(res.headers().clone()).await? {
        Some((typ, filename)) => {
            match typ {
                Some(typ) => {
                    if typ != "attachment".to_string(){
                        warn!("This download file is a HTML file!")
                    }
                }
                None => {},
            };
            Ok((res.content_length(), filename))
        },
        None => Ok((res.content_length(), None))
    }
}

#[inline]
async fn get_filename(map: HeaderMap) -> Result<Option<(Option<String>, Option<String>)>, Box<dyn std::error::Error>>{
    match map.get("Content-Disposition") {
        Some(_filename) => {
            let rea = Regex::new(".*(?P<type>[a-zA-Z]+); filename=(?P<filename>.+))?")?;
            Ok(rea.captures(format!("{:#?}", map).as_str()).and_then(|cap|{
                Some((cap.name("type").map(|typ| typ.as_str().to_string()),
                cap.name("filename").map(|filename|filename.as_str().to_string())))
            }))
        },
        None => Ok(None)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let user_agent = fake_useragent::UserAgent::new().await?;
    let ua = user_agent.random()?;
    let (tot, name) = total("https://httpbin.org/get", ua.clone()).await?;
    match tot{
        Some(t) => println!("{}", t),
        None => ()
    }
    match name{
        Some(nm) => println!("{}", nm),
        None => ()
    }
    let resp = client_res!("https://httpbin.org/get", Client::new(), ua).text().await?;
    println!("{}", resp);
    Ok(())
}
