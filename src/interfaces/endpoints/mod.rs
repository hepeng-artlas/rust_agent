//! 路由与处理器：HTTP 端点定义。

pub mod session;
pub mod status;

use axum::Router;

use crate::state::AppState;

/// 创建应用的全部 API 路由，统一挂载在 `/api` 前缀下。
pub fn create_router() -> Router<AppState> {
    Router::new().nest("/api", status::router().merge(session::router()))
}
