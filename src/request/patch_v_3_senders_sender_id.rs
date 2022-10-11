use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchV3SendersSenderIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub sender_id: i64,
    pub address: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub from: Option<serde_json::Value>,
    pub nickname: Option<String>,
    pub reply_to: Option<serde_json::Value>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl<'a> PatchV3SendersSenderIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SenderId> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/senders/{sender_id}", sender_id = self.sender_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address2 {
            r = r.push_json(json!({ "address_2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.city {
            r = r.push_json(json!({ "city" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country {
            r = r.push_json(json!({ "country" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.from {
            r = r.push_json(json!({ "from" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.nickname {
            r = r.push_json(json!({ "nickname" : unwrapped }));
        }
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
    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_owned());
        self
    }
    pub fn address2(mut self, address2: &str) -> Self {
        self.address2 = Some(address2.to_owned());
        self
    }
    pub fn city(mut self, city: &str) -> Self {
        self.city = Some(city.to_owned());
        self
    }
    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }
    pub fn from(mut self, from: serde_json::Value) -> Self {
        self.from = Some(from);
        self
    }
    pub fn nickname(mut self, nickname: &str) -> Self {
        self.nickname = Some(nickname.to_owned());
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
