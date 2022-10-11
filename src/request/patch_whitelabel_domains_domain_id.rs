use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchWhitelabelDomainsDomainIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub domain_id: String,
    pub custom_spf: Option<bool>,
    pub default: Option<bool>,
}
impl<'a> PatchWhitelabelDomainsDomainIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthentication200Response> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/whitelabel/domains/{domain_id}", domain_id = self.domain_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.custom_spf {
            r = r.push_json(json!({ "custom_spf" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.default {
            r = r.push_json(json!({ "default" : unwrapped }));
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
    pub fn custom_spf(mut self, custom_spf: bool) -> Self {
        self.custom_spf = Some(custom_spf);
        self
    }
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
}
