use std::error::Error;

use tokio::net::TcpStream;
use txt::frame::TxtFrame;
use txt::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:9000").await?;

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
