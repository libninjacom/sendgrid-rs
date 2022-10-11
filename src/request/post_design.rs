use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostDesignRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub editor: String,
    pub name: String,
    pub categories: Vec<String>,
    pub generate_plain_content: bool,
    pub subject: String,
    pub html_content: String,
    pub plain_content: String,
}
impl<'a> PostDesignRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DesignOutput> {
        let mut r = self.client.client.post("/v3/designs");
        r = r.push_json(json!({ "editor" : self.editor }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "categories" : self.categories }));
        r = r
            .push_json(
                json!({ "generate_plain_content" : self.generate_plain_content }),
            );
        r = r.push_json(json!({ "subject" : self.subject }));
        r = r.push_json(json!({ "html_content" : self.html_content }));
        r = r.push_json(json!({ "plain_content" : self.plain_content }));
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
pub struct PostDesignRequired<'a> {
    pub editor: &'a str,
    pub name: &'a str,
    pub categories: &'a [&'a str],
    pub generate_plain_content: bool,
    pub subject: &'a str,
    pub html_content: &'a str,
    pub plain_content: &'a str,
}
impl<'a> PostDesignRequired<'a> {}
