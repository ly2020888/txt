use bytes::{Buf, Bytes};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fmt::Display, io::Cursor};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxtFrame {
    pub action: i32,
    pub file_name: String,
    pub file_body: Vec<u8>,
}

// "start with '+' ,end with '\r''\n'"
impl TxtFrame {
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), TxtError> {
        match Self::get_u8(src) {
            Ok(b'+') => {
                let _ = Self::get_line(src);
                Ok(())
            }
            Ok(_) => Err(TxtError::UnknownToken),
            Err(e) => Err(e),
        }
    }
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<TxtFrame, TxtError> {
        match Self::get_u8(src) {
            Ok(b'+') => {
                let metadata = Self::get_line(src);
                match metadata {
                    Ok(data) => Self::unserialize(Bytes::from(data.to_vec())),
                    Err(e) => Err(e),
                }
            }
            Ok(_) => Err(TxtError::UnknownToken),
            Err(e) => Err(e),
        }
    }

    pub fn serialize(tf: TxtFrame) -> Result<Bytes, TxtError> {
        serde_json::to_string(&tf)
            .map_err(|err| TxtError::SerializeErr(err))
            .map(|tf| Bytes::from(tf))
    }

    pub fn unserialize(b: Bytes) -> Result<TxtFrame, TxtError> {
        serde_json::from_slice(&b.clone()).map_err(|err| TxtError::SerializeErr(err))
    }

    fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, TxtError> {
        if !src.has_remaining() {
            return Err(TxtError::Incomplete);
        }

        Ok(src.get_u8())
    }

    fn get_line<'a>(src: &'a mut Cursor<&[u8]>) -> Result<&'a [u8], TxtError> {
        let start = src.position() as usize;
        let end = src.get_ref().len() - 1;

        for i in start..end {
            if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
                src.set_position((i + 2) as u64);

                return Ok(&src.get_ref()[start..i]);
            }
        }
        Err(TxtError::Incomplete)
    }
}

impl Display for TxtFrame {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("文件名为{}", self.file_name);
        println!("文件体为{:?}", Bytes::from(self.file_body.clone()));
        Ok(())
    }
}

#[derive(Debug)]
pub enum TxtError {
    /// Not enough data is available to parse a message
    Incomplete,
    SerializeErr(serde_json::Error),
    UnknownToken,
    Reset,
    IoErr(std::io::Error),
    BadName,
}

impl From<std::io::Error> for TxtError {
    fn from(value: std::io::Error) -> Self {
        TxtError::IoErr(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let one_frame = TxtFrame {
            action: 0,
            file_name: "file1".to_string(),
            file_body: b"+asd56qw4e12q3w21a3s12d3ad\r\n".to_vec(),
        };
        let meta = TxtFrame::serialize(one_frame).unwrap();
        println!("tf is : {:?}", meta);
    }

    #[test]
    fn test_unserialize() {
        let one_frame = TxtFrame {
            action: 0,
            file_name: "file1".to_string(),
            file_body: b"+asd56qw4e12q3w21a3s12d3ad\r\n".to_vec(),
        };
        let meta = TxtFrame::serialize(one_frame).unwrap();
        let one_tf = TxtFrame::unserialize(meta).unwrap();
        println!("tf is : {:?}", one_tf);
    }

    #[test]
    fn test_parse() {
        let parse_me = b"+{\"action\":0,\"file_name\":\"file1\",\"file_body\":[43,97,115,100,53,54,113,119,52,101,49,50,113,51,119,50,49,97,51,115,49,50,100,51,97,100,13,10]}\r\n+{\"action\":0,\"file_name\":\"file1\",\"file_body\":[115,115,115,110,53,54,113,119,52,101,49,50,113,51,119,50,49,97,51,115,49,50,100,51,97,100,13,10]}\r\n+";
        let parse_me_slice = &parse_me[..];
        let mut src = Cursor::new(parse_me_slice);
        src.set_position(0);
        let tf = TxtFrame::parse(&mut src).unwrap();
        println!("tf is : {}", tf);
        let tf = TxtFrame::parse(&mut src).unwrap();
        println!("tf is : {}", tf);
        let _ = TxtFrame::parse(&mut src).map_err(|e| {
            println!("error should be : {:?}", e);
        });
    }

    #[test]
    fn test_cursor() {
        use bytes::Buf;
        let mut buf = &b"\x08 hello"[..];
        assert_eq!(8, buf.get_u8());
        println!(
            "buf whose current position advanced by 1 is : {:?}",
            Bytes::from(buf)
        );
    }
}
