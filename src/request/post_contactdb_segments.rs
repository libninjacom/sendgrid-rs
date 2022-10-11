use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbSegmentsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub conditions: Vec<ContactdbSegmentsConditions>,
    pub list_id: Option<i64>,
    pub name: String,
    pub recipient_count: Option<f64>,
}
impl<'a> PostContactdbSegmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbSegmentsWithId> {
        let mut r = self.client.client.post("/v3/contactdb/segments");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "conditions" : self.conditions }));
        if let Some(ref unwrapped) = self.list_id {
            r = r.push_json(json!({ "list_id" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.recipient_count {
            r = r.push_json(json!({ "recipient_count" : unwrapped }));
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
    pub fn list_id(mut self, list_id: i64) -> Self {
        self.list_id = Some(list_id);
        self
    }
    pub fn recipient_count(mut self, recipient_count: f64) -> Self {
        self.recipient_count = Some(recipient_count);
        self
    }
}
