use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutMcContactsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub contacts: Vec<ContactRequest>,
    pub list_ids: Option<Vec<String>>,
}
impl<'a> PutMcContactsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.put("/v3/marketing/contacts");
        r = r.push_json(json!({ "contacts" : self.contacts }));
        if let Some(ref unwrapped) = self.list_ids {
            r = r.push_json(json!({ "list_ids" : unwrapped }));
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
    pub fn list_ids(
        mut self,
        list_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .list_ids = Some(
            list_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
