#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

use chrono::Duration;
use once_cell::sync::Lazy;

pub static bufferSize: i64 = 1024;
pub static defaultURL: Lazy<String> = Lazy::new(||String::from("https://cf.xiu2.xyz/url"));
pub static defaultTimeout: Lazy<Duration> = Lazy::new(||Duration::seconds(10));
pub static defaultDisableDownload: bool = false;
pub static defaultTestNum: i64 = 10;
pub static defaultMinSpeed: f64 = 0.0;

pub static mut URL: Lazy<String> = Lazy::new(||defaultURL.clone());
pub static mut Timeout: Lazy<Duration> = Lazy::new(||defaultTimeout.clone());
pub static mut Disable: bool = defaultDisableDownload;

pub static mut TestCount: i64 = defaultTestNum;
pub static mut MinSpeed: f64 = defaultMinSpeed;

pub fn checkDownloadDefault() {
    unsafe {
        if URL.clone().is_empty(){
            URL = Lazy::new(||defaultURL.clone());
        }
        if Timeout.clone() <= Duration::seconds(0) {
            Timeout = Lazy::new(||defaultTimeout.clone());
        }
        if TestCount.clone() <= 0 {
            TestCount = defaultTestNum;
        }
        if MinSpeed.clone() <= 0.0 {
            MinSpeed = defaultMinSpeed;
        }
    }
}
