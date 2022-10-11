use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelDomainsSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: String,
}
impl<'a> GetWhitelabelDomainsSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthenticationDomainSpf> {
        let mut r = self.client.client.get("/v3/whitelabel/domains/subuser");
        r = r.push_query("username", &self.username.to_string());
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