use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAutomationStatRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub group_by: Option<Vec<String>>,
    pub step_ids: Option<Vec<String>>,
    pub aggregated_by: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub timezone: Option<String>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub id: String,
}
impl<'a> GetAutomationStatRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationsResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/stats/automations/{id}", id = self.id));
        if let Some(ref unwrapped) = self.group_by {
            for item in unwrapped {
                r = r.push_query("group_by[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.step_ids {
            for item in unwrapped {
                r = r.push_query("step_ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_query("start_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.timezone {
            r = r.push_query("timezone", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
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
    pub fn group_by(
        mut self,
        group_by: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .group_by = Some(
            group_by.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn step_ids(
        mut self,
        step_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .step_ids = Some(
            step_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_owned());
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
}
