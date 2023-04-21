use crate::comm::frame::{TxtError, TxtFrame};
use lazy_static::lazy_static;
use regex::Regex;

impl From<&str> for TxtFrame {
    fn from(value: &str) -> Self {
        TxtFrame {
            action: 0,
            file_name: value.to_string(),
            file_body: b"".to_vec(),
        }
    }
}

impl TxtFrame {
    pub fn read_content(&mut self) -> Result<(), TxtError> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^.+[\.txt]|[\.md]|[\.zip]|[\.pdf]|[\.epub]").unwrap();
        }

        if RE.is_match(&self.file_name) {
            Ok(())
        } else {
            Err(TxtError::BadName)
        }
    }
}

#[cfg(test)]
mod test {

    use crate::comm::frame::TxtFrame;
    #[test]
    fn test_regex() {
        let mut one_frame = TxtFrame {
            action: 0,
            file_name: "file1.zip".to_string(),
            file_body: b"+asd56qw4e12q3w21a3s12d3ad\r\n".to_vec(),
        };
        let _ = one_frame
            .read_content()
            .map(|_a| {
                println!("匹配成功{}", one_frame.file_name.clone());
            })
            .map_err(|e| {
                println!("匹配失败{}:{:?}", one_frame.file_name.clone(), e);
            });
    }
}
