use std::fmt::Display;

use crate::config;
use salvo::{
    async_trait, http::StatusCode, prelude::Json, Depot, Request,
    Response, Writer,
};
use serde::Serialize;

pub struct AppResponse<T: Serialize> {
    pub status_code: salvo::prelude::StatusCode,
    pub content: T,
}

impl<T: Serialize> AppResponse<T> {
    pub fn ok(content: impl Into<T>) -> AppResponse<T> {
        AppResponse {
            status_code: StatusCode::OK,
            content: content.into(),
        }
    }

    pub fn bad_request(content: impl Into<T>) -> AppResponse<T> {
        AppResponse {
            status_code: StatusCode::BAD_REQUEST,
            content: content.into(),
        }
    }

    pub fn internal_server_error(content: impl Into<T>) -> AppResponse<T> {
        AppResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            content: content.into(),
        }
    }
}

#[async_trait]
impl<T: Serialize + Send> Writer for AppResponse<T> {
    async fn write(
        mut self,
        _req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
    ) {
        res.status_code(self.status_code);
        res.render(Json(self.content));
    }
}

pub struct AppUncaughtErrResponse {
    pub description: String,
}

#[async_trait]
impl Writer for AppUncaughtErrResponse {
    async fn write(
        mut self,
        _req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
    ) {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(self.description));
    }
}

impl<T: Display> From<T> for AppUncaughtErrResponse {
    fn from(value: T) -> Self {
        AppUncaughtErrResponse {
            description: if config::log::EXPOSE_ERR {
                format!("未处理的服务器错误：{}", value)
            } else {
                "未处理的服务器错误".to_string()
            },
        }
    }
}

pub type AppResult<T> = Result<AppResponse<T>, AppUncaughtErrResponse>;
