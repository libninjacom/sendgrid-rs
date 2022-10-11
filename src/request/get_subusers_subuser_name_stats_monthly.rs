use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersSubuserNameStatsMonthlyRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub date: String,
    pub sort_by_metric: Option<String>,
    pub sort_by_direction: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub subuser_name: String,
}
impl<'a> GetSubusersSubuserNameStatsMonthlyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubuserStats> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/subusers/{subuser_name}/stats/monthly", subuser_name = self
                    .subuser_name
                ),
            );
        r = r.push_query("date", &self.date.to_string());
        if let Some(ref unwrapped) = self.sort_by_metric {
            r = r.push_query("sort_by_metric", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_by_direction {
            r = r.push_query("sort_by_direction", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn sort_by_metric(mut self, sort_by_metric: &str) -> Self {
        self.sort_by_metric = Some(sort_by_metric.to_owned());
        self
    }
    pub fn sort_by_direction(mut self, sort_by_direction: &str) -> Self {
        self.sort_by_direction = Some(sort_by_direction.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}
