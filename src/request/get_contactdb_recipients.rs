use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbRecipientsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbRecipientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/contactdb/recipients");
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
