#![allow(unused, dead_code)]
mod constant;
mod service;
mod utils;

#[macro_use]
extern crate log;

use mysql::prelude::*;
use service::account::register;
use std::{
    fmt::{Debug, Display},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    // env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // log4rs::init_file("./src/config/log4rs.yaml", Default::default()).unwrap();
    let db = utils::db::init(); // 初始化mysql
    println!("====");
    let db = utils::db::global();
    println!("db:{:?}",db);

    // let pool = db::global();
    let mut conn = db.get_conn().unwrap();
    let query = format!(
        "SELECT id,username FROM member_account WHERE id = '{}'",
        1
    );

    let res = conn.query_map(query, |(id, username)| UserInfo {
        id,
        username,
    }).unwrap();

    if res.len() != 0 {
        println!("success: {:?}",res);
    }


    let db1 = db;
    println!("db1:{:?}",db1);
    let mut conn1 = db1.get_conn().unwrap();
    let query = format!(
        "SELECT id,username FROM member_account WHERE id = '{}'",
        2
    );

    let res1 = conn1.query_map(query, |(id, username)| UserInfo {
        id,
        username,
    }).unwrap();

    if res.len() != 0 {
        println!("success1: {:?}",res1);
    }
}


#[derive(Debug)]
pub struct UserInfo {
    id:i64,
    username: String,
}
