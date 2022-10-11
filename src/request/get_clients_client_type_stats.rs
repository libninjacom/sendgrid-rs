use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetClientsClientTypeStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub aggregated_by: Option<String>,
    pub client_type: String,
}
impl<'a> GetClientsClientTypeStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/clients/{client_type}/stats", client_type = self.client_type
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
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
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
}
