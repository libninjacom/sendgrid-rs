use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserSettingsEnforcedTlsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub require_tls: Option<bool>,
    pub require_valid_cert: Option<bool>,
    pub version: Option<f64>,
}
impl<'a> PatchUserSettingsEnforcedTlsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EnforcedTlsRequestResponse> {
        let mut r = self.client.client.patch("/v3/user/settings/enforced_tls");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.require_tls {
            r = r.push_json(json!({ "require_tls" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.require_valid_cert {
            r = r.push_json(json!({ "require_valid_cert" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.version {
            r = r.push_json(json!({ "version" : unwrapped }));
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
    pub fn require_tls(mut self, require_tls: bool) -> Self {
        self.require_tls = Some(require_tls);
        self
    }
    pub fn require_valid_cert(mut self, require_valid_cert: bool) -> Self {
        self.require_valid_cert = Some(require_valid_cert);
        self
    }
    pub fn version(mut self, version: f64) -> Self {
        self.version = Some(version);
        self
    }
}
