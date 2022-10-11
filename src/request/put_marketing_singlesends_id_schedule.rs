use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutMarketingSinglesendsIdScheduleRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub send_at: String,
}
impl<'a> PutMarketingSinglesendsIdScheduleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(&format!("/v3/marketing/singlesends/{id}/schedule", id = self.id));
        r = r.push_json(json!({ "send_at" : self.send_at }));
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
