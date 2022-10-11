use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchCampaignsCampaignIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
    pub categories: Vec<String>,
    pub html_content: String,
    pub plain_content: String,
    pub subject: String,
    pub title: String,
}
impl<'a> PatchCampaignsCampaignIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CampaignResponse> {
        let mut r = self
            .client
            .client
            .patch(
                &format!("/v3/campaigns/{campaign_id}", campaign_id = self.campaign_id),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "categories" : self.categories }));
        r = r.push_json(json!({ "html_content" : self.html_content }));
        r = r.push_json(json!({ "plain_content" : self.plain_content }));
        r = r.push_json(json!({ "subject" : self.subject }));
        r = r.push_json(json!({ "title" : self.title }));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
pub struct PatchCampaignsCampaignIdRequired<'a> {
    pub campaign_id: i64,
    pub categories: &'a [&'a str],
    pub html_content: &'a str,
    pub plain_content: &'a str,
    pub subject: &'a str,
    pub title: &'a str,
}
impl<'a> PatchCampaignsCampaignIdRequired<'a> {}
