use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostCampaignsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub categories: Option<Vec<String>>,
    pub custom_unsubscribe_url: Option<String>,
    pub editor: Option<String>,
    pub html_content: Option<String>,
    pub ip_pool: Option<String>,
    pub list_ids: Option<Vec<i64>>,
    pub plain_content: Option<String>,
    pub segment_ids: Option<Vec<i64>>,
    pub sender_id: Option<i64>,
    pub subject: Option<String>,
    pub suppression_group_id: Option<i64>,
    pub title: String,
}
impl<'a> PostCampaignsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CampaignResponse> {
        let mut r = self.client.client.post("/v3/campaigns");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.categories {
            r = r.push_json(json!({ "categories" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_unsubscribe_url {
            r = r.push_json(json!({ "custom_unsubscribe_url" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.editor {
            r = r.push_json(json!({ "editor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.html_content {
            r = r.push_json(json!({ "html_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ip_pool {
            r = r.push_json(json!({ "ip_pool" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.list_ids {
            r = r.push_json(json!({ "list_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.plain_content {
            r = r.push_json(json!({ "plain_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.segment_ids {
            r = r.push_json(json!({ "segment_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.sender_id {
            r = r.push_json(json!({ "sender_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subject {
            r = r.push_json(json!({ "subject" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.suppression_group_id {
            r = r.push_json(json!({ "suppression_group_id" : unwrapped }));
        }
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
    pub fn custom_unsubscribe_url(mut self, custom_unsubscribe_url: &str) -> Self {
        self.custom_unsubscribe_url = Some(custom_unsubscribe_url.to_owned());
        self
    }
    pub fn editor(mut self, editor: &str) -> Self {
        self.editor = Some(editor.to_owned());
        self
    }
    pub fn html_content(mut self, html_content: &str) -> Self {
        self.html_content = Some(html_content.to_owned());
        self
    }
    pub fn ip_pool(mut self, ip_pool: &str) -> Self {
        self.ip_pool = Some(ip_pool.to_owned());
        self
    }
    pub fn list_ids(mut self, list_ids: Vec<i64>) -> Self {
        self.list_ids = Some(list_ids);
        self
    }
    pub fn plain_content(mut self, plain_content: &str) -> Self {
        self.plain_content = Some(plain_content.to_owned());
        self
    }
    pub fn segment_ids(mut self, segment_ids: Vec<i64>) -> Self {
        self.segment_ids = Some(segment_ids);
        self
    }
    pub fn sender_id(mut self, sender_id: i64) -> Self {
        self.sender_id = Some(sender_id);
        self
    }
    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_owned());
        self
    }
    pub fn suppression_group_id(mut self, suppression_group_id: i64) -> Self {
        self.suppression_group_id = Some(suppression_group_id);
        self
    }
}
