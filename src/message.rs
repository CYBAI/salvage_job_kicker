use super::media_linkage_credential::MediaLinkageCredential;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message<'a> {
    pub media_linkage_id: String,
    pub vault_path: String,
    pub company_id: String,
    pub media: String,
    pub action: &'a str,
}

impl<'a> Message<'a> {
    pub fn from_media_linkage_credential(cred: MediaLinkageCredential, action: &'a str) -> Self {
        return Message {
            media_linkage_id: cred.id,
            vault_path: cred.vault_path,
            company_id: cred.company_id,
            media: cred.media,
            action: action,
        };
    }
}
