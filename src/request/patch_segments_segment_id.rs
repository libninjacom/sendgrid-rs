use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: String,
    pub name: Option<String>,
    pub query_dsl: Option<String>,
}
impl<'a> PatchSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SegmentResponse> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/marketing/segments/2.0/{segment_id}", segment_id = self
                    .segment_id
                ),
            );
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.query_dsl {
            r = r.push_json(json!({ "query_dsl" : unwrapped }));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn query_dsl(mut self, query_dsl: &str) -> Self {
        self.query_dsl = Some(query_dsl.to_owned());
        self
    }
}
