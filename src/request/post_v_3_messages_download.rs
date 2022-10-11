use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostV3MessagesDownloadRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub query: Option<String>,
    pub authorization: String,
}
impl<'a> PostV3MessagesDownloadRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/messages/download");
        if let Some(ref unwrapped) = self.query {
            r = r.push_query("query", &unwrapped.to_string());
        }
        r = r.header("Authorization", &self.authorization.to_string());
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
    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_owned());
        self
    }
}
