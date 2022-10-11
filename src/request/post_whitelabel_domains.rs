use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelDomainsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub automatic_security: Option<bool>,
    pub custom_dkim_selector: Option<String>,
    pub custom_spf: Option<bool>,
    pub default: Option<bool>,
    pub domain: String,
    pub ips: Option<Vec<String>>,
    pub subdomain: Option<String>,
    pub username: Option<String>,
}
impl<'a> PostWhitelabelDomainsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AuthenticationDomain> {
        let mut r = self.client.client.post("/v3/whitelabel/domains");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.automatic_security {
            r = r.push_json(json!({ "automatic_security" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_dkim_selector {
            r = r.push_json(json!({ "custom_dkim_selector" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_spf {
            r = r.push_json(json!({ "custom_spf" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.default {
            r = r.push_json(json!({ "default" : unwrapped }));
        }
        r = r.push_json(json!({ "domain" : self.domain }));
        if let Some(ref unwrapped) = self.ips {
            r = r.push_json(json!({ "ips" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subdomain {
            r = r.push_json(json!({ "subdomain" : unwrapped }));
        }
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn automatic_security(mut self, automatic_security: bool) -> Self {
        self.automatic_security = Some(automatic_security);
        self
    }
    pub fn custom_dkim_selector(mut self, custom_dkim_selector: &str) -> Self {
        self.custom_dkim_selector = Some(custom_dkim_selector.to_owned());
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
    pub fn ips(mut self, ips: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ips = Some(ips.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn subdomain(mut self, subdomain: &str) -> Self {
        self.subdomain = Some(subdomain.to_owned());
        self
    }
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
        self
    }
}
