use axum::{Extension, extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    TypedHeader,
}, headers, response::IntoResponse};
use futures::SinkExt;
use tracing::{error, info};
// 支持socket并发读写
use futures::stream::StreamExt;
use crate::web::model::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    app_state: Extension<AppState>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(move |ws| handle_socket(ws, app_state))
}

async fn handle_socket(ws: WebSocket, Extension(app_state): Extension<AppState>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let read = app_state.game.lock().unwrap().get_recv();
    let send = app_state.game.lock().unwrap().get_send();
    // 新建一个任务处理websocket的读取操作
    let task1 = tokio::spawn(async move {
        info!("into thread1");
        if send.is_none() {
            return;
        }
        while let Some(msg) = ws_rx.next().await {
            if let Ok(msg) = msg {
                match msg.into_text() {
                    Ok(txt) => {
                        info!("recv data {}", txt);
                        if let Err(err) = send.as_ref().unwrap().send(txt) {
                            error!("Failed to send message to shell: {}", err);
                        }
                    }
                    Err(error) => error!("Failed to open file: {}", error),
                }
            } else {
                error!("client disconnected");
                return;
            }
        }
    });
    // 一个线程用来读取数据
    let task2 = tokio::spawn(async move {
        info!("into thread2");
        if let Some(read) = read.as_ref() {
            while let Ok(msg) = read.recv() {
                if let Err(err) = ws_tx.send(Message::Text(msg)).await {
                    error!("Failed to send data to websocket: {}", err);
                }
            }
        }
    });
    // 等待两个执行完毕
    tokio::join!(task1, task2);
}
