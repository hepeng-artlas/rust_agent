# rust_agent

基于 [adk-rust](https://crates.io/crates/adk-rust) 构建的 AI Agent 应用，采用 **DDD（领域驱动设计）分层架构** 组织代码，默认接入 DeepSeek 模型。

## 技术栈

- **语言**：Rust（edition 2024）
- **Agent 框架**：adk-rust（启用 `deepseek` feature）
- **异步运行时**：tokio
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
├── main.rs                  # 二进制入口，启动 Agent
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

## 快速开始

### 1. 准备环境变量

复制示例文件并填入你的 DeepSeek API Key：

```bash
cp .env.example .env
```

```env
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

