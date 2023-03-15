use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*;
use once_cell::sync::OnceCell;

// lazy_static! {
//     pub static ref MY_DB: Pool = init_mysql().unwrap();
// }
static MYSQLDB: OnceCell<Pool> = OnceCell::new();

pub fn global() -> &'static Pool {
    MYSQLDB.get().expect("init mysql pool")
}

// 启动的时候加载
fn init() {
    MYSQLDB.set(init_mysql().unwrap());
}
// 初始化数据库连接
pub fn init_mysql() -> Result<Pool> {
    let url = "mysql://test11:12345678@localhost:3306/rust11";
    let url = Opts::from_url(url).expect("url 转化出错!");
    let pool = Pool::new(url).expect("Pool 1122!");

    Ok(pool)
}
