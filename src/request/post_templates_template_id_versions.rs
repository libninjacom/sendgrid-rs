use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostTemplatesTemplateIdVersionsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
    pub active: Option<i64>,
    pub editor: Option<String>,
    pub generate_plain_content: Option<bool>,
    pub html_content: Option<String>,
    pub name: String,
    pub plain_content: Option<String>,
    pub subject: String,
    pub test_data: Option<String>,
}
impl<'a> PostTemplatesTemplateIdVersionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionalTemplateVersionOutput> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/templates/{template_id}/versions", template_id = self
                    .template_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.active {
            r = r.push_json(json!({ "active" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.editor {
            r = r.push_json(json!({ "editor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.generate_plain_content {
            r = r.push_json(json!({ "generate_plain_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.html_content {
            r = r.push_json(json!({ "html_content" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.plain_content {
            r = r.push_json(json!({ "plain_content" : unwrapped }));
        }
        r = r.push_json(json!({ "subject" : self.subject }));
        if let Some(ref unwrapped) = self.test_data {
            r = r.push_json(json!({ "test_data" : unwrapped }));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn active(mut self, active: i64) -> Self {
        self.active = Some(active);
        self
    }
    pub fn editor(mut self, editor: &str) -> Self {
        self.editor = Some(editor.to_owned());
        self
    }
    pub fn generate_plain_content(mut self, generate_plain_content: bool) -> Self {
        self.generate_plain_content = Some(generate_plain_content);
        self
    }
    pub fn html_content(mut self, html_content: &str) -> Self {
        self.html_content = Some(html_content.to_owned());
        self
    }
    pub fn plain_content(mut self, plain_content: &str) -> Self {
        self.plain_content = Some(plain_content.to_owned());
        self
    }
    pub fn test_data(mut self, test_data: &str) -> Self {
        self.test_data = Some(test_data.to_owned());
        self
    }
}
