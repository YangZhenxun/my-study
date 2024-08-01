use std::collections::{HashMap, HashSet};
use crate::fake_useragent::tools;
use crate::fake_useragent::*;

pub struct FakeUserAgent{
    pub browsers: Vec<String>,
    pub os: Vec<String>,
    pub min_version: f64,
    pub min_percentage: f64,
    pub platforms: Vec<String>,
    pub fallback: Vec<String>,
    pub safe_attrs: HashSet<()>,
    pub data_browsers: Vec<HashMap<String, String>>
}

impl FakeUserAgent {
    pub fn new() -> Result<FakeUserAgent, Box<dyn std::error::Error>> {
        let load = load()?;
        Ok(FakeUserAgent {data_browsers:load, ..Default::default()})
    }
    pub fn _filter_useragents(&self, request: Option<String>) -> Vec<HashMap<String, String>>{
        let mut filtered_useragents = tools::filter(|x: &HashMap<String, String>|
            return tools::lin(x["browser"].clone(), self.browsers.clone()) != None
                && tools::lin(x["os"].clone(), self.os.clone()) != None
                && tools::lin(x["type"].clone(), self.platforms.clone()) != None
                && x["version"].parse::<f64>().unwrap() >= self.min_version
                && x["percent"].parse::<f64>().unwrap() >= self.min_percentage,
        self.data_browsers.clone());
        match request {
            Some(req) => filtered_useragents = tools::filter(|x: &HashMap<String, String>|x["browser"] == req, filtered_useragents),
            None => return filtered_useragents
        }
        return filtered_useragents;
    }
}

impl Default for FakeUserAgent {
    fn default() -> Self {
        Self {
            browsers: vec![
                "chrome".to_string(), 
                "edge".to_string(), 
                "firefox".to_string(), 
                "safari".to_string()
                ],
            os : vec![
                "windows".to_string(), 
                "macos".to_string(), 
                "linux".to_string(),
                "ios".to_string(),
                "android".to_string(),
                ],
            min_version: 0.0,
            min_percentage: 0.0,
            platforms: vec![
                "pc".to_string(), 
                "mobile".to_string(),
                "tablet".to_string()
            ],
            fallback: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ".to_string(),
                "AppleWebKit/537.36 (KHTML, like Gecko) ".to_string(),
                "Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0".to_string()
            ],
            safe_attrs: vec![].into_iter().collect(),
            data_browsers: vec![]
        }
    }
}