use anyhow::{Ok, Result};
use rbatis::Rbatis;
use rbdc_mysql;
use rbdc_mysql::{
    driver::MysqlDriver,
    options::MySqlConnectOptions,
};

use crate::context::config::MysqlConf;

pub(super) fn init(c: &MysqlConf) -> Result<Rbatis> {
    let rb = Rbatis::new();

    let opt = MySqlConnectOptions::new()
        .host(&c.host)
        .port(c.port)
        .username(&c.username)
        .password(&c.password)
        .database(&c.database);
    rb.init_opt(MysqlDriver {}, opt)?;
    Ok(rb)
}
