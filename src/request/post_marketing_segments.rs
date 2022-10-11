use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingSegmentsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub name: String,
    pub parent_list_ids: Vec<String>,
    pub query_dsl: String,
    pub parent_list_id: String,
}
impl<'a> PostMarketingSegmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FullSegment> {
        let mut r = self.client.client.post("/v3/marketing/segments");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "parent_list_ids" : self.parent_list_ids }));
        r = r.push_json(json!({ "query_dsl" : self.query_dsl }));
        r = r.push_json(json!({ "parent_list_id" : self.parent_list_id }));
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
}
pub struct PostMarketingSegmentsRequired<'a> {
    pub name: &'a str,
    pub parent_list_ids: &'a [&'a str],
    pub query_dsl: &'a str,
    pub parent_list_id: &'a str,
}
impl<'a> PostMarketingSegmentsRequired<'a> {}
