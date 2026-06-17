//! 应用层错误：定义统一的业务错误类型，供各应用服务返回。
//!
//! 该类型与具体的 Web 框架解耦（不依赖 axum），仅描述错误语义与对应的状态码；
//! 由 `interfaces::errors` 负责将其适配为 HTTP 响应。

use std::fmt;

/// 应用统一错误类型。
#[derive(Debug)]
pub enum AppError {
    /// 请求参数错误（400）。
    BadRequest(String),
    /// 未授权（401）。
    Unauthorized(String),
    /// 资源不存在（404）。
    NotFound(String),
    /// 服务器内部错误（500）。
    Internal(String),
}

impl AppError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// 对应的业务状态码（与 HTTP 状态码语义保持一致）。
    pub fn status_code(&self) -> u16 {
        match self {
            Self::BadRequest(_) => 400,
            Self::Unauthorized(_) => 401,
            Self::NotFound(_) => 404,
            Self::Internal(_) => 500,
        }
    }

    /// 对外暴露的提示信息。
    pub fn message(&self) -> &str {
        match self {
            Self::BadRequest(m)
            | Self::Unauthorized(m)
            | Self::NotFound(m)
            | Self::Internal(m) => m,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status_code(), self.message())
    }
}

impl std::error::Error for AppError {}

/// 将任意 `anyhow::Error` 归类为内部错误，方便 `?` 传播。
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

/// 应用层统一返回类型。
pub type AppResult<T> = Result<T, AppError>;
