use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteListsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub delete_contacts: Option<bool>,
    pub id: String,
}
impl<'a> DeleteListsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/marketing/lists/{id}", id = self.id));
        if let Some(ref unwrapped) = self.delete_contacts {
            r = r.push_query("delete_contacts", &unwrapped.to_string());
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
    pub fn delete_contacts(mut self, delete_contacts: bool) -> Self {
        self.delete_contacts = Some(delete_contacts);
        self
    }
}
