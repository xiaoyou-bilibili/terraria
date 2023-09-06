use std::sync::{Arc};
use std::thread;
use crossbeam::channel;
use tracing::{error, info};
use crate::infra::shell::ShellServer;

pub struct GameServer {
    shell: Option<ShellServer>,
    read: Option<Arc<channel::Receiver<String>>>,
    send: Option<Arc<channel::Sender<String>>>,
}

impl GameServer {
    // 初始化game
    pub fn build() -> GameServer {
        GameServer{
            shell: None,
            read: None,
            send: None
        }
    }

    // 获取一个接受者
    pub fn get_recv(&self) -> Option<Arc<channel::Receiver<String>>> {
        if let Some(read) = self.read.as_ref() {
           return Some(Arc::clone(&read));
        }
        None
    }

    // 获取一个发送者
    pub fn get_send(&self) -> Option<Arc<channel::Sender<String>>> {
        if let Some(send) = self.send.as_ref() {
            return Some(Arc::clone(&send));
        }
        None
    }

    // 发送数据
    pub fn send_data(&mut self, data: &str) -> Result<(), String> {
        let sender = self.get_send();
        if let Some(send) = sender {
            if let Err(e) = send.send(String::from(data)) {
                return Err(format!("send data err {}", e))
            }
        };
        Ok(())
    }


    // 启动游戏
    pub fn start_game(&mut self) {
        self.shell = Some(ShellServer::build("./server/TerrariaServer.bin.x86_64", vec!["-config", "./config.txt"]));
        let (mut write, mut read) = self.shell.as_mut().unwrap().split();
        // self.read = Arc::new(Mutex::new(Some(read)));
        let (read_tx, read_rx) = channel::unbounded();
        let (send_tx, send_rx) = channel::unbounded();
        self.read = Some(Arc::new(read_rx));
        self.send = Some(Arc::new(send_tx));
        // 单开一个线程负责接受数据
        thread::spawn(move || {
            let mut buf = String::new();
            while let Ok(size) = read.read_data(&mut buf) {
                if size == 0 {
                    break;
                }
                if let Err(e) = read_tx.send(buf.clone()) {
                    error!("send data err {}", e)
                }
                info!("read data {}", buf);
                buf.clear();
            }
        });
        // 单开一个线程负责发送数据
        thread::spawn(move || {
            while let Ok(msg) = send_rx.recv() {
                info!("send data {}", msg);
                if let Err(e) = write.write_data(msg) {
                    error!("write data err {}", e)
                }
            }
        });
    }

    // 关闭游戏
    pub fn stop_game(&mut self) {
        if let Err(e) =  self.send_data("exit\r\n") {
            error!("send data err {}", e)
        }
    }
}