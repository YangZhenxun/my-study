use std::collections::{HashMap, HashSet};
use log::{trace, warn};

use super::tools;
use super::*;

use super::tools::random_choice;

pub struct FakeUserAgent{
    pub browsers: Vec<String>,
    pub os: Vec<String>,
    pub min_version: f64,
    pub min_percentage: f64,
    pub platforms: Vec<String>,
    pub fallback: String,
    pub safe_attrs: HashSet<String>,
    pub data_browsers: Vec<HashMap<String, String>>
}

impl FakeUserAgent {
    pub async fn new() -> Result<FakeUserAgent, Box<dyn std::error::Error>> {
        let load = load().await?;
        Ok(FakeUserAgent {data_browsers:load, ..Default::default()})
    }
    pub fn _filter_useragents(&self, request: Option<String>) -> Vec<HashMap<String, String>>{
        let mut filtered_useragents = tools::filter(|x: &HashMap<String, String>|
            return tools::lin::lin(x["browser"].clone(), self.browsers.clone()) != None
                && tools::lin::lin(x["os"].clone(), self.os.clone()) != None
                && tools::lin::lin(x["type"].clone(), self.platforms.clone()) != None
                && x["version"].parse::<f64>().unwrap() >= self.min_version
                && x["percent"].parse::<f64>().unwrap() >= self.min_percentage,
        self.data_browsers.clone());
        match request {
            Some(req) => filtered_useragents = tools::filter(|x: &HashMap<String, String>|x["browser"] == req, filtered_useragents),
            None => return filtered_useragents
        }
        filtered_useragents
    }
    pub fn getBrowser(&self, mut request: String) -> HashMap<String, String> {
        for (value, replacement) in settings::REPLACEMENTS.clone().into_iter(){
            request = request.replace(&value, &replacement);
        }
        request = request.to_lowercase();
        match settings::SHORTCUTS.get(&request) {
            Some(val) => request = val.clone(),
            None => request = request
        }
        if request == "random".to_string(){
            return tools::random_choice::<HashMap<String, String>>(self._filter_useragents(None));
        } else {
            return tools::random_choice::<HashMap<String, String>>(self._filter_useragents(Some(request)));
        }
    }
    pub fn __getattr__(&self, mut attr: String) -> Result<String, Box<dyn std::error::Error>>{
        trace!("Starting `fake_useragent::FakeUserAgent.__getattr__()` method...");
        match self.safe_attrs.get(&attr){
            Some(att) => Ok(att.to_string()),
            None => {
                for (value, replacement) in settings::REPLACEMENTS.clone().into_iter(){
                    attr = attr.replace(&value, &replacement);
                }
                attr = attr.to_lowercase();
                match settings::SHORTCUTS.get(&attr) {
                    Some(val) => attr = val.clone(),
                    None => attr = attr
                }
                if attr == "random".to_string(){
                    match random_choice(self._filter_useragents(None)).get("useragent") {
                        Some(val) => Ok(val.clone()),
                        None => {
                            if self.fallback.is_empty(){
                                return Err(Box::new(FakeUserAgentError))?;
                            } else {
                                warn!("Error occurred during getting browser: {attr}, 
                                but was suppressed with fallback.");
                                Ok(self.fallback.clone())
                            }
                        }
                    }
                } else {
                    match random_choice(self._filter_useragents(Some(attr.clone()))).get("useragent") {
                        Some(val) => Ok(val.clone()),
                        None => {
                            if self.fallback.is_empty(){
                                return Err(Box::new(FakeUserAgentError))?;
                            } else {
                                warn!("Error occurred during getting browser: {attr}, 
                                but was suppressed with fallback.");
                                Ok(self.fallback.clone())
                            }
                        }
                    }
                }
            }
        }
    }
    #[inline]
    pub fn chrome(&self) -> Result<String, Box<dyn std::error::Error>>{
        let res = self.__getattr__("chrome".to_string())?;
        Ok(res)
    }
    #[inline]
    pub fn googlechrome(&self) -> Result<String, Box<dyn std::error::Error>>{
        let res = self.chrome()?;
        Ok(res)
    }
    #[inline]
    pub fn edge(&self) -> Result<String, Box<dyn std::error::Error>>{
        let res = self.__getattr__("edge".to_string())?;
        Ok(res)
    }
    #[inline]
    pub fn firefox(&self) -> Result<String, Box<dyn std::error::Error>>{
        let res = self.__getattr__("firefox".to_string())?;
        Ok(res)
    }
    #[inline]
    pub fn ff(&self) -> Result<String, Box<dyn std::error::Error>>{
        let res = self.firefox()?;
        Ok(res)
    }
    #[inline]
    pub fn safari(&self) -> Result<String, Box<dyn std::error::Error>>{
        let res = self.__getattr__("safari".to_string())?;
        Ok(res)
    }
    #[inline]
    pub fn random(&self) -> Result<String, Box<dyn std::error::Error>>{
        let res = self.__getattr__("random".to_string())?;
        Ok(res)
    }
    #[inline]
    pub fn getFirefox(&self) -> HashMap<String, String>{
        self.getBrowser("firefox".to_string())
    }
    #[inline]
    pub fn getChrome(&self) -> HashMap<String, String>{
        self.getBrowser("chrome".to_string())
    }
    #[inline]
    pub fn getEdge(&self) -> HashMap<String, String>{
        self.getBrowser("edge".to_string())
    }
    #[inline]
    pub fn getSafari(&self) -> HashMap<String, String>{
        self.getBrowser("safari".to_string())
    }
    #[inline]
    pub fn getRandom(&self) -> HashMap<String, String>{
        self.getBrowser("random".to_string())
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
            fallback:
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ".to_string() +
                "AppleWebKit/537.36 (KHTML, like Gecko) " +
                "Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0",
            safe_attrs: vec![].into_iter().collect(),
            data_browsers: vec![]
        }
    }
}

pub type UserAgent = FakeUserAgent;