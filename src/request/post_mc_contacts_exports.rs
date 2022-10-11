use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMcContactsExportsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub file_type: Option<String>,
    pub list_ids: Option<Vec<String>>,
    pub max_file_size: Option<i64>,
    pub notifications: Option<serde_json::Value>,
    pub segment_ids: Option<Vec<String>>,
}
impl<'a> PostMcContactsExportsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/contacts/exports");
        if let Some(ref unwrapped) = self.file_type {
            r = r.push_json(json!({ "file_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.list_ids {
            r = r.push_json(json!({ "list_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_file_size {
            r = r.push_json(json!({ "max_file_size" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.notifications {
            r = r.push_json(json!({ "notifications" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.segment_ids {
            r = r.push_json(json!({ "segment_ids" : unwrapped }));
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
    pub fn file_type(mut self, file_type: &str) -> Self {
        self.file_type = Some(file_type.to_owned());
        self
    }
    pub fn list_ids(
        mut self,
        list_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .list_ids = Some(
            list_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn max_file_size(mut self, max_file_size: i64) -> Self {
        self.max_file_size = Some(max_file_size);
        self
    }
    pub fn notifications(mut self, notifications: serde_json::Value) -> Self {
        self.notifications = Some(notifications);
        self
    }
    pub fn segment_ids(
        mut self,
        segment_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .segment_ids = Some(
            segment_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
