use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetallAutomationStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub automation_ids: Option<Vec<String>>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
}
impl<'a> GetallAutomationStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationsResponse> {
        let mut r = self.client.client.get("/v3/marketing/stats/automations");
        if let Some(ref unwrapped) = self.automation_ids {
            for item in unwrapped {
                r = r.push_query("automation_ids[]", &item.to_string());
            }
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
    pub fn automation_ids(
        mut self,
        automation_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .automation_ids = Some(
            automation_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
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
