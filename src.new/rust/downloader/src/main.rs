<<<<<<< Updated upstream
<<<<<<< Updated upstream
<<<<<<< Updated upstream
<<<<<<< Updated upstream
<<<<<<< Updated upstream
mod fake_useragent;
use std::error::Error;
use log::{trace, warn};
use regex::Regex;
use rayon::prelude::*;
use reqwest::{header::{HeaderMap, HeaderValue, USER_AGENT}, Client};

macro_rules! client_res {
    ($url: expr, $client: expr, $ua: expr) => {
        $client.get($url)
            .header(USER_AGENT, $ua.parse::<HeaderValue>()?)
            .send()
            .await?
    };
}

async fn total(url: &str, ua: String, cli: Client) -> Result<(Option<u64>, Option<String>), Box<dyn Error>> {
    trace!("Using total function.");
    let res = client_res!(url, cli, ua);
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

async fn split(filesize: u64, num_threads: u64) -> Vec<(u64, u64)>{
    let chuck_size= filesize / num_threads;
    let i = (1..=num_threads).into_par_iter().map(|i|{
        let start = chuck_size * i;
        let end = if i <= num_threads - 1 { start+chuck_size } else { filesize };
        (start, end)
    });
    i.collect()
}
=======
=======
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes
use fake_user_agent::get_rua;
use std::collections::HashMap;
>>>>>>> Stashed changes


async fn total(url: String, session: reqwest::Client, header: reqwest::header::HeaderMap) -> Result<i64, Box<dyn std::error::Error>>{
    let req = session.get(url)
        .headers(header)
        .send()
        .await?
        .json::<HashMap<String, serde_json::value::Value>>()
        .await?;
    println!("{:#?}", req);
    let cont_len: String = String::from("Content-Length");
    let tot: i64= req.get(&cont_len).unwrap().clone().as_i64().unwrap();
    println!("{}", tot.clone());
    Ok(tot)
}


async fn total(url: String, session: reqwest::Client, header: reqwest::header::HeaderMap) -> Result<i64, Box<dyn std::error::Error>>{
    let req = session.get(url)
        .headers(header)
        .send()
        .await?
        .json::<HashMap<String, serde_json::value::Value>>()
        .await?;
    println!("{:#?}", req);
    let cont_len: String = String::from("Content-Length");
    let tot: i64= req.get(&cont_len).unwrap().clone().as_i64().unwrap();
    println!("{}", tot.clone());
    Ok(tot)
}


async fn total(url: String, session: reqwest::Client, header: reqwest::header::HeaderMap) -> Result<i64, Box<dyn std::error::Error>>{
    let req = session.get(url)
        .headers(header)
        .send()
        .await?
        .json::<HashMap<String, serde_json::value::Value>>()
        .await?;
    println!("{:#?}", req);
    let cont_len: String = String::from("Content-Length");
    let tot: i64= req.get(&cont_len).unwrap().clone().as_i64().unwrap();
    println!("{}", tot.clone());
    Ok(tot)
}


async fn total(url: String, session: reqwest::Client, header: reqwest::header::HeaderMap) -> Result<i64, Box<dyn std::error::Error>>{
    let req = session.get(url)
        .headers(header)
        .send()
        .await?
        .json::<HashMap<String, serde_json::value::Value>>()
        .await?;
    println!("{:#?}", req);
    let cont_len: String = String::from("Content-Length");
    let tot: i64= req.get(&cont_len).unwrap().clone().as_i64().unwrap();
    println!("{}", tot.clone());
    Ok(tot)
}


async fn total(url: String, session: reqwest::Client, header: reqwest::header::HeaderMap) -> Result<i64, Box<dyn std::error::Error>>{
    let req = session.get(url)
        .headers(header)
        .send()
        .await?
        .json::<HashMap<String, serde_json::value::Value>>()
        .await?;
    println!("{:#?}", req);
    let cont_len: String = String::from("Content-Length");
    let tot: i64= req.get(&cont_len).unwrap().clone().as_i64().unwrap();
    println!("{}", tot.clone());
    Ok(tot)
}

#[tokio::main]
<<<<<<< Updated upstream
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let start = chrono::Utc::now();
    let client = Client::new();
    let user_agent = fake_useragent::UserAgent::new().await?;
    let ua = user_agent.random()?;
    let (tot, name) = total("https://httpbin.org/get", ua.clone(), client.clone()).await?;
    match tot{
        Some(t) => println!("{:#?} tot:{}", split(t, 10).await, t),
        None => ()
    }
    match name{
        Some(nm) => println!("{}", nm),
        None => ()
    }
    let resp = client_res!("https://httpbin.org/get", client, ua).text().await?;
    let end = chrono::Utc::now();
    println!("{}", resp);
    println!("{:#?}", (end-start).num_nanoseconds());
=======
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = String::from("https://mirrors.tuna.tsinghua.edu.cn/centos/7.9.2009/isos/x86_64/CentOS-7-x86_64-DVD-2009.iso");
    let client = reqwest::Client::new();
    let mut a_header = reqwest::header::HeaderMap::new();
    a_header.insert("User-Agent", get_rua().parse()?);
    a_header.insert("Content-Type", "application/json".parse()?);
    let file_tot = total(url, client, a_header).await?;
    println!("{:#?}", file_tot);
<<<<<<< Updated upstream
<<<<<<< Updated upstream
<<<<<<< Updated upstream
<<<<<<< Updated upstream
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes
    Ok(())
}
