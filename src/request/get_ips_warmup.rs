use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsWarmupRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetIpsWarmupRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpWarmupResponse> {
        let mut r = self.client.client.get("/v3/ips/warmup");
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
