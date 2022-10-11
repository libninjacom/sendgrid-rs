use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMcContactsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub delete_all_contacts: Option<String>,
    pub ids: Option<String>,
}
impl<'a> DeleteMcContactsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/marketing/contacts");
        if let Some(ref unwrapped) = self.delete_all_contacts {
            r = r.push_query("delete_all_contacts", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
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
    pub fn delete_all_contacts(mut self, delete_all_contacts: &str) -> Self {
        self.delete_all_contacts = Some(delete_all_contacts.to_owned());
        self
    }
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
        self
    }
}
