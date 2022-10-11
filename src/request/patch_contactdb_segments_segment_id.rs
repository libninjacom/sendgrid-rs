use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchContactdbSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: Option<String>,
    pub on_behalf_of: Option<String>,
    pub conditions: Option<Vec<ContactdbSegmentsConditions>>,
    pub list_id: Option<f64>,
    pub name: String,
}
impl<'a> PatchContactdbSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbSegments> {
        let mut r = self.client.client.patch("/v3/contactdb/segments/{segment_id}");
        if let Some(ref unwrapped) = self.segment_id {
            r = r.push_query("segment_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.conditions {
            r = r.push_json(json!({ "conditions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.list_id {
            r = r.push_json(json!({ "list_id" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
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
    pub fn segment_id(mut self, segment_id: &str) -> Self {
        self.segment_id = Some(segment_id.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn conditions(mut self, conditions: Vec<ContactdbSegmentsConditions>) -> Self {
        self.conditions = Some(conditions);
        self
    }
    pub fn list_id(mut self, list_id: f64) -> Self {
        self.list_id = Some(list_id);
        self
    }
}
