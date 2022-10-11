use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostValidationsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub email: String,
    pub source: Option<String>,
}
impl<'a> PostValidationsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/validations/email");
        r = r.push_json(json!({ "email" : self.email }));
        if let Some(ref unwrapped) = self.source {
            r = r.push_json(json!({ "source" : unwrapped }));
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
    pub fn source(mut self, source: &str) -> Self {
        self.source = Some(source.to_owned());
        self
    }
}
