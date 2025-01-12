use super::access_policy::AccessPolicy;
use super::Gyazo;

#[derive(serde::Deserialize, Debug)]
/// Information about a Gyazo image.
pub struct ImageInfo {
    /// Unique identifier of the image.
    pub image_id: String,
    /// URL to the image permalink.
    pub permalink_url: Option<String>,
    /// Direct URL to the image.
    pub url: String,
    /// Access policy of the image.
    pub access_policy: Option<AccessPolicy>,
    /// OCR information of the image.
    pub ocr: Ocr,
    /// Metadata associated with the image.
    pub metadata: Metadata,
    /// Type of the image.
    pub r#type: String,
    /// Creation timestamp of the image.
    pub created_at: String,
}

#[derive(serde::Deserialize, Debug)]
/// Metadata for a Gyazo image.
pub struct Metadata {
    /// Name of the application used to capture the image.
    pub app: Option<String>,
    /// Title of the captured website.
    pub title: Option<String>,
    /// URL of the captured website.
    pub url: Option<String>,
    /// Description of the image.
    pub desc: Option<String>,
    /// Original title of the image.
    pub original_title: Option<String>,
    /// Original URL of the image.
    pub original_url: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
/// OCR information for a Gyazo image.
pub struct Ocr {
    /// The locale used for OCR text recognition (e.g., "en" for English).
    pub locale: String,
    /// Description extracted by OCR.
    pub description: String,
}

impl Gyazo {
    /// Retrieves a list of images from Gyazo.
    ///
    /// # Arguments
    ///
    /// * `page` - The page number to retrieve.
    /// * `per_page` - Number of images per page.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `ImageInfo` on success or a `reqwest::Error` on failure.
    // TODO: test
    pub async fn list(&self, page: u32, per_page: u32) -> Result<Vec<ImageInfo>, reqwest::Error> {
        let res = self
            .client
            .get("https://api.gyazo.com/api/images")
            .query(&[
                ("access_token", &self.access_token),
                ("page", &page.to_string()),
                ("per_page", &per_page.to_string()),
            ])
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<ImageInfo>>()
            .await?;

        Ok(res)
    }

    /// Retrieves information about a specific image.
    ///
    /// # Arguments
    ///
    /// * `image_id` - The ID of the image to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing `ImageInfo` on success or a `reqwest::Error` on failure.
    // TODO: test
    pub async fn image(&self, image_id: &str) -> Result<ImageInfo, reqwest::Error> {
        let url = format!("https://api.gyazo.com/api/images/{}", image_id);
        let res = self
            .client
            .get(url)
            .query(&[("access_token", &self.access_token)])
            .send()
            .await?
            .error_for_status()?
            .json::<ImageInfo>()
            .await?;

        Ok(res)
    }
}
