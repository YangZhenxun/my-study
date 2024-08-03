use std::{collections::HashMap, env::current_exe, error::Error, fs::{File, create_dir}, io::Read, path::{Path, PathBuf}};
use log::{info, trace, warn};
use crate::fake_useragent::*;

pub fn load() -> Result<Vec<HashMap<String, String>>, Box<dyn Error>>{
    trace!("Using function `load`.");
    let mut data: Vec<HashMap<String, String>> = Vec::new();
    let mut ret: Option<Vec<HashMap<String, String>>> = None;
    let mut json_lines = String::new();
    match current_exe()?.parent() {
        Some(var) => {
            match var.to_str() {
                Some(var) => {
                    let va = var.to_string()+"/data/browsers.json";
                    let val = Path::new(&va);
                    println!("{}",val.display());
                    if val.exists(){
                        let _ = File::open(val)?.read_to_string(&mut json_lines)?;
                        for line in json_lines.lines(){
                            data.push(serde_json::from_str(line)?)
                        }
                        ret = Option::Some(data);
                    } else {
                        warn!("`data/browsers.json` does not exist.");
                        let va2 = var.to_string()+"/data";
                        let val2 = Path::new(&va2);
                        if val2.exists(){
                            let _ = File::create(val)?;
                        } else {
                            create_dir(val2)?;
                            let _ = File::create(val).expect("m");
                        }
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