use std::net::SocketAddr;

use anyhow::Context;

/// 应用配置，从环境变量 / `.env` 加载。
#[derive(Debug, Clone)]
pub struct Settings {
    /// 运行环境：development / production 等。
    pub env: String,
    /// 日志级别：trace / debug / info / warn / error。
    pub log_level: String,
    /// HTTP 服务监听地址。
    pub host: String,
    /// HTTP 服务监听端口。
    pub port: u16,
    /// DeepSeek API Key（接入 LLM 时使用）。
    pub deepseek_api_key: Option<String>,
}

impl Settings {
    /// 从环境变量加载配置，缺省项使用默认值。
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            env: env_or("ENV", "development"),
            log_level: env_or("LOG_LEVEL", "info"),
            host: env_or("HOST", "127.0.0.1"),
            port: env_or("PORT", "8000")
                .parse()
                .context("环境变量 PORT 不是合法端口号")?,
            deepseek_api_key: std::env::var("DEEPSEEK_API_KEY").ok(),
        })
    }

    /// 解析出 HTTP 服务的监听地址。
    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .context("无法解析 HOST:PORT 为监听地址")
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
