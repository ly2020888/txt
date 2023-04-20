mod txt_files;

impl From<&str> for TxtFrame {
    fn from(value: &str) -> Self {}
}

impl TxtFrame {
    pub fn read_content(&mut self) -> TxtError {}
}
