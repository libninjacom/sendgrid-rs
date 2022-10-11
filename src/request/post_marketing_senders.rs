use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingSendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub address: String,
    pub address2: Option<String>,
    pub city: String,
    pub country: String,
    pub from: serde_json::Value,
    pub nickname: String,
    pub reply_to: Option<serde_json::Value>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl<'a> PostMarketingSendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SenderId> {
        let mut r = self.client.client.post("/v3/marketing/senders");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "address" : self.address }));
        if let Some(ref unwrapped) = self.address2 {
            r = r.push_json(json!({ "address_2" : unwrapped }));
        }
        r = r.push_json(json!({ "city" : self.city }));
        r = r.push_json(json!({ "country" : self.country }));
        r = r.push_json(json!({ "from" : self.from }));
        r = r.push_json(json!({ "nickname" : self.nickname }));
        if let Some(ref unwrapped) = self.reply_to {
            r = r.push_json(json!({ "reply_to" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.state {
            r = r.push_json(json!({ "state" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.zip {
            r = r.push_json(json!({ "zip" : unwrapped }));
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
    pub fn address2(mut self, address2: &str) -> Self {
        self.address2 = Some(address2.to_owned());
        self
    }
    pub fn reply_to(mut self, reply_to: serde_json::Value) -> Self {
        self.reply_to = Some(reply_to);
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn zip(mut self, zip: &str) -> Self {
        self.zip = Some(zip.to_owned());
        self
    }
}
pub struct PostMarketingSendersRequired<'a> {
    pub address: &'a str,
    pub city: &'a str,
    pub country: &'a str,
    pub from: serde_json::Value,
    pub nickname: &'a str,
}
impl<'a> PostMarketingSendersRequired<'a> {}
