use std::env;
use anyhow::{anyhow, Ok};
use once_cell::sync::Lazy;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::info;
use crate::context::config::HttpDomainConf;
use crate::context::CTX;

static CLIENT: Lazy<ClientWithMiddleware> = Lazy::new(|| {
    ClientBuilder::new(reqwest::Client::default())
        .build()
});

pub struct MarketSrv;

impl MarketSrv {
    async fn do_get<R>(&self, params: &[(&str, &str)], url: &str) -> anyhow::Result<R>
        where
            R: std::fmt::Debug + DeserializeOwned,
    {
        let mc = &CTX.config.app.aliyun.market;

        let app_code = &mc.app_code;
        let app_code = app_code.as_ref()
            .map(|s| s.clone())
            .unwrap_or(env::var("APP_CODE")?);

        info!("send get to {},data is :{:?}", url, params);

        let rs = CLIENT
            .get(url)
            .query(params)
            .header("Authorization", format!("APPCODE {}", app_code))
            .send()
            .await?
            .json()
            .await?;
        info!("query result: {:?}", rs);
        Ok(rs)
    }

    pub async fn shumai(&self, card_no: String) -> anyhow::Result<ShumaiRes> {
        let mc = &CTX.config.app.aliyun.market;
        if let Some(HttpDomainConf { ref http_domain }) = mc.shumai {
            let params = &[("bankcard", card_no.as_str())];
            self.do_get(params, http_domain).await
        } else {
            Err(anyhow!("数脉未配置"))
        }
    }

    pub async fn lianzhuo(&self, card_no: String) -> anyhow::Result<LianzhuoRes> {
        let mc = &CTX.config.app.aliyun.market;
        if let Some(HttpDomainConf { ref http_domain }) = mc.lianzhuo {
            let params = &[("bankno", card_no.as_str())];
            self.do_get(params, http_domain).await
        } else {
            Err(anyhow!("连卓未配置"))
        }
    }
}

/*
{
    "msg": "成功",
    "success": true,
    "code": 200,
    "data": {
        "order_no": "676448654286000128",//订单号
        "bank": "中国建设银行",//银行名称
        "province": "广东省",//开户省
        "city": "潮州市",//开户市
        "card_name": "龙卡通",//卡名称
        "tel": "95533",//官方客服电话
        "type": "借记卡",//卡类型
        "logo": "http://pic.dataox.com.cn/jianshe.png",//银行logo
        "abbreviation": "CCB",//银联支付网关简码
        "card_bin": "621700",//银行卡bin码
        "bin_digits": 6,//银行卡bin码长度
        "card_digits": 19,//银行卡号长度
        "isLuhn": true,//是否支持luhn校验
        "weburl": "http://www.ccb.com/"//银行官方网站
    }
}
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct ShumaiRes {
    pub msg: String,
    pub success: bool,
    pub code: i32,
    pub data: Option<ShumaiData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShumaiData {
    pub order_no: Option<String>,
    pub bank: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub card_name: String,
    pub tel: Option<String>,
    #[serde(rename = "type")]
    pub card_type: String,
    pub logo: Option<String>,
    pub abbreviation: String,
    pub card_bin: String,
    pub bin_digits: i8,
    pub card_digits: Option<u8>,
    #[serde(rename = "isLuhn")]
    pub is_luhn: Option<bool>,
    pub weburl: Option<String>,
}

/*
{
    "data": {
        "area": "北京 - 北京",
        "bank_name": "中国银行",
        "bank_id": "01040000",//银行机构代码
        "bank_code": "104100000004",//银行总行的联行号，不是银行卡开户支行的联行号
        "bankno": "6217580100005980644",
        "bank_url": "http:\/\/www.boc.cn",
        "card_type": "借记卡",
        "card_name": "医保联名借记IC卡",
        "bank_phone": "95566",
        "bank_logo": "http:\/\/img.lundroid.com\/4d1f85.png"
    },
    "resp": {
        "code": 0,
        "desc": "OK"
    }
}
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct LianzhuoRes {
    pub data: Option<LianzhuoData>,
    pub resp: LianzhuoResp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LianzhuoData {
    pub area: Option<String>,
    pub bank_name: String,
    pub bank_id: Option<String>,
    pub bank_code: Option<String>,
    pub bankno: String,
    pub bank_url: Option<String>,
    pub card_type: Option<String>,
    pub card_name: Option<String>,
    pub bank_phone: Option<String>,
    pub bank_logo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LianzhuoResp {
    pub code: i32,
    pub desc: String,
}