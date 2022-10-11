use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelLinksRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub default: Option<bool>,
    pub domain: String,
    pub subdomain: Option<String>,
}
impl<'a> PostWhitelabelLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkBranding200Response> {
        let mut r = self.client.client.post("/v3/whitelabel/links");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.default {
            r = r.push_json(json!({ "default" : unwrapped }));
        }
        r = r.push_json(json!({ "domain" : self.domain }));
        if let Some(ref unwrapped) = self.subdomain {
            r = r.push_json(json!({ "subdomain" : unwrapped }));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
    pub fn subdomain(mut self, subdomain: &str) -> Self {
        self.subdomain = Some(subdomain.to_owned());
        self
    }
}
