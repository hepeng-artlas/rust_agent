//! rust_claw —— 基于 DDD（领域驱动设计）的分层架构。
//!
//! 分层说明（依赖方向由外向内：interfaces -> application -> domain，infrastructure 实现 domain 定义的接口）：
//! - `core`           核心配置与全局设置
//! - `domain`         领域层：实体、值对象、仓储与外部服务接口、领域服务
//! - `application`    应用层：用例编排与应用服务
//! - `infrastructure` 基础设施层：仓储、存储、外部服务的具体实现
//! - `interfaces`     接口层（表现层）：HTTP 路由、DTO、中间件、异常处理

pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;
