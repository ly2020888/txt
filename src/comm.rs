pub mod frame;

use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

pub use frame::{TxtError, TxtFrame};
const TCP_BUF_SIZE: usize = 16 * 1024 * 1024;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(TCP_BUF_SIZE),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<TxtFrame>, TxtError> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
            if let Ok(0) = self.stream.read_buf(&mut self.buffer).await {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(TxtError::Reset);
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<TxtFrame>, TxtError> {
        let mut src = Cursor::new(&self.buffer[..]);

        match frame::TxtFrame::check(&mut src) {
            Ok(_) => {
                let len = src.position() as usize;
                src.set_position(0);
                let tf = frame::TxtFrame::parse(&mut src).map_err(|e| {
                    println!("解析数据帧出错:{:?}", e);
                });
                if let Ok(tf) = tf {
                    self.buffer.advance(len);
                    Ok(Some(tf))
                } else {
                    Ok(None)
                }
            }
            Err(TxtError::Incomplete) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn write_frame(&mut self, tf: &TxtFrame) -> Result<(), TxtError> {
        self.stream.write(b"+").await?;
        let wbytes = frame::TxtFrame::serialize(tf.clone())
            .map_err(|err| {
                return err;
            })
            .unwrap();
        self.stream.write(&wbytes).await?;
        self.stream.write(b"\r\n").await?;
        self.stream.flush().await?;
        Ok(())
    }
}
