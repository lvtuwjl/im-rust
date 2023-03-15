#![allow(unused, dead_code)]
mod constant;
mod service;
mod utils;

#[macro_use]
extern crate log;

use service::account::register;
use std::{
    fmt::{Debug, Display},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    // env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log4rs::init_file("./src/config/log4rs.yaml", Default::default()).unwrap();
    // utils::mysql1::query_rows(); // 查询多条数据
    println!("====");

    for i in 0..100 {
        let st = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("纳秒: {} {}", i, st.as_nanos());
        // println!("{:?}",SystemTime::now());
        // register::send_sms("".to_string(), "telephone".to_string());
    }
    // chrono::DateTime;
    println!("{}", "end..");

    // let st = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // println!("纳秒: {}",st.as_nanos());

    // let ts = st.as_secs();
    // let tmil = st.as_millis();
    // let tmic = st.as_micros();
    // let tn = st.as_nanos();
    // println!("{} {} {} {}",ts,tmil,tmic,tn);
}
