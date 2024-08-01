use std::{collections::HashMap, env::current_exe, error::Error, fs::File, io::Read};
use log::{trace, warn};
use crate::fake_useragent::*;

pub fn load() -> Result<Vec<HashMap<String, String>>, Box<dyn Error>>{
    trace!("Using function `load`.");
    let mut data: Vec<HashMap<String, String>> = Vec::new();
    let mut ret: Option<Vec<HashMap<String, String>>> = None;
    let mut json_lines = String::new();
    match current_exe()?.parent() {
        Some(var) => {
            if var.exists(){
                let _ = File::open(var)?.read_to_string(&mut json_lines)?;
                for line in json_lines.lines(){
                    data.push(serde_json::from_str(line)?)
                }
                ret = Option::Some(data);
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