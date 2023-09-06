use std::sync::{Arc, Mutex};
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::infra::game::GameServer;

pub type HandleResult<T> = Json<Response<T>>;

// 应用状态共享
#[derive(Clone)]
pub struct AppState {
    // 数据库
    pub game: Arc<Mutex<GameServer>>
}

// 自定义通用返回
#[derive(Serialize)]
pub struct Response<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: String,
}

// 自定义通用返回快捷方法
impl<T> Response<T>
where
    T: Serialize,
{
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }
    // 请求成功
    pub fn ok(data: T) -> HandleResult<T> {
        Json(Self::new(0, "ok".to_string(), Some(data)))
    }
    // 请求成功不返回数据
    pub fn ok2() -> HandleResult<T> {
        Json(Self::new(0, "ok".to_string(), None))
    }
    // 请求失败
    pub fn err(msg: &str) -> HandleResult<T> {
        Json(Self::new(-1, String::from(msg), None))
    }
    // 请求失败
    pub fn err2<E: std::fmt::Debug>(msg: &str, err: E) -> HandleResult<T> {
        Json(Self::new(
            -1,
            String::from(format!("{} {:?}", msg, err).as_str()),
            None,
        ))
    }
}

// 修改配置
#[derive(Debug, Serialize, Deserialize)]
pub struct EditConfig {
    pub max_player: i32,   // 最大玩家数
    pub port: i32,         // 端口
    pub password: String,  // 密码
    pub word_size: i32,    // 世界大小 1 小 2 中 3 大
    pub word_name: String, // 世界名称
    pub difficulty: i32,   // 难度  0（普通），1（专家），2（大师），3（旅行者）
    pub message: String,   // 今天发送的消息
}

// 配置内容
#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfigResp {
    pub game_status: bool,
    pub config: EditConfig,
}

// 命令数据
#[derive(Debug, Serialize, Deserialize)]
pub struct SendCmdReq {
    pub cmd: String,
}