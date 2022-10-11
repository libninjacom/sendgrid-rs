use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSsoCertificatesCertIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub cert_id: String,
    pub enabled: Option<bool>,
    pub integration_id: Option<String>,
    pub public_certificate: Option<String>,
}
impl<'a> PatchSsoCertificatesCertIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/sso/certificates/{cert_id}", cert_id = self.cert_id));
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.integration_id {
            r = r.push_json(json!({ "integration_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.public_certificate {
            r = r.push_json(json!({ "public_certificate" : unwrapped }));
        }
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn integration_id(mut self, integration_id: &str) -> Self {
        self.integration_id = Some(integration_id.to_owned());
        self
    }
    pub fn public_certificate(mut self, public_certificate: &str) -> Self {
        self.public_certificate = Some(public_certificate.to_owned());
        self
    }
}
