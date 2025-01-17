use rayon::prelude::*;

pub async fn place() -> Result<String, Box<dyn std::error::Error>>{
    match std::env::current_dir()?.to_str(){
        Some(path) => {
            let a: Vec<&str> = path.par_split('/').collect();
            let mut b = a[0].to_string();
            println!("{}, {:#?}", b, a);
            for i in &a[1..=(a.len()-3)]{
                b += ("/".to_string() + *i).as_str();
            }
            Ok(b+"/"+"Downloaded files/")
        },
        None => {
            println!("Downloaded files");
            Ok("Downloaded files/".to_string())
        }
    }
}
pub fn nplace() -> Result<String, Box<dyn std::error::Error>>{
    match std::env::current_dir()?.to_str(){
        Some(path) => {
            let a: Vec<&str> = path.par_split('/').collect();
            let mut b = a[0].to_string();
            println!("{}, {:#?}", b, a);
            for i in &a[1..=(a.len()-3)]{
                b += ("/".to_string() + *i).as_str();
            }
            Ok(b+"/"+"Downloaded files/")
        },
        None => {
            println!("Downloaded files");
            Ok("Downloaded files/".to_string())
        }
    }
}