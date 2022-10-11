use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSinglesendStatsExportRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ids: Option<Vec<String>>,
    pub timezone: Option<String>,
}
impl<'a> GetSinglesendStatsExportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<String> {
        let mut r = self.client.client.get("/v3/marketing/stats/singlesends/export");
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.timezone {
            r = r.push_query("timezone", &unwrapped.to_string());
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
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_owned());
        self
    }
}
