use std::fs::File;
use std::io::{BufReader, Write};
use tracing::log::error;
use crate::web::model::{EditConfig, HandleResult, Response};

pub fn write_data(conf: EditConfig) -> HandleResult<String> {
    // 创建一个新文件，如果文件已存在，则会被覆盖
    let file = File::create("config.json");
    if let Err(e) = file {
        return Response::err2("文件创建失败", e);
    }
    // 转义json
    let json_string = serde_json::to_string(&conf).unwrap();
    // 将文本写入文件
    let mut file = file.unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
    if let Err(e) = file.sync_all() {
        return Response::err2("写入文件失败", e);
    }
    Response::ok2()
}

pub fn read_data() ->Option<EditConfig> {
    let file = File::open("config.json");
    if let Err(e) = file {
        error!("文件读取失败{}", e);
        return None
    }
    let reader = BufReader::new(file.unwrap());
    let conf: EditConfig = serde_json::from_reader(reader).unwrap();
    return Some(conf)
}