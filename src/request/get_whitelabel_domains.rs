use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelDomainsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub exclude_subusers: Option<bool>,
    pub username: Option<String>,
    pub domain: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetWhitelabelDomainsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthentication200Response> {
        let mut r = self.client.client.get("/v3/whitelabel/domains");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exclude_subusers {
            r = r.push_query("exclude_subusers", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.username {
            r = r.push_query("username", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.domain {
            r = r.push_query("domain", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn exclude_subusers(mut self, exclude_subusers: bool) -> Self {
        self.exclude_subusers = Some(exclude_subusers);
        self
    }
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
        self
    }
    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
