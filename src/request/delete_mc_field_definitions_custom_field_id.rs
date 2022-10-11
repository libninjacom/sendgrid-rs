use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMcFieldDefinitionsCustomFieldIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub custom_field_id: String,
}
impl<'a> DeleteMcFieldDefinitionsCustomFieldIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/marketing/field_definitions/{custom_field_id}", custom_field_id
                    = self.custom_field_id
                ),
            );
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
