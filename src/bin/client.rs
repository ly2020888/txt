use std::error::Error;

use tokio::fs;
use tokio::net::TcpStream;
use txt::frame::{Action, TxtFrame};
use txt::txt_files;
use txt::Connection;

// To-DO: 使用环境变量或配置文件取代硬编码
const IP: &str = "127.0.0.1:9000";

// To-DO: 实现命令解析，发起上传，下载、获取索引等功能
// 完善命令行工具的提示信息。
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect(IP).await?;

    // Write some data.
    let mut con = Connection::new(stream);
    let _ = fs::create_dir(txt_files::PATH).await;
    let tf = TxtFrame {
        action: Action::Download,
        file_name: "file1.txt".to_string(),
        file_body: b"body1".to_vec(),
    };
    let _ = con.write_frame(&tf).await;
    let mut file_frame = con.read_frame().await.unwrap().unwrap();
    println!("收到数据帧:{}", file_frame);
    let _ = file_frame
        .write_file(&file_frame.file_name.clone())
        .await
        .map_err(|e| {
            println!("{:?}", e);
        });
    Ok(())
}
