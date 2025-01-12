use super::Gyazo;

#[derive(serde::Deserialize, Debug)]
/// Response returned after deleting an image.
pub struct DeleteResponse {
    /// ID of the deleted image.
    pub image_id: String,
    /// Type of the response.
    pub r#type: String,
}

impl Gyazo {
    /// Deletes an image from Gyazo.
    ///
    /// # Arguments
    ///
    /// * `image_id` - The ID of the image to delete.
    ///
    /// # Returns
    ///
    /// A `Result` containing `DeleteResponse` on success or a `reqwest::Error` on failure.
    // TODO: test
    pub async fn delete(&self, image_id: &str) -> Result<DeleteResponse, reqwest::Error> {
        let url = format!("https://api.gyazo.com/api/images/{}", image_id);
        let res = self
            .client
            .delete(&url)
            .query(&[("access_token", &self.access_token)])
            .send()
            .await?
            .error_for_status()?
            .json::<DeleteResponse>()
            .await?;

        Ok(res)
    }
}
