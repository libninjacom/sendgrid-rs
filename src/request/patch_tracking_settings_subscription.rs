use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchTrackingSettingsSubscriptionRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
    pub html_content: Option<String>,
    pub landing: Option<String>,
    pub plain_content: Option<String>,
    pub replace: Option<String>,
    pub url: Option<String>,
}
impl<'a> PatchTrackingSettingsSubscriptionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionTrackingSettings> {
        let mut r = self.client.client.patch("/v3/tracking_settings/subscription");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.html_content {
            r = r.push_json(json!({ "html_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.landing {
            r = r.push_json(json!({ "landing" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.plain_content {
            r = r.push_json(json!({ "plain_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.replace {
            r = r.push_json(json!({ "replace" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.url {
            r = r.push_json(json!({ "url" : unwrapped }));
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
    pub fn html_content(mut self, html_content: &str) -> Self {
        self.html_content = Some(html_content.to_owned());
        self
    }
    pub fn landing(mut self, landing: &str) -> Self {
        self.landing = Some(landing.to_owned());
        self
    }
    pub fn plain_content(mut self, plain_content: &str) -> Self {
        self.plain_content = Some(plain_content.to_owned());
        self
    }
    pub fn replace(mut self, replace: &str) -> Self {
        self.replace = Some(replace.to_owned());
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
}
