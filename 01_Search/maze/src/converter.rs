use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

pub struct MazeConverter<'a> {
    pub file_name: &'a str,
}

impl<'a> MazeConverter<'a> {
    pub fn new(file_name: &'a str) -> Self {
        Self { file_name }
    }

    pub async fn read(&self) {
        let file = tokio::fs::File::open(self.file_name)
            .await
            .expect("could not open file.");

        let mut lines = BufReader::new(file).lines();

        while let Some(line) = lines.next_line().await.expect("error") {
            println!("{}", line);
        }

        println!("reached end of read fct.");
    }
}
