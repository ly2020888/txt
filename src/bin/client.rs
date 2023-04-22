use std::error::Error;

use clap::{Parser, Subcommand};
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
    let cli = Cli::parse();
    let stream = TcpStream::connect(IP).await?;
    let con = Connection::new(stream);

    init().await;
    process(cli, con).await;

    Ok(())
}

async fn init() {
    tokio::spawn(async {
        let _ = fs::create_dir(txt_files::PATH).await;
    });
}

async fn process(cli: Cli, con: Connection) {
    let mut con = con;
    match &cli.command {
        Commands::Upload { file } => {
            let mut new_tf: TxtFrame = file.into();
            new_tf.action = Action::Upload;
            let res = new_tf.read_file().await;
            if let Ok(_) = res {
                let _ = con.write_frame(&new_tf).await;
                return;
            } else {
                println!("读取文件:{}出错", new_tf.file_name);
            }
        }
        Commands::Download { file } => {
            let mut new_tf: TxtFrame = file.into();
            new_tf.action = Action::Download;
            request(new_tf, con).await;
        }

        Commands::Select { file } => {
            let mut new_tf: TxtFrame = Default::default();

            if let Some(file) = file {
                new_tf.file_name = file.into();
            }
            new_tf.action = Action::Select;
            request(new_tf, con).await;
            txt_files::show_catalog().await;
        }
    }
}

async fn request(new_tf: TxtFrame, con: Connection) {
    let mut con = con;
    let _ = con.write_frame(&new_tf).await;
    match con.read_frame().await {
        Ok(op) => {
            if let Some(mut v) = op {
                println!("服务器返回:{}", v.file_name);
                if new_tf.file_name == v.file_name {
                    let _ = v.write_file(&v.file_name.clone()).await;
                }
                return;
            }
            println!("获取到的文件为空");
        }
        Err(e) => {
            println!("下载失败:{:?}", e);
        }
    }
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 上传一个文件, 必须提供文件名
    Upload {
        /// 需要操作的文件名称，所有文件限于同目录下的downloads文件
        #[arg(short, long)]
        file: String,
    },
    /// 下载一个文件, 必须提供文件名
    Download {
        /// 需要操作的文件名称，所有文件限于同目录下的downloads文件
        #[arg(short, long)]
        file: String,
    },
    /// 搜索一个文件, 如果不提供文件名则返回全体文件列表
    Select {
        /// 需要操作的文件名称，所有文件限于同目录下的downloads文件
        #[arg(short, long)]
        file: Option<String>,
    },
}
