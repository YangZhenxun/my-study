use std::collections::{HashMap, HashSet};

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
        let load = crate::utils::load()?;
        Ok(FakeUserAgent {data_browsers:load, ..Default::default()})
    }
    pub fn _filter_useragents(&self, request: Option<HashMap<String, String>>){
        let mut filtered_useragents = crate::filter::filter(|x: &HashMap<String, String>|
            return crate::lin::lin(*x["browser"], self.browsers) != None
                && crate::lin::lin(*x["os"], self.os) != None
                && crate::lin::lin(*x["type"], self.platforms) != None
                && *x["version"] >= self.min_version
                && *x["percent"] >= self.min_percentage,
        self.data_browsers);
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