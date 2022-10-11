use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingContactsImportsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> GetMarketingContactsImportsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactImport> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/contacts/imports/{id}", id = self.id));
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
}
