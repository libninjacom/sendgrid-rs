use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteContactdbListsListIdRecipientsRecipientIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub list_id: i64,
    pub recipient_id: i64,
    pub on_behalf_of: Option<String>,
}
impl<'a> DeleteContactdbListsListIdRecipientsRecipientIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete("/v3/contactdb/lists/{list_id}/recipients/{recipient_id}");
        r = r.push_query("list_id", &self.list_id.to_string());
        r = r.push_query("recipient_id", &self.recipient_id.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
