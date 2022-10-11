use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionBouncesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub delete_all: Option<bool>,
    pub emails: Option<Vec<String>>,
}
impl<'a> DeleteSuppressionBouncesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/suppression/bounces");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.delete_all {
            r = r.push_json(json!({ "delete_all" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.emails {
            r = r.push_json(json!({ "emails" : unwrapped }));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn delete_all(mut self, delete_all: bool) -> Self {
        self.delete_all = Some(delete_all);
        self
    }
    pub fn emails(mut self, emails: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.emails = Some(emails.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
