/// Access policy for the uploaded image.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AccessPolicy {
    /// The image is visible to anyone with the link.
    Anyone,
    /// The image is visible only to the uploader.
    OnlyMe,
}

impl AccessPolicy {
    /// Converts the enum into a string representation for the API.
    ///
    /// # Returns
    ///
    /// A string slice that represents the access policy.
    pub fn as_str(&self) -> &str {
        match self {
            AccessPolicy::Anyone => "anyone",
            AccessPolicy::OnlyMe => "onlyme",
        }
    }
}
