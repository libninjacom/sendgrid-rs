use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSubusersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub email: String,
    pub ips: Vec<String>,
    pub password: String,
    pub username: String,
}
impl<'a> PostSubusersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubuserPost> {
        let mut r = self.client.client.post("/v3/subusers");
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "ips" : self.ips }));
        r = r.push_json(json!({ "password" : self.password }));
        r = r.push_json(json!({ "username" : self.username }));
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
pub struct PostSubusersRequired<'a> {
    pub email: &'a str,
    pub ips: &'a [&'a str],
    pub password: &'a str,
    pub username: &'a str,
}
impl<'a> PostSubusersRequired<'a> {}
