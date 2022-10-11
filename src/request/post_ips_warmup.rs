use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostIpsWarmupRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ip: Option<String>,
}
impl<'a> PostIpsWarmupRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpWarmupResponse> {
        let mut r = self.client.client.post("/v3/ips/warmup");
        if let Some(ref unwrapped) = self.ip {
            r = r.push_json(json!({ "ip" : unwrapped }));
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
    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = Some(ip.to_owned());
        self
    }
}
