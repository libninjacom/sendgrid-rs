use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSubusersSubuserNameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
    pub disabled: Option<bool>,
}
impl<'a> PatchSubusersSubuserNameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(
                &format!("/v3/subusers/{subuser_name}", subuser_name = self.subuser_name),
            );
        if let Some(ref unwrapped) = self.disabled {
            r = r.push_json(json!({ "disabled" : unwrapped }));
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
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }
}
