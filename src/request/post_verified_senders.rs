use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostVerifiedSendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub address: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub from_email: String,
    pub from_name: Option<String>,
    pub nickname: String,
    pub reply_to: String,
    pub reply_to_name: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl<'a> PostVerifiedSendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<VerifiedSenderResponseSchema> {
        let mut r = self.client.client.post("/v3/verified_senders");
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address2 {
            r = r.push_json(json!({ "address2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.city {
            r = r.push_json(json!({ "city" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country {
            r = r.push_json(json!({ "country" : unwrapped }));
        }
        r = r.push_json(json!({ "from_email" : self.from_email }));
        if let Some(ref unwrapped) = self.from_name {
            r = r.push_json(json!({ "from_name" : unwrapped }));
        }
        r = r.push_json(json!({ "nickname" : self.nickname }));
        r = r.push_json(json!({ "reply_to" : self.reply_to }));
        if let Some(ref unwrapped) = self.reply_to_name {
            r = r.push_json(json!({ "reply_to_name" : unwrapped }));
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
    pub fn from_name(mut self, from_name: &str) -> Self {
        self.from_name = Some(from_name.to_owned());
        self
    }
    pub fn reply_to_name(mut self, reply_to_name: &str) -> Self {
        self.reply_to_name = Some(reply_to_name.to_owned());
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
