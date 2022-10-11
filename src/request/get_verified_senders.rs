use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetVerifiedSendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<f64>,
    pub last_seen_id: Option<f64>,
    pub id: Option<i64>,
}
impl<'a> GetVerifiedSendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/verified_senders");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.last_seen_id {
            r = r.push_query("lastSeenID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.id {
            r = r.push_query("id", &unwrapped.to_string());
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
    pub fn limit(mut self, limit: f64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn last_seen_id(mut self, last_seen_id: f64) -> Self {
        self.last_seen_id = Some(last_seen_id);
        self
    }
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
}
