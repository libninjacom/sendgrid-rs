use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMarketingSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: String,
    pub name: String,
    pub parent_list_ids: Option<Vec<String>>,
    pub query_dsl: String,
}
impl<'a> PatchMarketingSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FullSegment> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/marketing/segments/{segment_id}", segment_id = self.segment_id
                ),
            );
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.parent_list_ids {
            r = r.push_json(json!({ "parent_list_ids" : unwrapped }));
        }
        r = r.push_json(json!({ "query_dsl" : self.query_dsl }));
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
    pub fn parent_list_ids(
        mut self,
        parent_list_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .parent_list_ids = Some(
            parent_list_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
