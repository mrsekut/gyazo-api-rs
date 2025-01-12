# gyazo-api

`gyazo-api` is a Rust library for interacting with the [Gyazo API](https://gyazo.com/api/docs).
It allows you to upload images, retrieve image details, and fetch image lists, all with ease.

## Features

- Fetch a list of images from your Gyazo account.
- Retrieve detailed information about a specific image, including metadata and OCR data.
- Upload new images to Gyazo with customizable options.
- Fully asynchronous API using `reqwest`.

## Installation

```bash
cargo add gyazo-api
```

## Usage

### **Uploading an Image**

```rust
use gyazo_api::{upload::GyazoUploadOptions, Gyazo};

#[tokio::main]
async fn main() {
    let gyazo = Gyazo::new("YOUR_GYAZO_TOKEN");

    let options = GyazoUploadOptions {
        title: Some("My Image Title".to_string()),
        ..Default::default()
    };

    match gyazo.upload("image.jpg", Some(&options)).await {
        Ok(response) => println!("Image uploaded successfully: {:?}", response),
        Err(e) => eprintln!("Error uploading image: {}", e),
    }
}
```

### **Fetching a List of Images**

```rust
use gyazo_api::Gyazo;

#[tokio::main]
async fn main() {
    let gyazo = Gyazo::new("YOUR_GYAZO_TOKEN");

    match gyazo.list(1, 10).await {
        Ok(images) => {
            for image in images {
                println!("Image ID: {}, URL: {}", image.image_id, image.url);
            }
        }
        Err(e) => eprintln!("Error fetching image list: {}", e),
    }
}
```

### **Retrieving Image Details**

```rust
use gyazo_api::Gyazo;

#[tokio::main]
async fn main() {
    let gyazo = Gyazo::new("YOUR_GYAZO_TOKEN");

    match gyazo.image("IMAGE_ID").await {
        Ok(image_info) => println!("Image Info: {:?}", image_info),
        Err(e) => eprintln!("Error fetching image info: {}", e),
    }
}
```

## License

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)
