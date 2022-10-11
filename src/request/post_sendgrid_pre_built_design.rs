use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSendgridPreBuiltDesignRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub editor: Option<String>,
    pub name: Option<String>,
}
impl<'a> PostSendgridPreBuiltDesignRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DesignOutput> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/designs/pre-builts/{id}", id = self.id));
        if let Some(ref unwrapped) = self.editor {
            r = r.push_json(json!({ "editor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
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
    pub fn editor(mut self, editor: &str) -> Self {
        self.editor = Some(editor.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
