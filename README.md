# gyazo-api-rs

## Installation

```
cargo add gyazo
```

## Usage

```rs
use gyazo::gyazo::{upload::GyazoUploadOptions, Gyazo};

#[tokio::main]
async fn main() {
    let gyazo = Gyazo::new("YOUR GYAZO TOKEN");

    let options = GyazoUploadOptions {
        title: Some("My Image Title".to_string()),
        ..Default::default()
    };
    let result = gyazo.upload("image.jpg", Some(&options)).await.unwrap();
}
```
