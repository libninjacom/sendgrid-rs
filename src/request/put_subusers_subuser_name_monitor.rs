use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutSubusersSubuserNameMonitorRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
    pub email: String,
    pub frequency: f64,
}
impl<'a> PutSubusersSubuserNameMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Monitor> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/v3/subusers/{subuser_name}/monitor", subuser_name = self
                    .subuser_name
                ),
            );
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "frequency" : self.frequency }));
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
