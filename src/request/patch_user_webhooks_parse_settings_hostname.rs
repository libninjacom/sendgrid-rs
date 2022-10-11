use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserWebhooksParseSettingsHostnameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub hostname: String,
    pub send_raw: Option<bool>,
    pub spam_check: Option<bool>,
    pub url: Option<String>,
}
impl<'a> PatchUserWebhooksParseSettingsHostnameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ParseSetting> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/user/webhooks/parse/settings/{hostname}", hostname = self
                    .hostname
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.send_raw {
            r = r.push_json(json!({ "send_raw" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.spam_check {
            r = r.push_json(json!({ "spam_check" : unwrapped }));
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
    pub fn send_raw(mut self, send_raw: bool) -> Self {
        self.send_raw = Some(send_raw);
        self
    }
    pub fn spam_check(mut self, spam_check: bool) -> Self {
        self.spam_check = Some(spam_check);
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
}
