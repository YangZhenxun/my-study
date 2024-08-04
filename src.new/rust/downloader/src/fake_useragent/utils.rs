use std::{collections::HashMap, env::current_exe, error::Error, path::Path};
use log::{trace, warn};
use crate::fake_useragent::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn load() -> Result<Vec<HashMap<String, String>>, Box<dyn Error>>{
    trace!("Using `downloader::fake_useragent::utils::load`.");
    let mut data: Vec<HashMap<String, String>> = Vec::new();
    let mut ret: Option<Vec<HashMap<String, String>>> = None;
    let mut json_lines = String::new();
    match current_exe()?.parent() {
        Some(var) => {
            match var.to_str() {
                Some(var) => {
                    let va = var.to_string()+"/data/browsers.json";
                    let val = Path::new(&va);
                    if val.exists(){
                        let _ = File::open(val).await?.read_to_string(&mut json_lines).await?;
                        for line in json_lines.lines(){
                            data.push(serde_json::from_str(line)?)
                        }
                        ret = Option::Some(data);
                    }
                },
                None => ()
            }
        },
        None => warn!("Could not find local data/json file or could not parse the contents.")
    }
    if ret == None {
        Err(Box::new(error::FakeUserAgentError))?;
    }
    match ret {
        Some(val) => Ok(val),
        None => Ok(Vec::new())
    }
}