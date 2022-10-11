use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchPartnerSettingsNewRelicRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enable_subuser_statistics: Option<bool>,
    pub enabled: Option<bool>,
    pub license_key: Option<String>,
}
impl<'a> PatchPartnerSettingsNewRelicRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PartnerSettingsNewRelic> {
        let mut r = self.client.client.patch("/v3/partner_settings/new_relic");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enable_subuser_statistics {
            r = r.push_json(json!({ "enable_subuser_statistics" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.license_key {
            r = r.push_json(json!({ "license_key" : unwrapped }));
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
    pub fn enable_subuser_statistics(mut self, enable_subuser_statistics: bool) -> Self {
        self.enable_subuser_statistics = Some(enable_subuser_statistics);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn license_key(mut self, license_key: &str) -> Self {
        self.license_key = Some(license_key.to_owned());
        self
    }
}
