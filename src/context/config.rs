use anyhow::{Ok, Result};
use serde::Deserialize;

pub(super) fn init() -> Result<Config> {
    use std::{fs, env};
    let conf = match env::var("MARKET_CONFIG_PATH") {
        Result::Ok(path) => {
            let config_data = fs::read_to_string(path)?;
            serde_yaml::from_str::<Config>(config_data.as_str())
        }
        Err(_) => {
            let config_data = include_str!("../../config/application.yaml");
            serde_yaml::from_str::<Config>(config_data)
        }
    };

    Ok(conf?)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub app: AppConf,
    pub mysql: MysqlConf,
    pub server: ServerConf,
    pub log: LogConf,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LogConf {
    pub file_log: Option<FileLogConf>,
    pub console_log: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct FileLogConf {
    pub enable: bool,
    pub log_path: String,
    pub log_prefix: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConf {
    pub aliyun: AliyunConf,
    pub white_list: Option<WhiteListConf>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WhiteListConf {
    pub enable: Option<bool>,
    pub ip_white_list: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct AliyunConf {
    pub market: Market,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Market {
    pub app_code: Option<String>,
    pub shumai: Option<HttpDomainConf>,
    pub lianzhuo: Option<HttpDomainConf>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct HttpDomainConf {
    pub http_domain: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct MysqlConf {
    pub max_connections: Option<u32>,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerConf {
    pub port: u16,
    pub addr: String,
}
