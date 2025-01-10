use dotenvy::dotenv;
use gyazo::{Gyazo, GyazoUploadOptions};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let gyazo_token = env::var("GYAZO_TOKEN").expect("Error reading GYAZO_TOKEN");

    let options = GyazoUploadOptions {
        title: Some("My Image Title".to_string()),
        ..Default::default() // 他のフィールドはデフォルト値（`None`）
    };
    let g = Gyazo::new(gyazo_token);
    let a = g.upload("image.jpg", Some(&options)).await.unwrap();
    println!("{:?}", a);
}
