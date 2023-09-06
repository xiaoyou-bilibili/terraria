use crate::web::model::{AppState, EditConfig, GetConfigResp, HandleResult, Response, SendCmdReq};
use axum::{Extension, Json};
use std::fs::File;
use std::io::Write;
use lazy_static::lazy_static;
use crate::infra::conf::{read_data, write_data};
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    pub static ref GAME_STATUS: Mutex<bool> = Mutex::new(false);
}

// 修改游戏配置
pub async fn edit_config(Json(payload): Json<EditConfig>) -> HandleResult<String> {
    // 创建一个新文件，如果文件已存在，则会被覆盖
    let file = File::create("server/config.txt");
    if let Err(e) = file {
        return Response::err2("文件创建失败", e);
    }
    // 要写入文件的文本
    let content = format!("# 服务器配置，请不要直接修改\nworld=/home/xiaoyou/game/terraria/world/main.wld\nmaxplayers={}\nport={}\npassword={}\nautocreate={}\nworldname={}\ndifficulty={}\nmotd={}\nlanguage=zh-Hans\nupnp=1",
                          payload.max_player,
                          payload.port,
                          payload.password,
                          payload.word_size,
                          payload.word_name,
                          payload.difficulty,
                          payload.message,
    );

    // 将文本写入文件
    let mut file = file.unwrap();
    file.write_all(content.as_bytes()).unwrap();
    if let Err(e) = file.sync_all() {
        return Response::err2("写入文件失败", e);
    }
    // 写入配置文件
    return write_data(payload)
}

// 启动游戏
pub async fn start_game(
    Extension(mut app_state): Extension<AppState>,
) -> HandleResult<Vec<String>> {
    app_state.game.lock().unwrap().start_game();
    let mut status = GAME_STATUS.lock().unwrap();
    *status = true;
    Response::ok2()
}

// 关闭游戏
pub async fn stop_game(
    Extension(mut app_state): Extension<AppState>,
) -> HandleResult<Vec<String>> {
    app_state.game.lock().unwrap().stop_game();
    let mut status = GAME_STATUS.lock().unwrap();
    *status = false;
    Response::ok2()
}

// 发送命令
pub async fn send_cmd(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<SendCmdReq>
) -> HandleResult<Vec<String>> {
    if let Err(e) = app_state.game.lock().unwrap().send_data(format!("{}\r\n", payload.cmd).as_str()) {
        return Response::err2("发送命令失败 {}", e)
    }
    Response::ok2()
}

// 获取配置
pub async fn get_config(
    Extension(app_state): Extension<AppState>,
) -> HandleResult<GetConfigResp> {
    // 获取配置文件
    let conf = read_data();
    if conf.is_some() {
        return Response::ok(GetConfigResp{
            game_status: *(GAME_STATUS.lock().unwrap()),
            config: conf.unwrap(),
        })
    }
    Response::err("no conf found")
}