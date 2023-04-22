use txt::{
    frame::{Action, TxtFrame},
    Connection,
};

use std::io;
use tokio::net::{TcpListener, TcpStream};
use txt::txt_files;

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
async fn action(con: &mut Connection, frame: TxtFrame) {
    let mut frame = frame;
    match frame.action {
        Action::Empty => {
            println!("客户端消息：{}", frame.file_name);
            return;
        }
        Action::Download => {
            println!("收到下载请求：{}", frame.file_name);
            let _ = frame.read_file().await.map_err(|e| {
                frame.file_name = e.into();
            });
            let _ = con.write_frame(&frame).await.map_err(|e| {
                println!("服务器通信失败：{:?}", e);
            });
            return;
        }
        Action::Upload => {
            println!("收到上传请求：{}", frame.file_name);
            let _ = frame
                .write_file(&frame.file_name.clone())
                .await
                .map_err(|e| {
                    println!("服务器写入文件失败：{:?}", e);
                });
            let mut reply: TxtFrame = Default::default();
            reply.action = Action::Empty;
            reply.file_name = "成功写入服务器".to_string();
            let _ = con.write_frame(&reply);
            return;
        }
        Action::Select => {
            println!("收到目录加载请求");
            let mut index: TxtFrame = "index".into();
            let content: bytes::Bytes = txt_files::get_catalog(txt_files::PATH)
                .map_or(bytes::Bytes::from(""), |ok| bytes::Bytes::from(ok));
            index.file_body = content.to_vec();
            let _ = con.write_frame(&index).await.map_err(|e| {
                println!("服务器写入文件失败：{:?}", e);
            });
            return;
        }
    }
}
