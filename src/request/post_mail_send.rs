use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMailSendRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub asm: Option<serde_json::Value>,
    pub attachments: Option<Vec<serde_json::Value>>,
    pub batch_id: Option<String>,
    pub categories: Option<Vec<String>>,
    pub content: Vec<serde_json::Value>,
    pub custom_args: Option<String>,
    pub from: FromEmailObject,
    pub headers: Option<serde_json::Value>,
    pub ip_pool_name: Option<String>,
    pub mail_settings: Option<serde_json::Value>,
    pub personalizations: Vec<serde_json::Value>,
    pub reply_to: Option<ReplyToEmailObject>,
    pub reply_to_list: Option<Vec<serde_json::Value>>,
    pub send_at: Option<i64>,
    pub subject: String,
    pub template_id: Option<String>,
    pub tracking_settings: Option<serde_json::Value>,
}
impl<'a> PostMailSendRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/v3/mail/send");
        if let Some(ref unwrapped) = self.asm {
            r = r.push_json(json!({ "asm" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.attachments {
            r = r.push_json(json!({ "attachments" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.batch_id {
            r = r.push_json(json!({ "batch_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.categories {
            r = r.push_json(json!({ "categories" : unwrapped }));
        }
        r = r.push_json(json!({ "content" : self.content }));
        if let Some(ref unwrapped) = self.custom_args {
            r = r.push_json(json!({ "custom_args" : unwrapped }));
        }
        r = r.push_json(json!({ "from" : self.from }));
        if let Some(ref unwrapped) = self.headers {
            r = r.push_json(json!({ "headers" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ip_pool_name {
            r = r.push_json(json!({ "ip_pool_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.mail_settings {
            r = r.push_json(json!({ "mail_settings" : unwrapped }));
        }
        r = r.push_json(json!({ "personalizations" : self.personalizations }));
        if let Some(ref unwrapped) = self.reply_to {
            r = r.push_json(json!({ "reply_to" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.reply_to_list {
            r = r.push_json(json!({ "reply_to_list" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.send_at {
            r = r.push_json(json!({ "send_at" : unwrapped }));
        }
        r = r.push_json(json!({ "subject" : self.subject }));
        if let Some(ref unwrapped) = self.template_id {
            r = r.push_json(json!({ "template_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tracking_settings {
            r = r.push_json(json!({ "tracking_settings" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn asm(mut self, asm: serde_json::Value) -> Self {
        self.asm = Some(asm);
        self
    }
    pub fn attachments(mut self, attachments: Vec<serde_json::Value>) -> Self {
        self.attachments = Some(attachments);
        self
    }
    pub fn batch_id(mut self, batch_id: &str) -> Self {
        self.batch_id = Some(batch_id.to_owned());
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
    pub fn custom_args(mut self, custom_args: &str) -> Self {
        self.custom_args = Some(custom_args.to_owned());
        self
    }
    pub fn headers(mut self, headers: serde_json::Value) -> Self {
        self.headers = Some(headers);
        self
    }
    pub fn ip_pool_name(mut self, ip_pool_name: &str) -> Self {
        self.ip_pool_name = Some(ip_pool_name.to_owned());
        self
    }
    pub fn mail_settings(mut self, mail_settings: serde_json::Value) -> Self {
        self.mail_settings = Some(mail_settings);
        self
    }
    pub fn reply_to(mut self, reply_to: ReplyToEmailObject) -> Self {
        self.reply_to = Some(reply_to);
        self
    }
    pub fn reply_to_list(mut self, reply_to_list: Vec<serde_json::Value>) -> Self {
        self.reply_to_list = Some(reply_to_list);
        self
    }
    pub fn send_at(mut self, send_at: i64) -> Self {
        self.send_at = Some(send_at);
        self
    }
    pub fn template_id(mut self, template_id: &str) -> Self {
        self.template_id = Some(template_id.to_owned());
        self
    }
    pub fn tracking_settings(mut self, tracking_settings: serde_json::Value) -> Self {
        self.tracking_settings = Some(tracking_settings);
        self
    }
}
pub struct PostMailSendRequired<'a> {
    pub content: Vec<serde_json::Value>,
    pub from: FromEmailObject,
    pub personalizations: Vec<serde_json::Value>,
    pub subject: &'a str,
}
impl<'a> PostMailSendRequired<'a> {}
