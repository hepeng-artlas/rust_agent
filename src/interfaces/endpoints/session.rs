use std::convert::Infallible;

use adk_rust::futures::{Stream, StreamExt};
use async_stream::stream;
use axum::extract::{Path, State};
use axum::response::sse::{Event as SseAxumEvent, KeepAlive, Sse};
use axum::{Json, Router, routing::post};

use crate::application::errors::AppResult;
use crate::interfaces::schemas::response::Response;
use crate::interfaces::schemas::session::{ChatRequest, ChatResponse, CreateSessionResponse};
use crate::interfaces::schemas::sse::{SseFrame, adk_event_to_frames};
use crate::state::AppState;

/// 里程碑 1 暂无鉴权，统一使用固定用户标识。
const DEFAULT_USER_ID: &str = "user";

/// 会话模块路由。
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sessions", post(create_session))
        .route("/sessions/{session_id}/chat", post(chat))
        .route("/sessions/{session_id}/chat/stream", post(chat_stream))
}

/// 创建一个新的空白会话。
async fn create_session(
    State(state): State<AppState>,
) -> AppResult<Json<Response<CreateSessionResponse>>> {
    let session_id = state.agent_service.create_session(DEFAULT_USER_ID).await?;

    Ok(Json(Response::with(
        200,
        "创建会话成功",
        CreateSessionResponse { session_id },
    )))
}

/// 向指定会话发起一次对话（非流式，返回完整回复）。
async fn chat(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<ChatRequest>,
) -> AppResult<Json<Response<ChatResponse>>> {
    let reply = state
        .agent_service
        .chat(&session_id, DEFAULT_USER_ID, &request.message)
        .await?;

    Ok(Json(Response::success(ChatResponse { session_id, reply })))
}

/// 向指定会话发起一次流式对话（SSE）：实时下发 `delta`，最后下发 `message` 与 `done`。
async fn chat_stream(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<ChatRequest>,
) -> AppResult<Sse<impl Stream<Item = Result<SseAxumEvent, Infallible>>>> {
    let event_stream = state
        .agent_service
        .chat_stream(&session_id, DEFAULT_USER_ID, &request.message)
        .await?;

    let sse_stream = stream! {
        let mut event_stream = event_stream;
        while let Some(item) = event_stream.next().await {
            match item {
                Ok(event) => {
                    for frame in adk_event_to_frames(&event) {
                        yield Ok(to_sse_event(frame));
                    }
                }
                Err(err) => {
                    yield Ok(to_sse_event(SseFrame::error(err.to_string())));
                    break;
                }
            }
        }
        yield Ok(to_sse_event(SseFrame::done()));
    };

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}

/// 将一个 SSE 帧转换为 axum 的 SSE 事件。
fn to_sse_event(frame: SseFrame) -> SseAxumEvent {
    SseAxumEvent::default().event(frame.event).data(frame.data.to_string())
}
