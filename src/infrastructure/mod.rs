//! 基础设施层：实现 domain 定义的仓储与外部服务接口，对接具体技术（数据库、第三方等）。
//!
//! - `repositories` 仓储实现
//! - `external`     外部服务实现（llm、存储等）

pub mod external;
pub mod repositories;
