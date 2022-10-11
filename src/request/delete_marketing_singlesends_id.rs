use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMarketingSinglesendsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> DeleteMarketingSinglesendsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/marketing/singlesends/{id}", id = self.id));
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
}
