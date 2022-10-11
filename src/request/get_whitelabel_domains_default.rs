use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelDomainsDefaultRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub domain: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetWhitelabelDomainsDefaultRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthentication200Response> {
        let mut r = self.client.client.get("/v3/whitelabel/domains/default");
        if let Some(ref unwrapped) = self.domain {
            r = r.push_query("domain", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}