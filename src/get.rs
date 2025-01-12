use super::Gyazo;

#[derive(serde::Deserialize, Debug)]
pub struct ImageInfo {
    pub image_id: String,
    pub permalink_url: Option<String>,
    pub url: String,
    pub access_policy: Option<String>,
    pub ocr: Ocr,
    pub metadata: Metadata,
    pub r#type: String,
    pub created_at: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Metadata {
    pub app: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub desc: Option<String>,
    pub original_title: Option<String>,
    pub original_url: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Ocr {
    pub locale: String,
    pub description: String,
}

impl Gyazo {
    // TODO: test, docs
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

    // TODO: test, docs
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
