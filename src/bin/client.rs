use std::error::Error;

use tokio::net::TcpStream;
use txt::frame::TxtFrame;
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
    let tf = TxtFrame {
        action: 0,
        file_name: "file_1".to_string(),
        file_body: b"body1".to_vec(),
    };
    let _ = con.write_frame(&tf).await;
    Ok(())
}
