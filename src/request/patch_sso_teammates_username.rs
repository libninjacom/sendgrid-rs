use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSsoTeammatesUsernameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: String,
    pub first_name: Option<String>,
    pub is_admin: Option<bool>,
    pub last_name: Option<String>,
    pub scopes: Option<Vec<String>>,
}
impl<'a> PatchSsoTeammatesUsernameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoTeammatesPatchResponse> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/sso/teammates/{username}", username = self.username));
        if let Some(ref unwrapped) = self.first_name {
            r = r.push_json(json!({ "first_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_admin {
            r = r.push_json(json!({ "is_admin" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.last_name {
            r = r.push_json(json!({ "last_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.scopes {
            r = r.push_json(json!({ "scopes" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }
    pub fn is_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = Some(is_admin);
        self
    }
    pub fn last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
        self
    }
    pub fn scopes(mut self, scopes: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.scopes = Some(scopes.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
