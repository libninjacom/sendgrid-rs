use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostIpsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub count: i64,
    pub subusers: Option<Vec<String>>,
    pub warmup: Option<bool>,
}
impl<'a> PostIpsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/ips");
        r = r.push_json(json!({ "count" : self.count }));
        if let Some(ref unwrapped) = self.subusers {
            r = r.push_json(json!({ "subusers" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.warmup {
            r = r.push_json(json!({ "warmup" : unwrapped }));
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
    pub fn subusers(
        mut self,
        subusers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .subusers = Some(
            subusers.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn warmup(mut self, warmup: bool) -> Self {
        self.warmup = Some(warmup);
        self
    }
}
