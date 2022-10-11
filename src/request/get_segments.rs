use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSegmentsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ids: Option<Vec<serde_json::Value>>,
    pub parent_list_ids: Option<String>,
    pub no_parent_list_id: Option<bool>,
}
impl<'a> GetSegmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AllSegmentsResponse> {
        let mut r = self.client.client.get("/v3/marketing/segments/2.0");
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.parent_list_ids {
            r = r.push_query("parent_list_ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.no_parent_list_id {
            r = r.push_query("no_parent_list_id", &unwrapped.to_string());
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
    pub fn ids(mut self, ids: Vec<serde_json::Value>) -> Self {
        self.ids = Some(ids);
        self
    }
    pub fn parent_list_ids(mut self, parent_list_ids: &str) -> Self {
        self.parent_list_ids = Some(parent_list_ids.to_owned());
        self
    }
    pub fn no_parent_list_id(mut self, no_parent_list_id: bool) -> Self {
        self.no_parent_list_id = Some(no_parent_list_id);
        self
    }
}
