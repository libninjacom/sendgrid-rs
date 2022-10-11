use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub query_json: Option<bool>,
    pub segment_id: String,
}
impl<'a> GetMarketingSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FullSegment> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/marketing/segments/{segment_id}", segment_id = self.segment_id
                ),
            );
        if let Some(ref unwrapped) = self.query_json {
            r = r.push_query("query_json", &unwrapped.to_string());
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
    pub fn query_json(mut self, query_json: bool) -> Self {
        self.query_json = Some(query_json);
        self
    }
}
