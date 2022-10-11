use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ip: Option<String>,
    pub exclude_whitelabels: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub subuser: Option<String>,
    pub sort_by_direction: Option<String>,
}
impl<'a> GetIpsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/ips");
        if let Some(ref unwrapped) = self.ip {
            r = r.push_query("ip", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exclude_whitelabels {
            r = r.push_query("exclude_whitelabels", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subuser {
            r = r.push_query("subuser", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_by_direction {
            r = r.push_query("sort_by_direction", &unwrapped.to_string());
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
    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = Some(ip.to_owned());
        self
    }
    pub fn exclude_whitelabels(mut self, exclude_whitelabels: bool) -> Self {
        self.exclude_whitelabels = Some(exclude_whitelabels);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn subuser(mut self, subuser: &str) -> Self {
        self.subuser = Some(subuser.to_owned());
        self
    }
    pub fn sort_by_direction(mut self, sort_by_direction: &str) -> Self {
        self.sort_by_direction = Some(sort_by_direction.to_owned());
        self
    }
}
