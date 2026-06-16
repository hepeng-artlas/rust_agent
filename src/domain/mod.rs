//! 领域层：业务核心。仅定义领域概念与抽象接口，不依赖任何外部框架。
//!
//! - `models`       领域模型：实体、值对象、聚合
//! - `repositories` 仓储接口（由 infrastructure 实现）
//! - `external`     外部服务抽象接口（由 infrastructure 实现，依赖倒置）
//! - `services`     领域服务：agents、flows、prompts、tools

pub mod external;
pub mod models;
pub mod repositories;
pub mod services;
