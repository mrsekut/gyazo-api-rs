// pub mod delete;
// pub mod list;
pub mod upload;

use reqwest::Client;

/// Gyazo API client.
pub struct Gyazo {
    pub access_token: String,
    pub client: Client,
}

impl Gyazo {
    pub fn new(access_token: String) -> Self {
        Gyazo {
            access_token,
            client: Client::new(),
        }
    }
}
