use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub address: String,
    pub address2: String,
    pub city: String,
    pub country: String,
    pub from: serde_json::Value,
    pub nickname: String,
    pub reply_to: serde_json::Value,
    pub state: String,
    pub zip: String,
}
impl<'a> PostSendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SenderId> {
        let mut r = self.client.client.post("/v3/senders");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "address" : self.address }));
        r = r.push_json(json!({ "address_2" : self.address2 }));
        r = r.push_json(json!({ "city" : self.city }));
        r = r.push_json(json!({ "country" : self.country }));
        r = r.push_json(json!({ "from" : self.from }));
        r = r.push_json(json!({ "nickname" : self.nickname }));
        r = r.push_json(json!({ "reply_to" : self.reply_to }));
        r = r.push_json(json!({ "state" : self.state }));
        r = r.push_json(json!({ "zip" : self.zip }));
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
}
pub struct PostSendersRequired<'a> {
    pub address: &'a str,
    pub address2: &'a str,
    pub city: &'a str,
    pub country: &'a str,
    pub from: serde_json::Value,
    pub nickname: &'a str,
    pub reply_to: serde_json::Value,
    pub state: &'a str,
    pub zip: &'a str,
}
impl<'a> PostSendersRequired<'a> {}
