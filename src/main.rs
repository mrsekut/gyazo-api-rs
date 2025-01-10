use dotenvy::dotenv;
use gyazo::Gyazo;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let gyazo_token = env::var("GYAZO_TOKEN").expect("Error reading GYAZO_TOKEN");
    let g = Gyazo::new(gyazo_token);
    let a = g.upload("image.jpg").await.unwrap();
    println!("{:?}", a);
}
