use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingTestSendEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub custom_unsubscribe_url: Option<String>,
    pub emails: Vec<String>,
    pub from_address: Option<String>,
    pub sender_id: Option<i64>,
    pub suppression_group_id: Option<i64>,
    pub template_id: String,
    pub version_id_override: Option<String>,
}
impl<'a> PostMarketingTestSendEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/test/send_email");
        if let Some(ref unwrapped) = self.custom_unsubscribe_url {
            r = r.push_json(json!({ "custom_unsubscribe_url" : unwrapped }));
        }
        r = r.push_json(json!({ "emails" : self.emails }));
        if let Some(ref unwrapped) = self.from_address {
            r = r.push_json(json!({ "from_address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.sender_id {
            r = r.push_json(json!({ "sender_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.suppression_group_id {
            r = r.push_json(json!({ "suppression_group_id" : unwrapped }));
        }
        r = r.push_json(json!({ "template_id" : self.template_id }));
        if let Some(ref unwrapped) = self.version_id_override {
            r = r.push_json(json!({ "version_id_override" : unwrapped }));
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
    pub fn custom_unsubscribe_url(mut self, custom_unsubscribe_url: &str) -> Self {
        self.custom_unsubscribe_url = Some(custom_unsubscribe_url.to_owned());
        self
    }
    pub fn from_address(mut self, from_address: &str) -> Self {
        self.from_address = Some(from_address.to_owned());
        self
    }
    pub fn sender_id(mut self, sender_id: i64) -> Self {
        self.sender_id = Some(sender_id);
        self
    }
    pub fn suppression_group_id(mut self, suppression_group_id: i64) -> Self {
        self.suppression_group_id = Some(suppression_group_id);
        self
    }
    pub fn version_id_override(mut self, version_id_override: &str) -> Self {
        self.version_id_override = Some(version_id_override.to_owned());
        self
    }
}
