#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

use chrono::Duration;
use once_cell::sync::Lazy;
use std::net;

pub static defaultOutput: Lazy<String> = Lazy::new(||String::from("result.csv"));
pub static maxDelay: Lazy<Duration> = Lazy::new(||Duration::milliseconds(9999));
pub static minDelay: Lazy<Duration> = Lazy::new(||Duration::milliseconds(0));
pub static maxLossRate: f32 = 1.0;

pub static mut InputMaxDelay: Lazy<Duration> = Lazy::new(||maxDelay.clone());
pub static mut InputMinDelay: Lazy<Duration> = Lazy::new(||minDelay.clone());
pub static mut InputMaxLossRate: f32 = maxLossRate;
pub static mut Output: Lazy<String> = Lazy::new(||defaultOutput.clone());
pub static mut PrintNum: i64 = 10;

pub fn NoPrintResult() -> bool {
    unsafe {
        return PrintNum==0;
    }
}

pub fn noOutput() -> bool {
    unsafe {
        return *Output == "".to_string() || *Output == "".to_string();
    }
}

pub struct PingData {
    pub IP: net::IpAddr,
    pub Sended: i64,
    pub Received: i64,
    pub Delay: Duration,
}

pub struct CloudflareIPData {
    pub unused: PingData,
    pub lossRate: f32,
    pub DownloadSpeed: f64,
}

impl CloudflareIPData{
    pub fn GetLossRate(&mut self) -> f32 {
        if self.lossRate == 0.0{
            let pingLost = self.unused.Sended - self.unused.Received;
            self.lossRate = pingLost as f32 / self.unused.Sended as f32;
        }
        return self.lossRate;
    }
    
}