#[derive(Debug)]
pub struct BindPhoneRequest {
    member_id: i64,
    area_code: String,
    phone: String,
}

impl BindPhoneRequest {
    // verify phone
    fn verify_phone(&self) -> Result<i32, i32> {
        if self.area_code != "86" && self.phone.len() != 11 {
            let iii = 88;
            return Err(-1);
        }

        Ok(0)
    }
}

pub struct UserLoginRequest {
    pub user_name: String,
    pub password: String,
    pub sms_code: String,
}
