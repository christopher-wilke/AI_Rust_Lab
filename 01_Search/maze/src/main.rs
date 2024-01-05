mod converter;

#[tokio::main]
async fn main() {
    let converter = converter::MazeConverter::new("./levels/01.txt");
    converter.read().await;
}
