//! 会话模块的请求/响应 DTO。

use serde::{Deserialize, Serialize};

/// 创建会话的响应体。
#[derive(Debug, Clone, Serialize)]
pub struct CreateSessionResponse {
    /// 新建会话的 id。
    pub session_id: String,
}

/// 发起对话的请求体。
#[derive(Debug, Clone, Deserialize)]
pub struct ChatRequest {
    /// 用户输入的消息内容。
    pub message: String,
}

/// 对话的响应体（里程碑 1：一次性返回完整回复，非流式）。
#[derive(Debug, Clone, Serialize)]
pub struct ChatResponse {
    /// 会话 id。
    pub session_id: String,
    /// 智能体生成的完整回复文本。
    pub reply: String,
}
