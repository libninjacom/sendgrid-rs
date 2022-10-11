use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcContatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetMcContatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/contacts");
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
