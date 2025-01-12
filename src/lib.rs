#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! # Gyazo API
//!
//! This crate provides a client for interacting with the Gyazo API.
//! It includes modules for uploading, fetching, and deleting images.
//!
//! ## Example
//!
//! ```rust
//! use gyazo_api::{upload::GyazoUploadOptions, Gyazo};
//!
//! let gyazo = Gyazo::new("your_access_token".to_string());
//! let options = GyazoUploadOptions {
//!     title: Some("My Image Title".to_string()),
//!     ..Default::default()
//! };
//! let result = gyazo.upload("image.jpg", Some(&options)).await;
//! ```

/// Handles image deletion via the Gyazo API.
pub mod delete;

/// Handles fetching images and metadata via the Gyazo API.
pub mod get;

/// Handles uploading images via the Gyazo API.
pub mod upload;

// pub mod onEmbed;

mod access_policy;

use reqwest::Client;

/// A client for interacting with the Gyazo API.
pub struct Gyazo {
    /// Access token for authentication.
    pub access_token: String,
    /// HTTP client for making requests.
    pub client: Client,
}

impl Gyazo {
    /// Creates a new instance of the Gyazo client.
    ///
    /// # Arguments
    ///
    /// * `access_token` - A `String` representing the access token.
    ///
    /// # Example
    ///
    /// ```
    /// let gyazo = Gyazo::new("your_access_token".to_string());
    /// ```
    pub fn new(access_token: String) -> Self {
        Gyazo {
            access_token,
            client: Client::new(),
        }
    }
}
