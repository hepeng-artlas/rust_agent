//! 应用共享状态（依赖注入容器 / 组合根）。


use std::sync::Arc;

use crate::application::services::agent_service::AgentService;
use crate::application::services::status_service::StatusService;
use crate::core::config::Settings;

#[derive(Clone)]
pub struct AppState {
    /// 全局配置（只读，多请求共享）。
    pub settings: Arc<Settings>,
    /// 状态服务。
    pub status_service: StatusService,
    /// Agent 服务（基于 adk-rust）。
    pub agent_service: AgentService,
}

impl AppState {
    /// 装配应用依赖。后续接入数据库等有状态依赖时，在此构造并注入。
    pub fn new(settings: Settings) -> anyhow::Result<Self> {
        let agent_service = AgentService::new(&settings)?;
        Ok(Self {
            settings: Arc::new(settings),
            status_service: StatusService::new(),
            agent_service,
        })
    }
}
