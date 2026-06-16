use serde::Serialize;

/// 统一 API 响应结构。
///
/// `code` 为业务状态码（与 HTTP 状态码语义保持一致），`msg` 为提示信息，`data` 为业务数据。
#[derive(Debug, Clone, Serialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> Response<T> {
    /// 成功响应，固定 code 为 200。
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data,
        }
    }

    /// 自定义 code 与提示信息的响应（用于失败或特定业务码）。
    pub fn with(code: i32, msg: impl Into<String>, data: T) -> Self {
        Self {
            code,
            msg: msg.into(),
            data,
        }
    }
}
