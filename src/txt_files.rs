use crate::comm::frame::{Action, TxtError, TxtFrame};
use bytes::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::read_dir, io};
use tokio::fs;

pub const PATH: &str = "downloads/";

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^.+[\.txt]|[\.md]|[\.zip]|[\.pdf]|[\.epub]|[\.mobi]|[\.prc]").unwrap();
}

pub fn get_catalog(path: &str) -> Result<String, io::Error> {
    let mut indexs = "".to_string();
    for entry in read_dir(path)? {
        if let Ok(entry) = entry {
            let file = entry.path();
            let filename = file.to_str().unwrap();
            indexs += filename;
            indexs += "\r\n";
        }
    }
    Ok(indexs)
}

pub async fn catalog_frame(catalog: String) -> TxtFrame {
    TxtFrame {
        action: Action::Empty,
        file_name: "index".to_string(),
        file_body: Bytes::from(catalog).to_vec(),
    }
}

impl From<&str> for TxtFrame {
    fn from(value: &str) -> Self {
        let tf = TxtFrame {
            action: Action::Empty,
            file_name: value.to_string(),
            file_body: b"".to_vec(),
        };
        tf
    }
}

impl TxtFrame {
    pub async fn read_file(&mut self) -> Result<(), TxtError> {
        if RE.is_match(&self.file_name) {
            match fs::read_dir(PATH).await {
                Ok(_) => (),
                Err(_) => {
                    tokio::spawn(async {
                        let _ = fs::create_dir(PATH).await;
                    });
                    ()
                }
            };
            let contents = fs::read(PATH.to_string() + &self.file_name).await?;
            self.file_body = contents;

            Ok(())
        } else {
            Err(TxtError::BadName)
        }
    }

    // 把一个数据帧转化为文件
    pub async fn write_file(&mut self, filename: &str) -> Result<(), TxtError> {
        let path = PATH.to_string() + &filename;

        let exist = fs::try_exists(&path).await?;
        if !exist || filename == "index" {
            fs::write(&path, &self.file_body).await?;
        } else {
            let v: Vec<&str> = path.split(".").collect();
            let mut path_new = v[0].to_string();
            path_new.insert_str(v[0].len(), "(1).");
            fs::write(path_new + v[1], &self.file_body).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::comm::frame::TxtFrame;
    use tokio::runtime::Runtime;

    #[test]
    fn test_read_file() {
        // 给定文件名，自动封装为数据帧
        let mut one_frame: TxtFrame = "file1.txt".into();
        let rt = Runtime::new().unwrap();
        let handle = rt.handle();
        let fu = async move {
            let _ = one_frame
                .read_file()
                .await
                .map(|_a| {
                    println!("找到：{}", one_frame.clone());
                })
                .map_err(|e| {
                    println!("找不到：{}:{:?}", one_frame.file_name.clone(), e);
                });
        };
        handle.block_on(fu);
    }
    #[test]
    fn test_write_file() {
        let rt = Runtime::new().unwrap();
        let handle = rt.handle();
        handle.block_on(async {
            let mut tf: TxtFrame = "downloads.zip".into();
            let _ = tf.read_file().await;
            //println!("tf数据帧:{}", tf);
            let _ = tf.write_file("downloads2.zip").await.map_err(|err| {
                println!("写入文件失败:{:?}", err);
            });
        })
    }

    #[test]
    fn test_read_index() {
        let rt = Runtime::new().unwrap();
        let handle = rt.handle();
        let indexs: String = get_catalog(PATH)
            .map_err(|e| {
                println!("读取目录错误：{:?}", e);
            })
            .map_or("".to_string(), |ok| ok);
        println!("{indexs}");
        handle.block_on(async move {
            let mut tf = catalog_frame(indexs).await;
            let _ = tf.write_file("index").await;
        })
    }
}
