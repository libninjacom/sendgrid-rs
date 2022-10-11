use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostUserWebhooksEventTestRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub oauth_client_id: Option<String>,
    pub oauth_client_secret: Option<String>,
    pub oauth_token_url: Option<String>,
    pub url: Option<String>,
}
impl<'a> PostUserWebhooksEventTestRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/v3/user/webhooks/event/test");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.oauth_client_id {
            r = r.push_json(json!({ "oauth_client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.oauth_client_secret {
            r = r.push_json(json!({ "oauth_client_secret" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.oauth_token_url {
            r = r.push_json(json!({ "oauth_token_url" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.url {
            r = r.push_json(json!({ "url" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn oauth_client_id(mut self, oauth_client_id: &str) -> Self {
        self.oauth_client_id = Some(oauth_client_id.to_owned());
        self
    }
    pub fn oauth_client_secret(mut self, oauth_client_secret: &str) -> Self {
        self.oauth_client_secret = Some(oauth_client_secret.to_owned());
        self
    }
    pub fn oauth_token_url(mut self, oauth_token_url: &str) -> Self {
        self.oauth_token_url = Some(oauth_token_url.to_owned());
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
}
