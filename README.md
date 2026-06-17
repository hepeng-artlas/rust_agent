# rust_claw

基于 [axum](https://github.com/tokio-rs/axum) + [adk-rust](https://crates.io/crates/adk-rust) 构建的 AI Agent 后端服务，采用 **DDD（领域驱动设计）分层架构** 组织代码，默认接入 DeepSeek 模型。

## 技术栈

- **语言**：Rust（edition 2024）
- **Web 框架**：axum + tower-http（CORS / 日志中间件）
- **Agent 框架**：adk-rust（启用 `deepseek` feature）
- **异步运行时**：tokio
- **流式响应**：SSE（axum sse + async-stream）
- **序列化**：serde / serde_json
- **日志**：tracing / tracing-subscriber
- **配置**：dotenvy（从 `.env` 读取环境变量）

## 架构设计

项目遵循 DDD 分层思想，依赖方向严格由外向内：

```
interfaces  ──►  application  ──►  domain  ◄──  infrastructure
```

- 外层依赖内层，`domain` 作为业务核心不依赖任何外层。
- `infrastructure` 反向实现 `domain` 中定义的接口（依赖倒置），运行时由外层注入具体实现。

## 目录结构

```
src/
├── main.rs                  # 二进制入口，启动 axum HTTP 服务
├── lib.rs                   # crate 根，声明各分层模块
│
├── core/                    # 核心层：配置、环境变量、全局设置
│
├── domain/                  # 领域层（业务核心，不依赖外部框架）
│   ├── models/              #   实体、值对象、聚合
│   ├── repositories/        #   仓储接口（持久化抽象）
│   ├── external/            #   外部服务抽象接口（依赖倒置）
│   └── services/            #   领域服务（核心业务逻辑）
│
├── application/             # 应用层（用例编排）
│   └── services/            #   应用服务，组合领域对象完成具体用例
│
├── infrastructure/          # 基础设施层（实现领域层定义的接口）
│   ├── repositories/        #   仓储实现（对接数据库等）
│   └── external/            #   外部服务实现（LLM 客户端、存储等）
│
└── interfaces/              # 接口层 / 表现层（对外入口）
    ├── endpoints/           #   路由与处理器
    └── schemas/             #   请求 / 响应 DTO
```

> 当前为起步阶段的轻量骨架，随着功能扩展可在对应层内再拆分子模块（例如外部服务变多时，在 `infrastructure/external/` 下新增 `llm.rs`、`search.rs` 等）。

## 功能进展

以 adk-rust 为核心、保留 DDD 分层与统一 API 契约，已落地：

- ✅ **健康检查**：`GET /api/status`
- ✅ **会话管理（内存）**：基于 adk `InMemorySessionService`，对话上下文自动维护（重启即丢失）
- ✅ **对话（非流式）**：`POST /api/sessions/{id}/chat`，一次性返回完整回复
- ✅ **对话（SSE 流式）**：`POST /api/sessions/{id}/chat/stream`，实时下发文本增量

> Agent 编排目前为最简单的单 LLM 智能体（DeepSeek）。规划/工具调用、持久化存储（SQLite/Postgres）、沙箱等能力将在后续里程碑接入。

## 快速开始

### 1. 准备环境变量

复制示例文件并按需修改：

```bash
cp .env.example .env
```

```env
ENV=development
LOG_LEVEL=info
HOST=127.0.0.1
PORT=8000
DEEPSEEK_API_KEY=your-deepseek-api-key
```

### 2. 构建

```bash
cargo build
```

### 3. 运行

```bash
cargo run
```

服务默认监听 `http://127.0.0.1:8000`。

## API

所有接口统一前缀 `/api`，统一响应结构为 `{ code, msg, data }`（SSE 流式接口除外）。

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/status` | 系统健康检查 |
| POST | `/api/sessions` | 创建一个新会话，返回 `session_id` |
| POST | `/api/sessions/{session_id}/chat` | 向会话发起一次对话（非流式，返回完整回复） |
| POST | `/api/sessions/{session_id}/chat/stream` | 向会话发起一次对话（SSE 流式） |

### 健康检查

```json
{
  "code": 200,
  "msg": "success",
  "data": [{ "service": "api", "status": "ok", "details": "" }]
}
```

### 对话（非流式）

请求体：`{ "message": "你好" }`，响应：

```json
{
  "code": 200,
  "msg": "success",
  "data": { "session_id": "<id>", "reply": "你好！有什么可以帮你的？" }
}
```

### 对话（SSE 流式）

请求体同上。响应为 SSE 事件流，事件信封为 `event: <type>`，`data` 为 JSON：

| 事件 | 数据 | 说明 |
|------|------|------|
| `delta` | `{ "text": "<chunk>" }` | 增量文本块，用于实时打字机渲染 |
| `message` | `{ "role": "assistant", "text": "<full>" }` | 一条完整的助手消息（权威值） |
| `error` | `{ "error": "<reason>" }` | 出错事件 |
| `done` | `{}` | 流结束标记 |

示例（`curl -N` 关闭缓冲方可看到逐字输出）：

```bash
# 1) 创建会话
curl -X POST http://127.0.0.1:8000/api/sessions

# 2) 流式对话
curl -N -X POST http://127.0.0.1:8000/api/sessions/<session_id>/chat/stream \
  -H "Content-Type: application/json" \
  -d '{"message":"用三句话介绍一下你自己"}'
```

