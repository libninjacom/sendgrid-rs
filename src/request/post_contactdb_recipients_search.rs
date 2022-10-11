use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbRecipientsSearchRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub conditions: Vec<ContactdbSegmentsConditions>,
    pub list_id: i64,
}
impl<'a> PostContactdbRecipientsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/contactdb/recipients/search");
        r = r.push_json(json!({ "conditions" : self.conditions }));
        r = r.push_json(json!({ "list_id" : self.list_id }));
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
}
