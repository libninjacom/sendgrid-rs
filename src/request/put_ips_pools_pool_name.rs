use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutIpsPoolsPoolNameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub pool_name: String,
    pub name: Option<String>,
}
impl<'a> PutIpsPoolsPoolNameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpPoolResponse> {
        let mut r = self
            .client
            .client
            .put(&format!("/v3/ips/pools/{pool_name}", pool_name = self.pool_name));
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
