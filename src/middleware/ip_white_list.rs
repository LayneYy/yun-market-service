use anyhow::anyhow;
use axum::{
    Extension,
    extract::{ConnectInfo, FromRequestParts},
    http::request::Parts,
};
use std::net::SocketAddr;
use tracing::{error, info};

use async_trait::async_trait;
use crate::context::CTX;

use crate::domain::vo::response::ApiResponse;


pub struct IpWhiteList;

#[async_trait]
impl<S> FromRequestParts<S> for IpWhiteList
    where
        S: Send + Sync,
{
    type Rejection = ApiResponse<()>;

    /// Perform the extraction.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match &CTX.config.app.white_list {
            Some(ip_conf) if ip_conf.enable.unwrap_or(false) => {
                let return_fn = |remote: &str| {
                    let ips = &ip_conf.ip_white_list;
                    let is_white_list = ips.into_iter().any(|ip| remote == ip.as_str());
                    if is_white_list {
                        Ok(Self)
                    } else {
                        error!("{} was rejected", remote);
                        let res = ApiResponse::<()>::fail_with_msg("reject");
                        Err(res)
                    }
                };
                let headers = &parts.headers;
                let x_real_ip_key = "X-Real-IP";
                if headers.contains_key(x_real_ip_key) {
                    let remote = headers.get(x_real_ip_key).unwrap();
                    if let Ok(remote) = remote.to_str() {
                        info!("{}:{}",x_real_ip_key, remote);
                        return return_fn(remote);
                    }
                }

                match Extension::<ConnectInfo<SocketAddr>>::from_request_parts(parts, state)
                    .await
                    .map_err(|_e| anyhow!("can't extract addr"))
                {
                    Ok(Extension(connect_info)) => {
                        let remote = match &connect_info.0 {
                            SocketAddr::V4(v4) => v4.ip().to_string(),
                            SocketAddr::V6(v6) => v6.ip().to_string(),
                        };
                        info!("connect_info:{:?}",connect_info);
                        return_fn(remote.as_str())
                    }
                    Err(e) => {
                        let res = ApiResponse::<()>::fail_with_msg(format!("can't extract addr:{:?}", e));
                        Err(res)
                    }
                }
            }
            _ => Ok(Self),
        }
    }
}
