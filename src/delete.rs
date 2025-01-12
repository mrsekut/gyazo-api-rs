use super::Gyazo;

#[derive(serde::Deserialize, Debug)]
pub struct DeleteResponse {
    pub image_id: String,
    pub r#type: String,
}

impl Gyazo {
    // TODO: test, docs
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
