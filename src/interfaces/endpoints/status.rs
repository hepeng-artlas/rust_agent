use axum::{Json, Router, routing::get};

use crate::application::services::status_service::StatusService;
use crate::domain::models::health_status::HealthStatus;
use crate::interfaces::schemas::response::Response;

/// 状态模块路由。
pub fn router() -> Router {
    Router::new().route("/status", get(get_status))
}

/// 系统健康检查：检查 API 及各依赖组件的运行状态。
async fn get_status() -> Json<Response<Vec<HealthStatus>>> {
    let statuses = StatusService::new().check_all().await;

    if statuses.iter().any(HealthStatus::is_error) {
        Json(Response::with(503, "系统存在服务异常", statuses))
    } else {
        Json(Response::success(statuses))
    }
}
