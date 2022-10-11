use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub subusers: String,
    pub start_date: String,
    pub end_date: Option<String>,
}
impl<'a> GetSubusersStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<CategoryStats>> {
        let mut r = self.client.client.get("/v3/subusers/stats");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        r = r.push_query("subusers", &self.subusers.to_string());
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
}
