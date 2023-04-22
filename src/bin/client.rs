use std::error::Error;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
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
    let stream = TcpStream::connect(IP).await?;
    let mut con = Connection::new(stream);

    init().await;

    Ok(())
}

// let tf = TxtFrame {
//     action: Action::Download,
//     file_name: "file1.txt".to_string(),
//     file_body: b"body1".to_vec(),
// };
// let _ = con.write_frame(&tf).await;
// let mut file_frame = con.read_frame().await.unwrap().unwrap();
// println!("收到数据帧:{}", file_frame);
// let _ = file_frame
//     .write_file(&file_frame.file_name.clone())
//     .await
//     .map_err(|e| {
//         println!("{:?}", e);
//     });
async fn init() {
    tokio::spawn(async {
        let _ = fs::create_dir(txt_files::PATH).await;
    });
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: u8,
    },
}
