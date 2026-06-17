//! SSE 流式事件协议：将 adk-rust 的 `Event` 翻译为前端可消费的 SSE 帧。
//!
//! 里程碑 2 的事件协议（envelope 为 `event: <type>`，`data` 为 JSON）：
//! - `delta`   增量文本块，用于前端打字机式实时渲染。`{ "text": "<chunk>" }`
//! - `message` 一条完整的助手消息（权威值）。`{ "role": "assistant", "text": "<full>" }`
//! - `error`   出错事件。`{ "error": "<reason>" }`
//! - `done`    流结束标记。`{}`
//!
//! 说明：adk 的流式分块（`partial = true`）携带的是**增量** delta；
//! 而最终事件（`partial = false`）携带**完整累计文本**，因此两者分别映射为
//! `delta` 与 `message`，既支持实时渲染，也提供一条权威的最终消息。

use adk_rust::Event as AdkEvent;
use serde_json::{Value, json};

/// 一个待发送的 SSE 帧（事件名 + JSON 数据）。
pub struct SseFrame {
    /// SSE 的 `event:` 字段。
    pub event: &'static str,
    /// SSE 的 `data:` 字段（序列化为 JSON 字符串发送）。
    pub data: Value,
}

impl SseFrame {
    fn new(event: &'static str, data: Value) -> Self {
        Self { event, data }
    }

    /// 错误帧。
    pub fn error(reason: impl Into<String>) -> Self {
        Self::new("error", json!({ "error": reason.into() }))
    }

    /// 结束帧。
    pub fn done() -> Self {
        Self::new("done", json!({}))
    }
}

/// 将一个 adk `Event` 翻译为零或多个 SSE 帧。
///
/// 目前只关注文本内容：增量分块映射为 `delta`，最终完整文本映射为 `message`。
/// 工具调用等事件类型将在后续里程碑接入。
pub fn adk_event_to_frames(event: &AdkEvent) -> Vec<SseFrame> {
    let Some(content) = &event.llm_response.content else {
        return Vec::new();
    };

    let text: String = content.parts.iter().filter_map(|part| part.text()).collect();
    if text.is_empty() {
        return Vec::new();
    }

    let frame = if event.llm_response.partial {
        SseFrame::new("delta", json!({ "text": text }))
    } else {
        SseFrame::new("message", json!({ "role": "assistant", "text": text }))
    };

    vec![frame]
}
