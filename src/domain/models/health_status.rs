use serde::Serialize;

/// 健康检查状态。
#[derive(Debug, Clone, Serialize)]
pub struct HealthStatus {
    /// 检查对应的服务名。
    pub service: String,
    /// 检查结果：`ok` 表示正常，`error` 表示异常。
    pub status: String,
    /// 异常时的详情提示。
    pub details: String,
}

impl HealthStatus {
    /// 构造一个正常状态。
    pub fn ok(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            status: "ok".to_string(),
            details: String::new(),
        }
    }

    /// 构造一个异常状态。
    pub fn error(service: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            status: "error".to_string(),
            details: details.into(),
        }
    }

    /// 是否为异常状态。
    pub fn is_error(&self) -> bool {
        self.status == "error"
    }
}
