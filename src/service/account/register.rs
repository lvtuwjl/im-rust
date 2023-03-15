use super::request::*;
use super::request::*;
// use crate::constant::errors::StatusCode;
use crate::utils::db;
use crate::utils::errors::{Error, Result};
use crate::utils::server_error::ServerError;
use anyhow::anyhow;
use mysql::prelude::*;
use mysql::*;
use rand::Rng;

#[derive(Debug)]
pub struct UserInfo {
    name: String,
    password: String,
    avator: String,
}

impl UserInfo {
    pub fn new(name: String, password: String, avator: String) -> Self {
        UserInfo {
            name,
            password,
            avator,
        }
    }
    // }

    // 用户注册
    pub fn register(&self, name: String, password: String, avator: String) -> Result<i32> {
        // 检查数据格式是否正确
        // is_verify()
        // todo!
        // 已经注册过
        if self.is_register(name, password, avator)? {
            return Ok(1004);
        }

        // 插入用户数据
        // todo!

        Ok(0)
    }

    // 是否已经注册过
    fn is_register(&self, name: String, password: String, avator: String) -> Result<bool> {
        // 查询数据库
        let pool = db::global();
        let mut conn = pool.get_conn()?;
        let query = format!(
            "SELECT name,password,avator FROM user_info WHERE name = '{}'",
            name
        );

        let res = conn.query_map(query, |(name, password, avator)| UserInfo {
            name,
            password,
            avator,
        })?;

        if res.len() != 0 {
            return Ok(true);
        }

        Ok(false)
    }

    // 绑定手机号
    fn bind_phone(&self, bpr: BindPhoneRequest) {
        println!("{:?}", bpr);
        // 校验手机号
        // if bpr.ph
    }

    // 绑定邮箱
    fn bind_email(&self) {}

    // 用户登陆
    fn user_login(&self, req: UserLoginRequest) {
        // println!("{}",req.name)
        if req.user_name != "" && req.password != "" {
            //  账户密码登陆
        } else if req.sms_code.len() > 0 {
            // 手机号登陆
        }

        // 用户存在否？
    }
}

pub fn send_sms(mut country_code: String, telephone: String) -> Result<String> {
    if country_code.len() == 0 {
        country_code = String::from("86")
    }

    if country_code != "86" || telephone.len() != 11 {
        let err = anyhow!(ServerError::new(101, String::from("手机号格式不正确")));
        info!("{}", err);

        return Err(err);
    }

    Ok(build_sms_code()?)
}

fn build_sms_code() -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::<u8>::with_capacity(6);

    for _ in 0..v.capacity() {
        v.push(rng.gen_range('0' as u8, '9' as u8));
    }
    println!("sms code random data: {}", String::from_utf8(v.clone())?);

    Ok(String::from_utf8(v)?)
}

#[test]
fn test_send_sms() {
    println!("{}", 666);
    send_sms("".to_string(), "telephone".to_string());
}
