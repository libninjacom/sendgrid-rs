use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingSinglesendsSearchRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub categories: Option<Vec<String>>,
    pub name: Option<String>,
    pub status: Option<Vec<String>>,
}
impl<'a> PostMarketingSinglesendsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/singlesends/search");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.categories {
            r = r.push_json(json!({ "categories" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_json(json!({ "status" : unwrapped }));
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
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
    pub fn categories(
        mut self,
        categories: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .categories = Some(
            categories.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn status(mut self, status: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.status = Some(status.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
