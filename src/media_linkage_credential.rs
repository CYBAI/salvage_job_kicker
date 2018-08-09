#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaLinkageCredential {
    pub id: String,
    pub vault_path: String,
    pub company_id: String,
    pub media: String,
}