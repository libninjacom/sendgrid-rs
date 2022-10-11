use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelDnsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub domain_id: i64,
    pub email: String,
    pub link_id: i64,
    pub message: Option<String>,
}
impl<'a> PostWhitelabelDnsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/v3/whitelabel/dns/email");
        r = r.push_json(json!({ "domain_id" : self.domain_id }));
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "link_id" : self.link_id }));
        if let Some(ref unwrapped) = self.message {
            r = r.push_json(json!({ "message" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_owned());
        self
    }
}
