use txt::{frame::TxtFrame, Connection};

use std::io;
use tokio::net::{TcpListener, TcpStream};

// To-DO: 使用环境变量或配置文件取代硬编码
const IP: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(IP).await?;
    println!("准备开始接受请求");
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        process(stream).await;
    }
}

async fn process(stream: TcpStream) {
    let _ = tokio::spawn(async move {
        let mut con = Connection::new(stream);
        loop {
            let res = con.read_frame().await;
            match res {
                Ok(op) => {
                    if let Some(frame) = op {
                        // 处理数据帧
                        action(&mut con, frame).await;
                        continue;
                    }
                    return;
                }
                Err(_) => {
                    return;
                }
            }
        }
    });
}

// To-DO: 处理一个数据帧，完成上传，下载，获取索引的功能
async fn action(_con: &mut Connection, _frame: TxtFrame) {
    unimplemented!();
}
