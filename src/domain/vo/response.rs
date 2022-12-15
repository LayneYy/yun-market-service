use super::status::ApiStatusCode;
use anyhow::Result;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use tracing::{error, trace};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: ApiStatusCode,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: Option<T>) -> Self {
        Self::full(ApiStatusCode::Success, "SUCCESS", data)
    }

    #[allow(dead_code)]
    pub fn success_no_data() -> Self {
        Self::success(None)
    }

    pub fn full(code: ApiStatusCode, msg: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code,
            message: msg.into(),
            data,
        }
    }

    #[allow(dead_code)]
    pub fn with_out_data(code: ApiStatusCode, msg: impl Into<String>) -> Self {
        Self::full(code, msg, None)
    }

    #[allow(dead_code)]
    pub fn fail() -> Self {
        Self::full(ApiStatusCode::ServiceException, "service exception", None)
    }

    pub fn fail_with_msg(msg: impl Into<String>) -> Self {
        Self::full(ApiStatusCode::ServiceException, msg, None)
    }
}

impl<T: Serialize> From<Result<T>> for ApiResponse<T> {
    fn from(r: Result<T>) -> Self {
        match r {
            Ok(t) => Self::success(Some(t)),
            Err(e) => {
                error!("{}",e);
                trace!("{:?}",e);
                Self::fail_with_msg(e.to_string())
            }
        }
    }
}