//! Agent 服务：以 adk-rust 为核心，编排 LLM 智能体的会话与对话。
//!
//! 里程碑 1：仅支持最简单的单轮/多轮文本对话，会话与记忆由 adk-rust 的
//! `InMemorySessionService` 在内存中维护，后续可替换为 SQLite / Postgres 后端。

use std::collections::HashMap;
use std::sync::Arc;

use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_rust::futures::StreamExt;
use anyhow::Context;

use crate::core::config::Settings;

/// 应用内固定的 app 名称，用于 adk 会话作用域隔离。
const APP_NAME: &str = "rust_claw";

/// Agent 服务：持有一个共享的 adk `Runner` 与会话服务。
///
/// 所有字段均为 `Arc`，因此可廉价克隆并安全地在多请求间共享。
#[derive(Clone)]
pub struct AgentService {
    runner: Arc<Runner>,
    session_service: Arc<dyn SessionService>,
}

impl AgentService {
    /// 根据配置构建 Agent 服务：装配 DeepSeek 模型、LLM 智能体与执行器。
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        let api_key = settings
            .deepseek_api_key
            .clone()
            .context("缺少环境变量 DEEPSEEK_API_KEY，无法初始化 Agent 服务")?;

        let model = DeepSeekClient::chat(api_key).context("初始化 DeepSeek 模型失败")?;

        let agent = LlmAgentBuilder::new("assistant")
            .description("通用 AI 助手")
            .instruction("你是一个乐于助人的 AI 助手，请简洁准确地回答问题。")
            .model(Arc::new(model))
            .build()
            .context("构建 LLM 智能体失败")?;

        let session_service: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());

        let runner = Runner::builder()
            .app_name(APP_NAME)
            .agent(Arc::new(agent))
            .session_service(session_service.clone())
            .build()
            .context("构建 Agent Runner 失败")?;

        Ok(Self {
            runner: Arc::new(runner),
            session_service,
        })
    }

    /// 创建一个新的空白会话，返回会话 id。
    pub async fn create_session(&self, user_id: &str) -> anyhow::Result<String> {
        let session_id = SessionId::generate();

        self.session_service
            .create(CreateRequest {
                app_name: APP_NAME.to_string(),
                user_id: user_id.to_string(),
                session_id: Some(session_id.to_string()),
                state: HashMap::new(),
            })
            .await
            .context("创建会话失败")?;

        Ok(session_id.to_string())
    }

    /// 运行一次对话，返回 adk 的事件流（供流式与非流式调用共享）。
    async fn run(
        &self,
        session_id: &str,
        user_id: &str,
        message: &str,
    ) -> anyhow::Result<EventStream> {
        let user = UserId::new(user_id).context("非法的用户标识")?;
        let session = SessionId::try_from(session_id).context("非法的会话标识")?;
        let content = Content::new("user").with_text(message);

        self.runner.run(user, session, content).await.context("运行 Agent 失败")
    }

    /// 向指定会话发送一条用户消息，返回智能体生成的完整文本回复（非流式）。
    pub async fn chat(
        &self,
        session_id: &str,
        user_id: &str,
        message: &str,
    ) -> anyhow::Result<String> {
        let mut stream = self.run(session_id, user_id, message).await?;

        let mut reply = String::new();
        while let Some(event) = stream.next().await {
            let event = event.context("读取 Agent 事件失败")?;
            // 仅采纳最终事件（非 partial）的完整文本，避免与增量分块重复累计。
            if event.llm_response.partial {
                continue;
            }
            if let Some(content) = &event.llm_response.content {
                for part in &content.parts {
                    if let Some(text) = part.text() {
                        reply.push_str(text);
                    }
                }
            }
        }

        Ok(reply)
    }

    /// 向指定会话发送一条用户消息，返回底层事件流，供接口层翻译为 SSE。
    pub async fn chat_stream(
        &self,
        session_id: &str,
        user_id: &str,
        message: &str,
    ) -> anyhow::Result<EventStream> {
        self.run(session_id, user_id, message).await
    }
}
