use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutDesignRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub categories: Option<Vec<String>>,
    pub generate_plain_content: Option<bool>,
    pub html_content: Option<String>,
    pub name: Option<String>,
    pub plain_content: Option<String>,
    pub subject: Option<String>,
}
impl<'a> PutDesignRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DesignOutput> {
        let mut r = self.client.client.patch(&format!("/v3/designs/{id}", id = self.id));
        if let Some(ref unwrapped) = self.categories {
            r = r.push_json(json!({ "categories" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.generate_plain_content {
            r = r.push_json(json!({ "generate_plain_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.html_content {
            r = r.push_json(json!({ "html_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.plain_content {
            r = r.push_json(json!({ "plain_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subject {
            r = r.push_json(json!({ "subject" : unwrapped }));
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
    pub fn generate_plain_content(mut self, generate_plain_content: bool) -> Self {
        self.generate_plain_content = Some(generate_plain_content);
        self
    }
    pub fn html_content(mut self, html_content: &str) -> Self {
        self.html_content = Some(html_content.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn plain_content(mut self, plain_content: &str) -> Self {
        self.plain_content = Some(plain_content.to_owned());
        self
    }
    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_owned());
        self
    }
}
