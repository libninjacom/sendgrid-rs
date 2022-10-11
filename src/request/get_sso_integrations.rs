use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSsoIntegrationsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub si: Option<bool>,
}
impl<'a> GetSsoIntegrationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<SsoIntegration>> {
        let mut r = self.client.client.get("/v3/sso/integrations");
        if let Some(ref unwrapped) = self.si {
            r = r.push_query("si", &unwrapped.to_string());
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
    pub fn si(mut self, si: bool) -> Self {
        self.si = Some(si);
        self
    }
}
