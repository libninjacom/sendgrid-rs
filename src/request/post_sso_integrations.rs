use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSsoIntegrationsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub completed_integration: Option<bool>,
    pub enabled: bool,
    pub entity_id: String,
    pub name: String,
    pub signin_url: String,
    pub signout_url: String,
}
impl<'a> PostSsoIntegrationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoIntegration> {
        let mut r = self.client.client.post("/v3/sso/integrations");
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
    pub fn completed_integration(mut self, completed_integration: bool) -> Self {
        self.completed_integration = Some(completed_integration);
        self
    }
}
pub struct PostSsoIntegrationsRequired<'a> {
    pub enabled: bool,
    pub entity_id: &'a str,
    pub name: &'a str,
    pub signin_url: &'a str,
    pub signout_url: &'a str,
}
impl<'a> PostSsoIntegrationsRequired<'a> {}
