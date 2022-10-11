use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbListsListIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub list_id: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbListsListIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbList> {
        let mut r = self.client.client.get("/v3/contactdb/lists/{list_id}");
        if let Some(ref unwrapped) = self.list_id {
            r = r.push_query("list_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn list_id(mut self, list_id: i64) -> Self {
        self.list_id = Some(list_id);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
