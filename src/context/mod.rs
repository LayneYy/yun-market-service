use once_cell::sync::Lazy;
use rbatis::Rbatis;
use config::{Config, init};
use crate::service::bank_issue_srv::BankIssueSrv;

pub mod config;
pub mod database;

pub static CTX: Lazy<ApplicationContext> = Lazy::new(|| ApplicationContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::context::CTX.rb.clone()
    };
}

pub struct ApplicationContext {
    pub config: Config,
    pub rb: Rbatis,
    pub bank_issue_srv: BankIssueSrv,
}

impl Default for ApplicationContext {
    fn default() -> Self {
        let config = init().expect("read config fail!");
        Self {
            rb: database::init(&config.mysql).expect("init database fail!"),
            config,
            bank_issue_srv: BankIssueSrv,
        }
    }
}
