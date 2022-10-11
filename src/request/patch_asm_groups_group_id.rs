use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchAsmGroupsGroupIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub group_id: String,
    pub description: String,
    pub is_default: bool,
    pub name: String,
}
impl<'a> PatchAsmGroupsGroupIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SuppressionGroup> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/asm/groups/{group_id}", group_id = self.group_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "description" : self.description }));
        r = r.push_json(json!({ "is_default" : self.is_default }));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
pub struct PatchAsmGroupsGroupIdRequired<'a> {
    pub group_id: &'a str,
    pub description: &'a str,
    pub is_default: bool,
    pub name: &'a str,
}
impl<'a> PatchAsmGroupsGroupIdRequired<'a> {}
