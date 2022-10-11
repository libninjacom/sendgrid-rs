use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSsoIntegrationsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub si: Option<bool>,
    pub id: String,
    pub completed_integration: Option<bool>,
    pub enabled: bool,
    pub entity_id: String,
    pub name: String,
    pub signin_url: String,
    pub signout_url: String,
}
impl<'a> PatchSsoIntegrationsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoIntegration> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/sso/integrations/{id}", id = self.id));
        if let Some(ref unwrapped) = self.si {
            r = r.push_query("si", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.completed_integration {
            r = r.push_json(json!({ "completed_integration" : unwrapped }));
        }
        r = r.push_json(json!({ "enabled" : self.enabled }));
        r = r.push_json(json!({ "entity_id" : self.entity_id }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "signin_url" : self.signin_url }));
        r = r.push_json(json!({ "signout_url" : self.signout_url }));
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
    pub fn si(mut self, si: bool) -> Self {
        self.si = Some(si);
        self
    }
    pub fn completed_integration(mut self, completed_integration: bool) -> Self {
        self.completed_integration = Some(completed_integration);
        self
    }
}
pub struct PatchSsoIntegrationsIdRequired<'a> {
    pub id: &'a str,
    pub enabled: bool,
    pub entity_id: &'a str,
    pub name: &'a str,
    pub signin_url: &'a str,
    pub signout_url: &'a str,
}
impl<'a> PatchSsoIntegrationsIdRequired<'a> {}
