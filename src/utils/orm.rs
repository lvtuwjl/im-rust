use std::collections::HashMap;

use super::db;
use crate::service::account::user::UserInfo;
use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*;

// lazy_static! {
//     static ref MY_DB: Pool = init_mysql().unwrap();
// }

//  test..
// pub fn t7() {
//     let h11 = init_mysql().unwrap();
//     let h22 = &h11;
//     let h33 = &h11;
//     // let h22 = &h33;

//     println!("h123: {:?} {:?} {:?}", h11, h22, h33);
// }

// pub fn t8() {
//     let ref a89 = 88;
//     // let h9 = a89;
//     let m1 = &MY_DB;
//     let m2 = &MY_DB;
//     println!("H2:  {} {:?} {:?} {:?}", a89, *MY_DB, **m1, **m2);
// }

// 初始化数据库连接
// pub fn init_mysql() -> Result<Pool> {
//     let url = "mysql://test11:12345678@localhost:3306/rust11";
//     let url = Opts::from_url(url).expect("url 转化出错!");
//     let pool = Pool::new(url).expect("Pool 1122!");

//     Ok(pool)
// }

// #[test]
// 插入一条数据
pub fn insert_row() {
    // let pool = init_mysql().unwrap();
    let pool = db::global();
    println!("333");
    let mut conn = pool.get_conn().unwrap();
    println!("8888");
    // let py = vec![UserInfo {
    //     id: 0,
    //     name: "test4".to_string(),
    //     password: "123456".to_string(),
    //     created_at: "".to_string(),
    //     updated_at: "".to_string(),
    // }];

    // conn.exec_batch(
    //     r"INSERT INTO user_info (name,password) VALUES(:name,:password)",
    //     py.iter().map(|p| {
    //         params! {
    //             "name" => p.name.clone(),
    //             "password" => p.password.clone(),
    //         }
    //     }),
    // )
    // .unwrap();
    conn.exec_drop(
        r"INSERT INTO user_info (name,password,avatar,created_at) VALUES(:name,:password,:avatar,:created_at)",
        params! {
             "name" => "test12",
            "password" => "123456".to_string(),
            "avatar" => "",
            "created_at" => "2022-05-22 11:02:04",
        },
    )
    .unwrap();
    println!("{}", conn.last_insert_id());

    println!("end..");
}

// 查询多条数据到struct中
pub fn query_rows() {
    // let pool = init_mysql().unwrap();
    let pool = db::global();

    println!("1222");

    let mut conn = pool.get_conn().unwrap();

    println!("777");
    let res = conn
        .query_map(
            "select id,name,password,created_at,updated_at from user_info where name = 'test1'",
            |(id, name, password, created_at, updated_at)| UserInfo {
                id,
                name,
                password,
                created_at,
                updated_at,
            },
        )
        .expect("Query failed.");
    println!("233: {}", res.len());

    for i in res {
        let y1 = i.updated_at;
        let fmt = "%Y-%m-%d %H:%M:%S";
        let y1 = NaiveDateTime::parse_from_str(y1.as_str(), fmt).unwrap();

        println!("{} {} {} {} {}", i.id, i.name, i.password, i.created_at, y1);
        println!(
            "{:#?} {:#?} {} {}",
            y1.date(),
            y1.time(),
            y1.timestamp(),
            y1.timestamp_millis()
        )
    }
}

// struct UserInfo {
//     id: u64,
//     name: String,
//     password: String,
//     created_at: String,
//     updated_at: String,
// }
