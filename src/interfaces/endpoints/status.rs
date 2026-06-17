use axum::extract::State;
use axum::{Json, Router, routing::get};

use crate::domain::models::health_status::HealthStatus;
use crate::interfaces::schemas::response::Response;
use crate::state::AppState;

/// 状态模块路由。
pub fn router() -> Router<AppState> {
    Router::new().route("/status", get(get_status))
}

/// 系统健康检查：检查 API 及各依赖组件的运行状态。
async fn get_status(State(state): State<AppState>) -> Json<Response<Vec<HealthStatus>>> {
    let statuses = state.status_service.check_all().await;

    if statuses.iter().any(HealthStatus::is_error) {
        Json(Response::with(503, "系统存在服务异常", statuses))
    } else {
        Json(Response::success(statuses))
    }
}
