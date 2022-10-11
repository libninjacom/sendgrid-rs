use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelLinksLinkIdSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub link_id: i64,
    pub username: Option<String>,
}
impl<'a> PostWhitelabelLinksLinkIdSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkBranding200Response> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/whitelabel/links/{link_id}/subuser", link_id = self.link_id
                ),
            );
        if let Some(ref unwrapped) = self.username {
            r = r.push_json(json!({ "username" : unwrapped }));
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
