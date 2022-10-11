use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSsoCertificatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub enabled: Option<bool>,
    pub integration_id: String,
    pub public_certificate: String,
}
impl<'a> PostSsoCertificatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoCertificateBody> {
        let mut r = self.client.client.post("/v3/sso/certificates");
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        r = r.push_json(json!({ "integration_id" : self.integration_id }));
        r = r.push_json(json!({ "public_certificate" : self.public_certificate }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}
