use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        file: Option<String>,
    },
    /// 下载一个文件, 必须提供文件名
    Download {
        /// 需要操作的文件名称，所有文件限于同目录下的downloads文件
        #[arg(short, long)]
        file: Option<String>,
    },
    /// 搜索一个文件, 如果不提供文件名则返回全体文件列表
    Select {
        /// 需要操作的文件名称，所有文件限于同目录下的downloads文件
        #[arg(short, long)]
        file: Option<String>,
    },
}
fn main() {
    let cli = Cli::parse();

    // Continued program logic goes here...
}
