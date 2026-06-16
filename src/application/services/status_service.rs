use crate::domain::models::health_status::HealthStatus;

/// 状态服务：聚合各组件的健康检查结果。
///
/// 目前仅检查 API 服务自身，后续接入 postgres / redis 等检查器时，
/// 在此注入对应的 `domain::external` 检查器并行检查。
#[derive(Debug, Clone, Default)]
pub struct StatusService;

impl StatusService {
    pub fn new() -> Self {
        Self
    }

    /// 检查所有组件并返回健康状态列表。
    pub async fn check_all(&self) -> Vec<HealthStatus> {
        vec![HealthStatus::ok("api")]
    }
}
