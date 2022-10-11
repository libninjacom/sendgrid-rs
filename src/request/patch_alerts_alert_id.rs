use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchAlertsAlertIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub alert_id: i64,
    pub email_to: Option<String>,
    pub frequency: Option<String>,
    pub percentage: Option<i64>,
}
impl<'a> PatchAlertsAlertIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/alerts/{alert_id}", alert_id = self.alert_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email_to {
            r = r.push_json(json!({ "email_to" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.frequency {
            r = r.push_json(json!({ "frequency" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.percentage {
            r = r.push_json(json!({ "percentage" : unwrapped }));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn email_to(mut self, email_to: &str) -> Self {
        self.email_to = Some(email_to.to_owned());
        self
    }
    pub fn frequency(mut self, frequency: &str) -> Self {
        self.frequency = Some(frequency.to_owned());
        self
    }
    pub fn percentage(mut self, percentage: i64) -> Self {
        self.percentage = Some(percentage);
        self
    }
}
