use std::env;
use crate::web::handle::{edit_config, get_config, send_cmd, start_game, stop_game};
use axum::extract::DefaultBodyLimit;
use axum::{routing::{get, post}, Router, ServiceExt, Extension};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower::layer::layer_fn;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};
use tracing_subscriber::fmt;
use crate::infra::game::GameServer;
use crate::web::middleware;
use crate::web::model::{AppState};

mod infra;
mod web;

#[tokio::main]
async fn main() {
    fmt::Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();
    // 获取环境变量
    let mut token = "xiaoyou".to_string();
    match env::var("SERVER_TOKEN") {
        Ok(value) => token = value,
        Err(e) => error!("无法读取环境变量: {}", e),
    }
    // 新建router
    let app = Router::new()
        // 主页路由
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/config", post(edit_config))
        .route("/api/config", get(get_config))
        .route("/api/game/start", get(start_game))
        .route("/api/game/stop", get(stop_game))
        .route("/api/game/cmd", post(send_cmd))
        //绑定websocket路由
        .route("/api/ws", get(web::websocket::ws_handler))
        .layer(Extension(AppState{ game: Arc::new(Mutex::new(GameServer::build()))}))  //  全局状态共享
        // 自定义认证中间件
        .layer(layer_fn(move |inner| middleware::AuthMiddleware { inner, token: token.clone() }));
    info!("server listen on 3000");
    // 绑定socket
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = ServiceBuilder::new()
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100))
        .service(app);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
