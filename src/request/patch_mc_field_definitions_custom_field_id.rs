use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMcFieldDefinitionsCustomFieldIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub custom_field_id: String,
    pub name: String,
}
impl<'a> PatchMcFieldDefinitionsCustomFieldIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/marketing/field_definitions/{custom_field_id}", custom_field_id
                    = self.custom_field_id
                ),
            );
        r = r.push_json(json!({ "name" : self.name }));
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
