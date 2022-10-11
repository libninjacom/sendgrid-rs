use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersReputationsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub usernames: Option<String>,
}
impl<'a> GetSubusersReputationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/subusers/reputations");
        if let Some(ref unwrapped) = self.usernames {
            r = r.push_query("usernames", &unwrapped.to_string());
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
    pub fn usernames(mut self, usernames: &str) -> Self {
        self.usernames = Some(usernames.to_owned());
        self
    }
}
