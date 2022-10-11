use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSsoTeammatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub email: String,
    pub first_name: String,
    pub is_admin: bool,
    pub is_read_only: bool,
    pub last_name: String,
    pub scopes: Vec<String>,
}
impl<'a> PostSsoTeammatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoTeammateResponse> {
        let mut r = self.client.client.post("/v3/sso/teammates");
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "first_name" : self.first_name }));
        r = r.push_json(json!({ "is_admin" : self.is_admin }));
        r = r.push_json(json!({ "is_read_only" : self.is_read_only }));
        r = r.push_json(json!({ "last_name" : self.last_name }));
        r = r.push_json(json!({ "scopes" : self.scopes }));
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
pub struct PostSsoTeammatesRequired<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub is_admin: bool,
    pub is_read_only: bool,
    pub last_name: &'a str,
    pub scopes: &'a [&'a str],
}
impl<'a> PostSsoTeammatesRequired<'a> {}
