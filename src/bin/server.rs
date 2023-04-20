use txt::Connection;

use std::io;
use tokio::net::{TcpListener, TcpStream};

const IP: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(IP).await?;

    loop {
        println!("开始监听");
        let (stream, _) = listener.accept().await.unwrap();
        process(stream).await;
    }
}

async fn process(stream: TcpStream) {
    let _ = tokio::spawn(async move {
        let mut con = Connection::new(stream);
        let one_frame = con.read_frame().await;
        println!("收到:{:?}", one_frame);
    })
    .await;
}
