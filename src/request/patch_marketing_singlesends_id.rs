use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMarketingSinglesendsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub categories: Option<Vec<String>>,
    pub email_config: Option<serde_json::Value>,
    pub name: String,
    pub send_at: Option<String>,
    pub send_to: Option<serde_json::Value>,
}
impl<'a> PatchMarketingSinglesendsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendResponse> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/marketing/singlesends/{id}", id = self.id));
        if let Some(ref unwrapped) = self.categories {
            r = r.push_json(json!({ "categories" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email_config {
            r = r.push_json(json!({ "email_config" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.send_at {
            r = r.push_json(json!({ "send_at" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.send_to {
            r = r.push_json(json!({ "send_to" : unwrapped }));
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
    pub fn categories(
        mut self,
        categories: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .categories = Some(
            categories.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn email_config(mut self, email_config: serde_json::Value) -> Self {
        self.email_config = Some(email_config);
        self
    }
    pub fn send_at(mut self, send_at: &str) -> Self {
        self.send_at = Some(send_at.to_owned());
        self
    }
    pub fn send_to(mut self, send_to: serde_json::Value) -> Self {
        self.send_to = Some(send_to);
        self
    }
}
