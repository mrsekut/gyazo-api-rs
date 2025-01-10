use reqwest::{multipart, Client};
use std::fs;
use std::path::Path;

// use reqwest::StatusCode;

pub struct Gyazo {
    access_token: String,
    client: Client,
}

#[derive(serde::Deserialize, Debug)]
pub struct GyazoResponse {
    pub created_at: String,
    pub image_id: String,
    pub permalink_url: String,
    pub thumb_url: String,
    pub r#type: String,
    pub url: String,
}

impl Gyazo {
    pub fn new(access_token: String) -> Self {
        Gyazo {
            access_token,
            client: Client::new(),
        }
    }

    /// Uploads an image to Gyazo.
    // TODO: type, args:option
    pub async fn upload<P: AsRef<Path>>(
        &self,
        image_path: P,
    ) -> Result<GyazoResponse, reqwest::Error> {
        let file_content = fs::read(&image_path).expect("Failed to read the file");

        let form = multipart::Form::new()
            .text("access_token", self.access_token.clone())
            .part(
                "imagedata",
                multipart::Part::bytes(file_content)
                    .file_name(image_path.as_ref().to_str().unwrap().to_string()),
            );

        let response = self
            .client
            .post("https://upload.gyazo.com/api/upload")
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .json::<GyazoResponse>()
            .await?;

        Ok(response)
    }
    // pub async fn upload<P: AsRef<Path>>(
    //     &self,
    //     image_path: P,
    //     title: Option<&str>,
    //     desc: Option<&str>,
    // ) -> Result<UploadResponse, reqwest::Error> {
    //     let form = reqwest::multipart::Form::new()
    //         .text("access_token", self.access_token.clone())
    //         .text("title", title.unwrap_or("").to_string())
    //         .text("desc", desc.unwrap_or("").to_string())
    //         .file("imagedata", image_path)?;

    //     let response = self
    //         client.
    //         .post("https://upload.gyazo.com/api/upload")
    //         .multipart(form)
    //         .send()
    //         .await?
    //         .error_for_status()? // Converts non-2xx responses into errors
    //         .json::<UploadResponse>()
    //         .await?;

    //     Ok(response)
    // }

    // /// Lists uploaded images.
    // pub async fn list(&self, page: u32, per_page: u32) -> Result<ListResponse, reqwest::Error> {
    //     let response = self
    //         client.
    //         .get("https://api.gyazo.com/api/images")
    //         .query(&[
    //             ("access_token", &self.access_token),
    //             ("page", &page.to_string()),
    //             ("per_page", &per_page.to_string()),
    //         ])
    //         .send()
    //         .await?
    //         .error_for_status()?
    //         .json::<ListResponse>()
    //         .await?;

    //     Ok(response)
    // }

    // /// Deletes an image by its ID.
    // pub async fn delete(&self, image_id: &str) -> Result<DeleteResponse, reqwest::Error> {
    //     let url = format!("https://api.gyazo.com/api/images/{}", image_id);
    //     let response = self
    //         client.
    //         .delete(&url)
    //         .query(&[("access_token", &self.access_token)])
    //         .send()
    //         .await?
    //         .error_for_status()?
    //         .json::<DeleteResponse>()
    //         .await?;

    //     Ok(response)
    // }
}
