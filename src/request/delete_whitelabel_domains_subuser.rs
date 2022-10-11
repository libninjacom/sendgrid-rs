use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteWhitelabelDomainsSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: Option<String>,
}
impl<'a> DeleteWhitelabelDomainsSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/whitelabel/domains/subuser");
        if let Some(ref unwrapped) = self.username {
            r = r.push_query("username", &unwrapped.to_string());
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
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
        self
    }
}
