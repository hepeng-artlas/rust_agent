//! 异常处理：将应用层 `AppError` 适配为统一的 HTTP 响应结构。
//!
//! 借助 axum 的 `IntoResponse`，任意 handler 只要返回 `Result<_, AppError>`，
//! 出错时即可自动序列化为统一的 `Response`（code / msg / data）。

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response as HttpResponse};

use crate::application::errors::AppError;
use crate::interfaces::schemas::response::Response;

impl IntoResponse for AppError {
    fn into_response(self) -> HttpResponse {
        let code = self.status_code();
        let message = self.message().to_string();

        tracing::error!(code, error = %self, "请求处理失败");

        let status = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body: Response<()> = Response::with(i32::from(code), message, ());

        (status, Json(body)).into_response()
    }
}
