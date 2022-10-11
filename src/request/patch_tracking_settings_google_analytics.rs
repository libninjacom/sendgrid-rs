use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchTrackingSettingsGoogleAnalyticsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
    pub utm_campaign: Option<String>,
    pub utm_content: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_source: Option<String>,
    pub utm_term: Option<String>,
}
impl<'a> PatchTrackingSettingsGoogleAnalyticsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<GoogleAnalyticsSettings> {
        let mut r = self.client.client.patch("/v3/tracking_settings/google_analytics");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_campaign {
            r = r.push_json(json!({ "utm_campaign" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_content {
            r = r.push_json(json!({ "utm_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_medium {
            r = r.push_json(json!({ "utm_medium" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_source {
            r = r.push_json(json!({ "utm_source" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_term {
            r = r.push_json(json!({ "utm_term" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn utm_campaign(mut self, utm_campaign: &str) -> Self {
        self.utm_campaign = Some(utm_campaign.to_owned());
        self
    }
    pub fn utm_content(mut self, utm_content: &str) -> Self {
        self.utm_content = Some(utm_content.to_owned());
        self
    }
    pub fn utm_medium(mut self, utm_medium: &str) -> Self {
        self.utm_medium = Some(utm_medium.to_owned());
        self
    }
    pub fn utm_source(mut self, utm_source: &str) -> Self {
        self.utm_source = Some(utm_source.to_owned());
        self
    }
    pub fn utm_term(mut self, utm_term: &str) -> Self {
        self.utm_term = Some(utm_term.to_owned());
        self
    }
}
