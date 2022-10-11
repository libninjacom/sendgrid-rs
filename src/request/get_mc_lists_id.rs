use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcListsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub contact_sample: Option<bool>,
    pub id: String,
}
impl<'a> GetMcListsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/lists/{id}", id = self.id));
        if let Some(ref unwrapped) = self.contact_sample {
            r = r.push_query("contact_sample", &unwrapped.to_string());
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
    pub fn contact_sample(mut self, contact_sample: bool) -> Self {
        self.contact_sample = Some(contact_sample);
        self
    }
}
