use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserWebhooksEventSettingsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub bounce: bool,
    pub click: bool,
    pub deferred: bool,
    pub delivered: bool,
    pub dropped: bool,
    pub enabled: bool,
    pub group_resubscribe: bool,
    pub group_unsubscribe: bool,
    pub oauth_client_id: Option<String>,
    pub oauth_client_secret: Option<String>,
    pub oauth_token_url: Option<String>,
    pub open: bool,
    pub processed: bool,
    pub spam_report: bool,
    pub unsubscribe: bool,
    pub url: String,
}
impl<'a> PatchUserWebhooksEventSettingsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EventWebhookResponse> {
        let mut r = self.client.client.patch("/v3/user/webhooks/event/settings");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "bounce" : self.bounce }));
        r = r.push_json(json!({ "click" : self.click }));
        r = r.push_json(json!({ "deferred" : self.deferred }));
        r = r.push_json(json!({ "delivered" : self.delivered }));
        r = r.push_json(json!({ "dropped" : self.dropped }));
        r = r.push_json(json!({ "enabled" : self.enabled }));
        r = r.push_json(json!({ "group_resubscribe" : self.group_resubscribe }));
        r = r.push_json(json!({ "group_unsubscribe" : self.group_unsubscribe }));
        if let Some(ref unwrapped) = self.oauth_client_id {
            r = r.push_json(json!({ "oauth_client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.oauth_client_secret {
            r = r.push_json(json!({ "oauth_client_secret" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.oauth_token_url {
            r = r.push_json(json!({ "oauth_token_url" : unwrapped }));
        }
        r = r.push_json(json!({ "open" : self.open }));
        r = r.push_json(json!({ "processed" : self.processed }));
        r = r.push_json(json!({ "spam_report" : self.spam_report }));
        r = r.push_json(json!({ "unsubscribe" : self.unsubscribe }));
        r = r.push_json(json!({ "url" : self.url }));
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
}
pub struct PatchUserWebhooksEventSettingsRequired<'a> {
    pub bounce: bool,
    pub click: bool,
    pub deferred: bool,
    pub delivered: bool,
    pub dropped: bool,
    pub enabled: bool,
    pub group_resubscribe: bool,
    pub group_unsubscribe: bool,
    pub open: bool,
    pub processed: bool,
    pub spam_report: bool,
    pub unsubscribe: bool,
    pub url: &'a str,
}
impl<'a> PatchUserWebhooksEventSettingsRequired<'a> {}
