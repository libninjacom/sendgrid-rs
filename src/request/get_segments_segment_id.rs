use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub contacts_sample: Option<bool>,
    pub segment_id: String,
}
impl<'a> GetSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SegmentResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/marketing/segments/2.0/{segment_id}", segment_id = self
                    .segment_id
                ),
            );
        if let Some(ref unwrapped) = self.contacts_sample {
            r = r.push_query("contacts_sample", &unwrapped.to_string());
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
    pub fn contacts_sample(mut self, contacts_sample: bool) -> Self {
        self.contacts_sample = Some(contacts_sample);
        self
    }
}
