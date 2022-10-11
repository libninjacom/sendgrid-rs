use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAccessSettingsActivityRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetAccessSettingsActivityRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/access_settings/activity");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAccessSettingsWhitelistRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetAccessSettingsWhitelistRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpAccessResponse> {
        let mut r = self.client.client.get("/v3/access_settings/whitelist");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostAccessSettingsWhitelistRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub ips: Vec<serde_json::Value>,
}
impl<'a> PostAccessSettingsWhitelistRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpAccessResponse> {
        let mut r = self.client.client.post("/v3/access_settings/whitelist");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "ips" : self.ips }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAccessSettingsWhitelistRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub ids: Option<Vec<i64>>,
}
impl<'a> DeleteAccessSettingsWhitelistRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/access_settings/whitelist");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ids {
            r = r.push_json(json!({ "ids" : unwrapped }));
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
    pub fn ids(mut self, ids: Vec<i64>) -> Self {
        self.ids = Some(ids);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAccessSettingsWhitelistRuleIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub rule_id: String,
}
impl<'a> GetAccessSettingsWhitelistRuleIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpAccessResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/access_settings/whitelist/{rule_id}", rule_id = self.rule_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAccessSettingsWhitelistRuleIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub rule_id: String,
}
impl<'a> DeleteAccessSettingsWhitelistRuleIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/access_settings/whitelist/{rule_id}", rule_id = self.rule_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAlertsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub authorization: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetAlertsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/alerts");
        if let Some(ref unwrapped) = self.authorization {
            r = r.header("Authorization", &unwrapped.to_string());
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
    pub fn authorization(mut self, authorization: &str) -> Self {
        self.authorization = Some(authorization.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostAlertsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub authorization: Option<String>,
    pub on_behalf_of: Option<String>,
    pub email_to: Option<String>,
    pub frequency: Option<String>,
    pub percentage: Option<i64>,
    pub type_: String,
}
impl<'a> PostAlertsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/alerts");
        if let Some(ref unwrapped) = self.authorization {
            r = r.header("Authorization", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email_to {
            r = r.push_json(json!({ "email_to" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.frequency {
            r = r.push_json(json!({ "frequency" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.percentage {
            r = r.push_json(json!({ "percentage" : unwrapped }));
        }
        r = r.push_json(json!({ "type" : self.type_ }));
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
    pub fn authorization(mut self, authorization: &str) -> Self {
        self.authorization = Some(authorization.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn email_to(mut self, email_to: &str) -> Self {
        self.email_to = Some(email_to.to_owned());
        self
    }
    pub fn frequency(mut self, frequency: &str) -> Self {
        self.frequency = Some(frequency.to_owned());
        self
    }
    pub fn percentage(mut self, percentage: i64) -> Self {
        self.percentage = Some(percentage);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAlertsAlertIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub authorization: Option<String>,
    pub on_behalf_of: Option<String>,
    pub alert_id: i64,
}
impl<'a> GetAlertsAlertIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/alerts/{alert_id}", alert_id = self.alert_id));
        if let Some(ref unwrapped) = self.authorization {
            r = r.header("Authorization", &unwrapped.to_string());
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
    pub fn authorization(mut self, authorization: &str) -> Self {
        self.authorization = Some(authorization.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAlertsAlertIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub alert_id: i64,
}
impl<'a> DeleteAlertsAlertIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/alerts/{alert_id}", alert_id = self.alert_id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchAlertsAlertIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub alert_id: i64,
    pub email_to: Option<String>,
    pub frequency: Option<String>,
    pub percentage: Option<i64>,
}
impl<'a> PatchAlertsAlertIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/alerts/{alert_id}", alert_id = self.alert_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email_to {
            r = r.push_json(json!({ "email_to" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.frequency {
            r = r.push_json(json!({ "frequency" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.percentage {
            r = r.push_json(json!({ "percentage" : unwrapped }));
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
    pub fn email_to(mut self, email_to: &str) -> Self {
        self.email_to = Some(email_to.to_owned());
        self
    }
    pub fn frequency(mut self, frequency: &str) -> Self {
        self.frequency = Some(frequency.to_owned());
        self
    }
    pub fn percentage(mut self, percentage: i64) -> Self {
        self.percentage = Some(percentage);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetApiKeysRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetApiKeysRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/api_keys");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateApiKeysRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub name: String,
    pub scopes: Option<Vec<String>>,
}
impl<'a> CreateApiKeysRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/api_keys");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.scopes {
            r = r.push_json(json!({ "scopes" : unwrapped }));
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
    pub fn scopes(mut self, scopes: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.scopes = Some(scopes.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetApiKeysApiKeyIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub api_key_id: String,
}
impl<'a> GetApiKeysApiKeyIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/api_keys/{api_key_id}", api_key_id = self.api_key_id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutApiKeysApiKeyIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub api_key_id: String,
    pub name: String,
    pub scopes: Option<Vec<String>>,
}
impl<'a> PutApiKeysApiKeyIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ApiKeyNameIdScopes> {
        let mut r = self
            .client
            .client
            .put(&format!("/v3/api_keys/{api_key_id}", api_key_id = self.api_key_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.scopes {
            r = r.push_json(json!({ "scopes" : unwrapped }));
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
    pub fn scopes(mut self, scopes: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.scopes = Some(scopes.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteApiKeysApiKeyIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub api_key_id: String,
}
impl<'a> DeleteApiKeysApiKeyIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/api_keys/{api_key_id}", api_key_id = self.api_key_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchApiKeysApiKeyIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub api_key_id: String,
    pub name: String,
}
impl<'a> PatchApiKeysApiKeyIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ApiKeyNameId> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/api_keys/{api_key_id}", api_key_id = self.api_key_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "name" : self.name }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAsmGroupsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetAsmGroupsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<SuppressionGroup>> {
        let mut r = self.client.client.get("/v3/asm/groups");
        if let Some(ref unwrapped) = self.id {
            r = r.push_query("id", &unwrapped.to_string());
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
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostAsmGroupsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
    pub name: Option<String>,
}
impl<'a> PostAsmGroupsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/asm/groups");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_default {
            r = r.push_json(json!({ "is_default" : unwrapped }));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAsmGroupsGroupIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub group_id: String,
}
impl<'a> GetAsmGroupsGroupIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/asm/groups/{group_id}", group_id = self.group_id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAsmGroupsGroupIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub group_id: String,
}
impl<'a> DeleteAsmGroupsGroupIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/asm/groups/{group_id}", group_id = self.group_id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchAsmGroupsGroupIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub group_id: String,
    pub description: String,
    pub is_default: bool,
    pub name: String,
}
impl<'a> PatchAsmGroupsGroupIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SuppressionGroup> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/asm/groups/{group_id}", group_id = self.group_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "description" : self.description }));
        r = r.push_json(json!({ "is_default" : self.is_default }));
        r = r.push_json(json!({ "name" : self.name }));
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
pub struct PatchAsmGroupsGroupIdRequired<'a> {
    pub group_id: &'a str,
    pub description: &'a str,
    pub is_default: bool,
    pub name: &'a str,
}
impl<'a> PatchAsmGroupsGroupIdRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAsmGroupsGroupIdSuppressionsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub group_id: String,
}
impl<'a> GetAsmGroupsGroupIdSuppressionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<String>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/asm/groups/{group_id}/suppressions", group_id = self.group_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostAsmGroupsGroupIdSuppressionsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub group_id: String,
    pub recipient_emails: Vec<String>,
}
impl<'a> PostAsmGroupsGroupIdSuppressionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/asm/groups/{group_id}/suppressions", group_id = self.group_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "recipient_emails" : self.recipient_emails }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostAsmGroupsGroupIdSuppressionsSearchRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub group_id: String,
    pub recipient_emails: Vec<String>,
}
impl<'a> PostAsmGroupsGroupIdSuppressionsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<String>> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/asm/groups/{group_id}/suppressions/search", group_id = self
                    .group_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "recipient_emails" : self.recipient_emails }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAsmGroupsGroupIdSuppressionsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub group_id: String,
    pub email: String,
}
impl<'a> DeleteAsmGroupsGroupIdSuppressionsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/asm/groups/{group_id}/suppressions/{email}", group_id = self
                    .group_id, email = self.email
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAsmSuppressionsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetAsmSuppressionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/asm/suppressions");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostAsmSuppressionsGlobalRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub recipient_emails: Vec<String>,
}
impl<'a> PostAsmSuppressionsGlobalRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/asm/suppressions/global");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "recipient_emails" : self.recipient_emails }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAsmSuppressionsGlobalEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> GetAsmSuppressionsGlobalEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/asm/suppressions/global/{email}", email = self.email));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAsmSuppressionsGlobalEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> DeleteAsmSuppressionsGlobalEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/asm/suppressions/global/{email}", email = self.email));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAsmSuppressionsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> GetAsmSuppressionsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/asm/suppressions/{email}", email = self.email));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetBrowsersStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub browsers: Option<String>,
    pub on_behalf_of: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
}
impl<'a> GetBrowsersStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/browsers/stats");
        if let Some(ref unwrapped) = self.browsers {
            r = r.push_query("browsers", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn browsers(mut self, browsers: &str) -> Self {
        self.browsers = Some(browsers.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
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
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCampaignsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetCampaignsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/campaigns");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCampaignsCampaignIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
}
impl<'a> GetCampaignsCampaignIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/v3/campaigns/{campaign_id}", campaign_id = self.campaign_id),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteCampaignsCampaignIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
}
impl<'a> DeleteCampaignsCampaignIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v3/campaigns/{campaign_id}", campaign_id = self.campaign_id),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCampaignsCampaignIdSchedulesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
}
impl<'a> GetCampaignsCampaignIdSchedulesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/campaigns/{campaign_id}/schedules", campaign_id = self
                    .campaign_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostCampaignsCampaignIdSchedulesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
    pub send_at: i64,
}
impl<'a> PostCampaignsCampaignIdSchedulesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/campaigns/{campaign_id}/schedules", campaign_id = self
                    .campaign_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "send_at" : self.send_at }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteCampaignsCampaignIdSchedulesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
}
impl<'a> DeleteCampaignsCampaignIdSchedulesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/campaigns/{campaign_id}/schedules", campaign_id = self
                    .campaign_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchCampaignsCampaignIdSchedulesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
    pub send_at: i64,
}
impl<'a> PatchCampaignsCampaignIdSchedulesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/campaigns/{campaign_id}/schedules", campaign_id = self
                    .campaign_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "send_at" : self.send_at }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostCampaignsCampaignIdSchedulesNowRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
}
impl<'a> PostCampaignsCampaignIdSchedulesNowRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/campaigns/{campaign_id}/schedules/now", campaign_id = self
                    .campaign_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostCampaignsCampaignIdSchedulesTestRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub campaign_id: i64,
    pub to: String,
}
impl<'a> PostCampaignsCampaignIdSchedulesTestRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/campaigns/{campaign_id}/schedules/test", campaign_id = self
                    .campaign_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "to" : self.to }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCategoriesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub category: Option<String>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/categories");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.category {
            r = r.push_query("category", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_owned());
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCategoriesStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub start_date: String,
    pub end_date: Option<String>,
    pub categories: String,
    pub aggregated_by: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetCategoriesStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<CategoryStats>> {
        let mut r = self.client.client.get("/v3/categories/stats");
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        r = r.push_query("categories", &self.categories.to_string());
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
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
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCategoriesStatsSumsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub sort_by_metric: Option<String>,
    pub sort_by_direction: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetCategoriesStatsSumsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CategoryStats> {
        let mut r = self.client.client.get("/v3/categories/stats/sums");
        if let Some(ref unwrapped) = self.sort_by_metric {
            r = r.push_query("sort_by_metric", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_by_direction {
            r = r.push_query("sort_by_direction", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
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
    pub fn sort_by_metric(mut self, sort_by_metric: &str) -> Self {
        self.sort_by_metric = Some(sort_by_metric.to_owned());
        self
    }
    pub fn sort_by_direction(mut self, sort_by_direction: &str) -> Self {
        self.sort_by_direction = Some(sort_by_direction.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
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
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetClientsStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub aggregated_by: Option<String>,
}
impl<'a> GetClientsStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/clients/stats");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
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
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetClientsClientTypeStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub aggregated_by: Option<String>,
    pub client_type: String,
}
impl<'a> GetClientsClientTypeStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/clients/{client_type}/stats", client_type = self.client_type
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
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
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbCustomFieldsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbCustomFieldsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/contactdb/custom_fields");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbCustomFieldsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub name: Option<String>,
    pub type_: Option<String>,
}
impl<'a> PostContactdbCustomFieldsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbCustomFieldWithId> {
        let mut r = self.client.client.post("/v3/contactdb/custom_fields");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_json(json!({ "type" : unwrapped }));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbCustomFieldsCustomFieldIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub custom_field_id: i64,
}
impl<'a> GetContactdbCustomFieldsCustomFieldIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbCustomFieldWithId> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/contactdb/custom_fields/{custom_field_id}", custom_field_id =
                    self.custom_field_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteContactdbCustomFieldsCustomFieldIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub custom_field_id: i64,
}
impl<'a> DeleteContactdbCustomFieldsCustomFieldIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<GlobalErrorResponseSchema> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/contactdb/custom_fields/{custom_field_id}", custom_field_id =
                    self.custom_field_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbListsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbListsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/contactdb/lists");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbListsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub name: String,
}
impl<'a> PostContactdbListsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbList> {
        let mut r = self.client.client.post("/v3/contactdb/lists");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "name" : self.name }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteContactdbListsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub body: serde_json::Value,
}
impl<'a> DeleteContactdbListsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/contactdb/lists");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "body" : self.body }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbListsListIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub list_id: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbListsListIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbList> {
        let mut r = self.client.client.get("/v3/contactdb/lists/{list_id}");
        if let Some(ref unwrapped) = self.list_id {
            r = r.push_query("list_id", &unwrapped.to_string());
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
    pub fn list_id(mut self, list_id: i64) -> Self {
        self.list_id = Some(list_id);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteContactdbListsListIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub delete_contacts: Option<bool>,
    pub on_behalf_of: Option<String>,
    pub list_id: String,
}
impl<'a> DeleteContactdbListsListIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/contactdb/lists/{list_id}", list_id = self.list_id));
        if let Some(ref unwrapped) = self.delete_contacts {
            r = r.push_query("delete_contacts", &unwrapped.to_string());
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
    pub fn delete_contacts(mut self, delete_contacts: bool) -> Self {
        self.delete_contacts = Some(delete_contacts);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchContactdbListsListIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub list_id: i64,
    pub on_behalf_of: Option<String>,
    pub name: String,
}
impl<'a> PatchContactdbListsListIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.patch("/v3/contactdb/lists/{list_id}");
        r = r.push_query("list_id", &self.list_id.to_string());
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "name" : self.name }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbListsListIdRecipientsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub list_id: i64,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbListsListIdRecipientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/contactdb/lists/{list_id}/recipients");
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        r = r.push_query("list_id", &self.list_id.to_string());
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
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbListsListIdRecipientsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub list_id: i64,
    pub body: serde_json::Value,
}
impl<'a> PostContactdbListsListIdRecipientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/contactdb/lists/{list_id}/recipients", list_id = self.list_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "body" : self.body }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbListsListIdRecipientsRecipientIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub list_id: i64,
    pub recipient_id: String,
}
impl<'a> PostContactdbListsListIdRecipientsRecipientIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/contactdb/lists/{list_id}/recipients/{recipient_id}", list_id =
                    self.list_id, recipient_id = self.recipient_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteContactdbListsListIdRecipientsRecipientIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub list_id: i64,
    pub recipient_id: i64,
    pub on_behalf_of: Option<String>,
}
impl<'a> DeleteContactdbListsListIdRecipientsRecipientIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete("/v3/contactdb/lists/{list_id}/recipients/{recipient_id}");
        r = r.push_query("list_id", &self.list_id.to_string());
        r = r.push_query("recipient_id", &self.recipient_id.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbRecipientsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbRecipientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/contactdb/recipients");
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
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
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbRecipientsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub body: serde_json::Value,
}
impl<'a> PostContactdbRecipientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbRecipientResponse> {
        let mut r = self.client.client.post("/v3/contactdb/recipients");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "body" : self.body }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteContactdbRecipientsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub body: serde_json::Value,
}
impl<'a> DeleteContactdbRecipientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/contactdb/recipients");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "body" : self.body }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchContactdbRecipientsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub body: serde_json::Value,
}
impl<'a> PatchContactdbRecipientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbRecipientResponse> {
        let mut r = self.client.client.patch("/v3/contactdb/recipients");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "body" : self.body }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbRecipientsBillableCountRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbRecipientsBillableCountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbRecipientCount> {
        let mut r = self.client.client.get("/v3/contactdb/recipients/billable_count");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbRecipientsCountRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbRecipientsCountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbRecipientCount> {
        let mut r = self.client.client.get("/v3/contactdb/recipients/count");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbRecipientsSearchRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub conditions: Vec<ContactdbSegmentsConditions>,
    pub list_id: i64,
}
impl<'a> PostContactdbRecipientsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/contactdb/recipients/search");
        r = r.push_json(json!({ "conditions" : self.conditions }));
        r = r.push_json(json!({ "list_id" : self.list_id }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbRecipientsRecipientIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub recipient_id: String,
}
impl<'a> GetContactdbRecipientsRecipientIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbRecipient> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/contactdb/recipients/{recipient_id}", recipient_id = self
                    .recipient_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteContactdbRecipientsRecipientIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub recipient_id: String,
}
impl<'a> DeleteContactdbRecipientsRecipientIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/contactdb/recipients/{recipient_id}", recipient_id = self
                    .recipient_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbRecipientsRecipientIdListsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub recipient_id: String,
}
impl<'a> GetContactdbRecipientsRecipientIdListsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/contactdb/recipients/{recipient_id}/lists", recipient_id = self
                    .recipient_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbReservedFieldsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbReservedFieldsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/contactdb/reserved_fields");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbSegmentsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbSegmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/contactdb/segments");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostContactdbSegmentsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub conditions: Vec<ContactdbSegmentsConditions>,
    pub list_id: Option<i64>,
    pub name: String,
    pub recipient_count: Option<f64>,
}
impl<'a> PostContactdbSegmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbSegmentsWithId> {
        let mut r = self.client.client.post("/v3/contactdb/segments");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "conditions" : self.conditions }));
        if let Some(ref unwrapped) = self.list_id {
            r = r.push_json(json!({ "list_id" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.recipient_count {
            r = r.push_json(json!({ "recipient_count" : unwrapped }));
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
    pub fn list_id(mut self, list_id: i64) -> Self {
        self.list_id = Some(list_id);
        self
    }
    pub fn recipient_count(mut self, recipient_count: f64) -> Self {
        self.recipient_count = Some(recipient_count);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: i64,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbSegments> {
        let mut r = self.client.client.get("/v3/contactdb/segments/{segment_id}");
        r = r.push_query("segment_id", &self.segment_id.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteContactdbSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub delete_contacts: Option<bool>,
    pub on_behalf_of: Option<String>,
    pub segment_id: String,
}
impl<'a> DeleteContactdbSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/contactdb/segments/{segment_id}", segment_id = self.segment_id
                ),
            );
        if let Some(ref unwrapped) = self.delete_contacts {
            r = r.push_query("delete_contacts", &unwrapped.to_string());
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
    pub fn delete_contacts(mut self, delete_contacts: bool) -> Self {
        self.delete_contacts = Some(delete_contacts);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchContactdbSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: Option<String>,
    pub on_behalf_of: Option<String>,
    pub conditions: Option<Vec<ContactdbSegmentsConditions>>,
    pub list_id: Option<f64>,
    pub name: String,
}
impl<'a> PatchContactdbSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactdbSegments> {
        let mut r = self.client.client.patch("/v3/contactdb/segments/{segment_id}");
        if let Some(ref unwrapped) = self.segment_id {
            r = r.push_query("segment_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.conditions {
            r = r.push_json(json!({ "conditions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.list_id {
            r = r.push_json(json!({ "list_id" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
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
    pub fn segment_id(mut self, segment_id: &str) -> Self {
        self.segment_id = Some(segment_id.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn conditions(mut self, conditions: Vec<ContactdbSegmentsConditions>) -> Self {
        self.conditions = Some(conditions);
        self
    }
    pub fn list_id(mut self, list_id: f64) -> Self {
        self.list_id = Some(list_id);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbSegmentsSegmentIdRecipientsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub on_behalf_of: Option<String>,
    pub segment_id: i64,
}
impl<'a> GetContactdbSegmentsSegmentIdRecipientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/contactdb/segments/{segment_id}/recipients", segment_id = self
                    .segment_id
                ),
            );
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
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
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContactdbStatusRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetContactdbStatusRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/contactdb/status");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListDesignsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub summary: Option<bool>,
}
impl<'a> ListDesignsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/designs");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.summary {
            r = r.push_query("summary", &unwrapped.to_string());
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
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
    pub fn summary(mut self, summary: bool) -> Self {
        self.summary = Some(summary);
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListSendgridPreBuiltDesignsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub summary: Option<bool>,
}
impl<'a> ListSendgridPreBuiltDesignsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/designs/pre-builts");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.summary {
            r = r.push_query("summary", &unwrapped.to_string());
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
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
    pub fn summary(mut self, summary: bool) -> Self {
        self.summary = Some(summary);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSendgridPreBuiltDesignRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> GetSendgridPreBuiltDesignRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DesignOutput> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/designs/pre-builts/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetDesignRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> GetDesignRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DesignOutput> {
        let mut r = self.client.client.get(&format!("/v3/designs/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostDesignDupRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub editor: Option<String>,
    pub name: Option<String>,
}
impl<'a> PostDesignDupRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DesignOutput> {
        let mut r = self.client.client.post(&format!("/v3/designs/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteDesignRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> DeleteDesignRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/designs/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetDevicesStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
}
impl<'a> GetDevicesStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/devices/stats");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetGeoStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub country: Option<String>,
    pub on_behalf_of: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
}
impl<'a> GetGeoStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/geo/stats");
        if let Some(ref unwrapped) = self.country {
            r = r.push_query("country", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
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
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostIpsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub count: i64,
    pub subusers: Option<Vec<String>>,
    pub warmup: Option<bool>,
}
impl<'a> PostIpsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/ips");
        r = r.push_json(json!({ "count" : self.count }));
        if let Some(ref unwrapped) = self.subusers {
            r = r.push_json(json!({ "subusers" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.warmup {
            r = r.push_json(json!({ "warmup" : unwrapped }));
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
    pub fn subusers(
        mut self,
        subusers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .subusers = Some(
            subusers.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn warmup(mut self, warmup: bool) -> Self {
        self.warmup = Some(warmup);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsAssignedRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetIpsAssignedRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/ips/assigned");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsPoolsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetIpsPoolsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<IpPoolResponse>> {
        let mut r = self.client.client.get("/v3/ips/pools");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostIpsPoolsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub name: String,
}
impl<'a> PostIpsPoolsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpPoolResponse> {
        let mut r = self.client.client.post("/v3/ips/pools");
        r = r.push_json(json!({ "name" : self.name }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsPoolsPoolNameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub pool_name: String,
}
impl<'a> GetIpsPoolsPoolNameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/ips/pools/{pool_name}", pool_name = self.pool_name));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutIpsPoolsPoolNameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub pool_name: String,
    pub name: Option<String>,
}
impl<'a> PutIpsPoolsPoolNameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpPoolResponse> {
        let mut r = self
            .client
            .client
            .put(&format!("/v3/ips/pools/{pool_name}", pool_name = self.pool_name));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteIpsPoolsPoolNameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub pool_name: String,
}
impl<'a> DeleteIpsPoolsPoolNameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/ips/pools/{pool_name}", pool_name = self.pool_name));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostIpsPoolsPoolNameIpsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub pool_name: String,
    pub ip: Option<String>,
}
impl<'a> PostIpsPoolsPoolNameIpsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/ips/pools/{pool_name}/ips", pool_name = self.pool_name));
        if let Some(ref unwrapped) = self.ip {
            r = r.push_json(json!({ "ip" : unwrapped }));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteIpsPoolsPoolNameIpsIpRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub pool_name: String,
    pub ip: String,
}
impl<'a> DeleteIpsPoolsPoolNameIpsIpRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/ips/pools/{pool_name}/ips/{ip}", pool_name = self.pool_name, ip
                    = self.ip
                ),
            );
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsRemainingRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetIpsRemainingRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/ips/remaining");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsWarmupRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetIpsWarmupRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpWarmupResponse> {
        let mut r = self.client.client.get("/v3/ips/warmup");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostIpsWarmupRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ip: Option<String>,
}
impl<'a> PostIpsWarmupRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpWarmupResponse> {
        let mut r = self.client.client.post("/v3/ips/warmup");
        if let Some(ref unwrapped) = self.ip {
            r = r.push_json(json!({ "ip" : unwrapped }));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsWarmupIpAddressRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ip_address: String,
}
impl<'a> GetIpsWarmupIpAddressRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IpWarmupResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/ips/warmup/{ip_address}", ip_address = self.ip_address));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteIpsWarmupIpAddressRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ip_address: String,
}
impl<'a> DeleteIpsWarmupIpAddressRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v3/ips/warmup/{ip_address}", ip_address = self.ip_address),
            );
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIpsIpAddressRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ip_address: String,
}
impl<'a> GetIpsIpAddressRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/ips/{ip_address}", ip_address = self.ip_address));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMailBatchRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> PostMailBatchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailBatchId> {
        let mut r = self.client.client.post("/v3/mail/batch");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailBatchBatchIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub batch_id: String,
}
impl<'a> GetMailBatchBatchIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailBatchId> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/mail/batch/{batch_id}", batch_id = self.batch_id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailSettingsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetMailSettingsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/mail_settings");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailSettingsAddressWhitelistRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetMailSettingsAddressWhitelistRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsAddressWhitelabel> {
        let mut r = self.client.client.get("/v3/mail_settings/address_whitelist");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMailSettingsAddressWhitelistRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
    pub list: Option<Vec<String>>,
}
impl<'a> PatchMailSettingsAddressWhitelistRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsAddressWhitelabel> {
        let mut r = self.client.client.patch("/v3/mail_settings/address_whitelist");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.list {
            r = r.push_json(json!({ "list" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn list(mut self, list: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.list = Some(list.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailSettingsBouncePurgeRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetMailSettingsBouncePurgeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsBouncePurge> {
        let mut r = self.client.client.get("/v3/mail_settings/bounce_purge");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMailSettingsBouncePurgeRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
    pub hard_bounces: Option<i64>,
    pub soft_bounces: Option<i64>,
}
impl<'a> PatchMailSettingsBouncePurgeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsBouncePurge> {
        let mut r = self.client.client.patch("/v3/mail_settings/bounce_purge");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.hard_bounces {
            r = r.push_json(json!({ "hard_bounces" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.soft_bounces {
            r = r.push_json(json!({ "soft_bounces" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn hard_bounces(mut self, hard_bounces: i64) -> Self {
        self.hard_bounces = Some(hard_bounces);
        self
    }
    pub fn soft_bounces(mut self, soft_bounces: i64) -> Self {
        self.soft_bounces = Some(soft_bounces);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailSettingsFooterRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetMailSettingsFooterRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsFooter> {
        let mut r = self.client.client.get("/v3/mail_settings/footer");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMailSettingsFooterRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
    pub html_content: Option<String>,
    pub plain_content: Option<String>,
}
impl<'a> PatchMailSettingsFooterRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsFooter> {
        let mut r = self.client.client.patch("/v3/mail_settings/footer");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.html_content {
            r = r.push_json(json!({ "html_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.plain_content {
            r = r.push_json(json!({ "plain_content" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailSettingsForwardBounceRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetMailSettingsForwardBounceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsForwardBounce> {
        let mut r = self.client.client.get("/v3/mail_settings/forward_bounce");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMailSettingsForwardBounceRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: Option<String>,
    pub enabled: Option<bool>,
}
impl<'a> PatchMailSettingsForwardBounceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsForwardBounce> {
        let mut r = self.client.client.patch("/v3/mail_settings/forward_bounce");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_json(json!({ "email" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
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
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailSettingsForwardSpamRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetMailSettingsForwardSpamRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsForwardSpam> {
        let mut r = self.client.client.get("/v3/mail_settings/forward_spam");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMailSettingsForwardSpamRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: Option<String>,
    pub enabled: Option<bool>,
}
impl<'a> PatchMailSettingsForwardSpamRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsForwardSpam> {
        let mut r = self.client.client.patch("/v3/mail_settings/forward_spam");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_json(json!({ "email" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
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
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailSettingsTemplateRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetMailSettingsTemplateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MailSettingsTemplate> {
        let mut r = self.client.client.get("/v3/mail_settings/template");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMailSettingsTemplateRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
    pub html_content: Option<String>,
}
impl<'a> PatchMailSettingsTemplateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.patch("/v3/mail_settings/template");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.html_content {
            r = r.push_json(json!({ "html_content" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn html_content(mut self, html_content: &str) -> Self {
        self.html_content = Some(html_content.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMailboxProvidersStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub mailbox_providers: Option<String>,
    pub on_behalf_of: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
}
impl<'a> GetMailboxProvidersStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/mailbox_providers/stats");
        if let Some(ref unwrapped) = self.mailbox_providers {
            r = r.push_query("mailbox_providers", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn mailbox_providers(mut self, mailbox_providers: &str) -> Self {
        self.mailbox_providers = Some(mailbox_providers.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
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
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcContatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetMcContatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/contacts");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutMcContactsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub contacts: Vec<ContactRequest>,
    pub list_ids: Option<Vec<String>>,
}
impl<'a> PutMcContactsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.put("/v3/marketing/contacts");
        r = r.push_json(json!({ "contacts" : self.contacts }));
        if let Some(ref unwrapped) = self.list_ids {
            r = r.push_json(json!({ "list_ids" : unwrapped }));
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
    pub fn list_ids(
        mut self,
        list_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .list_ids = Some(
            list_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMcContactsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub delete_all_contacts: Option<String>,
    pub ids: Option<String>,
}
impl<'a> DeleteMcContactsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/marketing/contacts");
        if let Some(ref unwrapped) = self.delete_all_contacts {
            r = r.push_query("delete_all_contacts", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
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
    pub fn delete_all_contacts(mut self, delete_all_contacts: &str) -> Self {
        self.delete_all_contacts = Some(delete_all_contacts.to_owned());
        self
    }
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingContactsBatchRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ids: Vec<String>,
}
impl<'a> PostMarketingContactsBatchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/contacts/batch");
        r = r.push_json(json!({ "ids" : self.ids }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcContactsCountRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetMcContactsCountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/contacts/count");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingContactsExportsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetMarketingContactsExportsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/contacts/exports");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMcContactsExportsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub file_type: Option<String>,
    pub list_ids: Option<Vec<String>>,
    pub max_file_size: Option<i64>,
    pub notifications: Option<serde_json::Value>,
    pub segment_ids: Option<Vec<String>>,
}
impl<'a> PostMcContactsExportsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/contacts/exports");
        if let Some(ref unwrapped) = self.file_type {
            r = r.push_json(json!({ "file_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.list_ids {
            r = r.push_json(json!({ "list_ids" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_file_size {
            r = r.push_json(json!({ "max_file_size" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.notifications {
            r = r.push_json(json!({ "notifications" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.segment_ids {
            r = r.push_json(json!({ "segment_ids" : unwrapped }));
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
    pub fn file_type(mut self, file_type: &str) -> Self {
        self.file_type = Some(file_type.to_owned());
        self
    }
    pub fn list_ids(
        mut self,
        list_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .list_ids = Some(
            list_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn max_file_size(mut self, max_file_size: i64) -> Self {
        self.max_file_size = Some(max_file_size);
        self
    }
    pub fn notifications(mut self, notifications: serde_json::Value) -> Self {
        self.notifications = Some(notifications);
        self
    }
    pub fn segment_ids(
        mut self,
        segment_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .segment_ids = Some(
            segment_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcContactsExportsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> GetMcContactsExportsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactExport> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/contacts/exports/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutMcContactsImportsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub field_mappings: Vec<serde_json::Value>,
    pub file_type: String,
    pub list_ids: Option<Vec<String>>,
}
impl<'a> PutMcContactsImportsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.put("/v3/marketing/contacts/imports");
        r = r.push_json(json!({ "field_mappings" : self.field_mappings }));
        r = r.push_json(json!({ "file_type" : self.file_type }));
        if let Some(ref unwrapped) = self.list_ids {
            r = r.push_json(json!({ "list_ids" : unwrapped }));
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
    pub fn list_ids(
        mut self,
        list_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .list_ids = Some(
            list_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingContactsImportsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> GetMarketingContactsImportsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactImport> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/contacts/imports/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMcContactsSearchRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub query: String,
}
impl<'a> PostMcContactsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/contacts/search");
        r = r.push_json(json!({ "query" : self.query }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingContactsSearchEmailsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub emails: Vec<String>,
}
impl<'a> PostMarketingContactsSearchEmailsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/contacts/search/emails");
        r = r.push_json(json!({ "emails" : self.emails }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcContactsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> GetMcContactsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContactDetails3> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/contacts/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcFieldDefinitionsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetMcFieldDefinitionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/field_definitions");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMcFieldDefinitionsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub field_type: String,
    pub name: String,
}
impl<'a> PostMcFieldDefinitionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/field_definitions");
        r = r.push_json(json!({ "field_type" : self.field_type }));
        r = r.push_json(json!({ "name" : self.name }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMcFieldDefinitionsCustomFieldIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub custom_field_id: String,
}
impl<'a> DeleteMcFieldDefinitionsCustomFieldIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/marketing/field_definitions/{custom_field_id}", custom_field_id
                    = self.custom_field_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMcFieldDefinitionsCustomFieldIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub custom_field_id: String,
    pub name: String,
}
impl<'a> PatchMcFieldDefinitionsCustomFieldIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/marketing/field_definitions/{custom_field_id}", custom_field_id
                    = self.custom_field_id
                ),
            );
        r = r.push_json(json!({ "name" : self.name }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcListsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page_size: Option<f64>,
    pub page_token: Option<String>,
}
impl<'a> GetMcListsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/lists");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
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
    pub fn page_size(mut self, page_size: f64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMcListsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub name: String,
}
impl<'a> PostMcListsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<List> {
        let mut r = self.client.client.post("/v3/marketing/lists");
        r = r.push_json(json!({ "name" : self.name }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcListsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub contact_sample: Option<bool>,
    pub id: String,
}
impl<'a> GetMcListsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/lists/{id}", id = self.id));
        if let Some(ref unwrapped) = self.contact_sample {
            r = r.push_query("contact_sample", &unwrapped.to_string());
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
    pub fn contact_sample(mut self, contact_sample: bool) -> Self {
        self.contact_sample = Some(contact_sample);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteListsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub delete_contacts: Option<bool>,
    pub id: String,
}
impl<'a> DeleteListsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/marketing/lists/{id}", id = self.id));
        if let Some(ref unwrapped) = self.delete_contacts {
            r = r.push_query("delete_contacts", &unwrapped.to_string());
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
    pub fn delete_contacts(mut self, delete_contacts: bool) -> Self {
        self.delete_contacts = Some(delete_contacts);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMcListsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub name: Option<String>,
}
impl<'a> PatchMcListsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<List> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/marketing/lists/{id}", id = self.id));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMcListsIdContactsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub contact_ids: String,
    pub id: String,
}
impl<'a> DeleteMcListsIdContactsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/marketing/lists/{id}/contacts", id = self.id));
        r = r.push_query("contact_ids", &self.contact_ids.to_string());
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMcListsIdContactsCountRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> GetMcListsIdContactsCountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/lists/{id}/contacts/count", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingSegmentsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ids: Option<Vec<serde_json::Value>>,
    pub parent_list_ids: Option<String>,
    pub no_parent_list_id: Option<bool>,
}
impl<'a> GetMarketingSegmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/segments");
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.parent_list_ids {
            r = r.push_query("parent_list_ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.no_parent_list_id {
            r = r.push_query("no_parent_list_id", &unwrapped.to_string());
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
    pub fn ids(mut self, ids: Vec<serde_json::Value>) -> Self {
        self.ids = Some(ids);
        self
    }
    pub fn parent_list_ids(mut self, parent_list_ids: &str) -> Self {
        self.parent_list_ids = Some(parent_list_ids.to_owned());
        self
    }
    pub fn no_parent_list_id(mut self, no_parent_list_id: bool) -> Self {
        self.no_parent_list_id = Some(no_parent_list_id);
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSegmentsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ids: Option<Vec<serde_json::Value>>,
    pub parent_list_ids: Option<String>,
    pub no_parent_list_id: Option<bool>,
}
impl<'a> GetSegmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AllSegmentsResponse> {
        let mut r = self.client.client.get("/v3/marketing/segments/2.0");
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.parent_list_ids {
            r = r.push_query("parent_list_ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.no_parent_list_id {
            r = r.push_query("no_parent_list_id", &unwrapped.to_string());
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
    pub fn ids(mut self, ids: Vec<serde_json::Value>) -> Self {
        self.ids = Some(ids);
        self
    }
    pub fn parent_list_ids(mut self, parent_list_ids: &str) -> Self {
        self.parent_list_ids = Some(parent_list_ids.to_owned());
        self
    }
    pub fn no_parent_list_id(mut self, no_parent_list_id: bool) -> Self {
        self.no_parent_list_id = Some(no_parent_list_id);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSegmentsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub name: String,
    pub parent_list_ids: Option<Vec<String>>,
    pub query_dsl: String,
}
impl<'a> PostSegmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SegmentResponse> {
        let mut r = self.client.client.post("/v3/marketing/segments/2.0");
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.parent_list_ids {
            r = r.push_json(json!({ "parent_list_ids" : unwrapped }));
        }
        r = r.push_json(json!({ "query_dsl" : self.query_dsl }));
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
    pub fn parent_list_ids(
        mut self,
        parent_list_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .parent_list_ids = Some(
            parent_list_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub contacts_sample: Option<bool>,
    pub segment_id: String,
}
impl<'a> GetSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SegmentResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/marketing/segments/2.0/{segment_id}", segment_id = self
                    .segment_id
                ),
            );
        if let Some(ref unwrapped) = self.contacts_sample {
            r = r.push_query("contacts_sample", &unwrapped.to_string());
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
    pub fn contacts_sample(mut self, contacts_sample: bool) -> Self {
        self.contacts_sample = Some(contacts_sample);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: String,
}
impl<'a> DeleteSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/marketing/segments/2.0/{segment_id}", segment_id = self
                    .segment_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: String,
    pub name: Option<String>,
    pub query_dsl: Option<String>,
}
impl<'a> PatchSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SegmentResponse> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/marketing/segments/2.0/{segment_id}", segment_id = self
                    .segment_id
                ),
            );
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.query_dsl {
            r = r.push_json(json!({ "query_dsl" : unwrapped }));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn query_dsl(mut self, query_dsl: &str) -> Self {
        self.query_dsl = Some(query_dsl.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub query_json: Option<bool>,
    pub segment_id: String,
}
impl<'a> GetMarketingSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FullSegment> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/marketing/segments/{segment_id}", segment_id = self.segment_id
                ),
            );
        if let Some(ref unwrapped) = self.query_json {
            r = r.push_query("query_json", &unwrapped.to_string());
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
    pub fn query_json(mut self, query_json: bool) -> Self {
        self.query_json = Some(query_json);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMarketingSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: String,
}
impl<'a> DeleteMarketingSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/marketing/segments/{segment_id}", segment_id = self.segment_id
                ),
            );
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMarketingSegmentsSegmentIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub segment_id: String,
    pub name: String,
    pub parent_list_ids: Option<Vec<String>>,
    pub query_dsl: String,
}
impl<'a> PatchMarketingSegmentsSegmentIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FullSegment> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/marketing/segments/{segment_id}", segment_id = self.segment_id
                ),
            );
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.parent_list_ids {
            r = r.push_json(json!({ "parent_list_ids" : unwrapped }));
        }
        r = r.push_json(json!({ "query_dsl" : self.query_dsl }));
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
    pub fn parent_list_ids(
        mut self,
        parent_list_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .parent_list_ids = Some(
            parent_list_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingSendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub address: String,
    pub address2: Option<String>,
    pub city: String,
    pub country: String,
    pub from: serde_json::Value,
    pub nickname: String,
    pub reply_to: Option<serde_json::Value>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl<'a> PostMarketingSendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SenderId> {
        let mut r = self.client.client.post("/v3/marketing/senders");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "address" : self.address }));
        if let Some(ref unwrapped) = self.address2 {
            r = r.push_json(json!({ "address_2" : unwrapped }));
        }
        r = r.push_json(json!({ "city" : self.city }));
        r = r.push_json(json!({ "country" : self.country }));
        r = r.push_json(json!({ "from" : self.from }));
        r = r.push_json(json!({ "nickname" : self.nickname }));
        if let Some(ref unwrapped) = self.reply_to {
            r = r.push_json(json!({ "reply_to" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.state {
            r = r.push_json(json!({ "state" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.zip {
            r = r.push_json(json!({ "zip" : unwrapped }));
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
    pub fn address2(mut self, address2: &str) -> Self {
        self.address2 = Some(address2.to_owned());
        self
    }
    pub fn reply_to(mut self, reply_to: serde_json::Value) -> Self {
        self.reply_to = Some(reply_to);
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn zip(mut self, zip: &str) -> Self {
        self.zip = Some(zip.to_owned());
        self
    }
}
pub struct PostMarketingSendersRequired<'a> {
    pub address: &'a str,
    pub city: &'a str,
    pub country: &'a str,
    pub from: serde_json::Value,
    pub nickname: &'a str,
}
impl<'a> PostMarketingSendersRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingSinglesendsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
}
impl<'a> GetMarketingSinglesendsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/singlesends");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
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
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingSinglesendsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub categories: Option<Vec<String>>,
    pub email_config: Option<serde_json::Value>,
    pub name: String,
    pub send_at: Option<String>,
    pub send_to: Option<serde_json::Value>,
}
impl<'a> PostMarketingSinglesendsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendResponse> {
        let mut r = self.client.client.post("/v3/marketing/singlesends");
        if let Some(ref unwrapped) = self.categories {
            r = r.push_json(json!({ "categories" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email_config {
            r = r.push_json(json!({ "email_config" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.send_at {
            r = r.push_json(json!({ "send_at" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.send_to {
            r = r.push_json(json!({ "send_to" : unwrapped }));
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
    pub fn email_config(mut self, email_config: serde_json::Value) -> Self {
        self.email_config = Some(email_config);
        self
    }
    pub fn send_at(mut self, send_at: &str) -> Self {
        self.send_at = Some(send_at.to_owned());
        self
    }
    pub fn send_to(mut self, send_to: serde_json::Value) -> Self {
        self.send_to = Some(send_to);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMarketingSinglesendsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ids: Option<Vec<String>>,
}
impl<'a> DeleteMarketingSinglesendsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.delete("/v3/marketing/singlesends");
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
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
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingSinglesendsCategoriesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetMarketingSinglesendsCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/marketing/singlesends/categories");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingSinglesendsSearchRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub categories: Option<Vec<String>>,
    pub name: Option<String>,
    pub status: Option<Vec<String>>,
}
impl<'a> PostMarketingSinglesendsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/singlesends/search");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.categories {
            r = r.push_json(json!({ "categories" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_json(json!({ "status" : unwrapped }));
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
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn status(mut self, status: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.status = Some(status.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMarketingSinglesendsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> GetMarketingSinglesendsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/singlesends/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingSinglesendsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub name: Option<String>,
}
impl<'a> PostMarketingSinglesendsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendResponse> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/marketing/singlesends/{id}", id = self.id));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMarketingSinglesendsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> DeleteMarketingSinglesendsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/marketing/singlesends/{id}", id = self.id));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchMarketingSinglesendsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub categories: Option<Vec<String>>,
    pub email_config: Option<serde_json::Value>,
    pub name: String,
    pub send_at: Option<String>,
    pub send_to: Option<serde_json::Value>,
}
impl<'a> PatchMarketingSinglesendsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendResponse> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/marketing/singlesends/{id}", id = self.id));
        if let Some(ref unwrapped) = self.categories {
            r = r.push_json(json!({ "categories" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email_config {
            r = r.push_json(json!({ "email_config" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.send_at {
            r = r.push_json(json!({ "send_at" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.send_to {
            r = r.push_json(json!({ "send_to" : unwrapped }));
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
    pub fn email_config(mut self, email_config: serde_json::Value) -> Self {
        self.email_config = Some(email_config);
        self
    }
    pub fn send_at(mut self, send_at: &str) -> Self {
        self.send_at = Some(send_at.to_owned());
        self
    }
    pub fn send_to(mut self, send_to: serde_json::Value) -> Self {
        self.send_to = Some(send_to);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutMarketingSinglesendsIdScheduleRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub send_at: String,
}
impl<'a> PutMarketingSinglesendsIdScheduleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(&format!("/v3/marketing/singlesends/{id}/schedule", id = self.id));
        r = r.push_json(json!({ "send_at" : self.send_at }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMarketingSinglesendsIdScheduleRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> DeleteMarketingSinglesendsIdScheduleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendSchedule> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/marketing/singlesends/{id}/schedule", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetallAutomationStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub automation_ids: Option<Vec<String>>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
}
impl<'a> GetallAutomationStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationsResponse> {
        let mut r = self.client.client.get("/v3/marketing/stats/automations");
        if let Some(ref unwrapped) = self.automation_ids {
            for item in unwrapped {
                r = r.push_query("automation_ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
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
    pub fn automation_ids(
        mut self,
        automation_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .automation_ids = Some(
            automation_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAutomationsStatsExportRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ids: Option<Vec<String>>,
    pub timezone: Option<String>,
}
impl<'a> GetAutomationsStatsExportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<String> {
        let mut r = self.client.client.get("/v3/marketing/stats/automations/export");
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.timezone {
            r = r.push_query("timezone", &unwrapped.to_string());
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
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAutomationStatRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub group_by: Option<Vec<String>>,
    pub step_ids: Option<Vec<String>>,
    pub aggregated_by: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub timezone: Option<String>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub id: String,
}
impl<'a> GetAutomationStatRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationsResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/stats/automations/{id}", id = self.id));
        if let Some(ref unwrapped) = self.group_by {
            for item in unwrapped {
                r = r.push_query("group_by[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.step_ids {
            for item in unwrapped {
                r = r.push_query("step_ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_query("start_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.timezone {
            r = r.push_query("timezone", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
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
    pub fn group_by(
        mut self,
        group_by: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .group_by = Some(
            group_by.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn step_ids(
        mut self,
        step_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .step_ids = Some(
            step_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_owned());
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAutomationLinkStatRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub group_by: Option<Vec<String>>,
    pub step_ids: Option<Vec<String>>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub id: String,
}
impl<'a> GetAutomationLinkStatRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationsLinkStatsResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/stats/automations/{id}/links", id = self.id));
        if let Some(ref unwrapped) = self.group_by {
            for item in unwrapped {
                r = r.push_query("group_by[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.step_ids {
            for item in unwrapped {
                r = r.push_query("step_ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
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
    pub fn group_by(
        mut self,
        group_by: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .group_by = Some(
            group_by.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn step_ids(
        mut self,
        step_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .step_ids = Some(
            step_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetallSinglesendStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub singlesend_ids: Option<Vec<String>>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
}
impl<'a> GetallSinglesendStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendsResponse> {
        let mut r = self.client.client.get("/v3/marketing/stats/singlesends");
        if let Some(ref unwrapped) = self.singlesend_ids {
            for item in unwrapped {
                r = r.push_query("singlesend_ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
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
    pub fn singlesend_ids(
        mut self,
        singlesend_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .singlesend_ids = Some(
            singlesend_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSinglesendStatsExportRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub ids: Option<Vec<String>>,
    pub timezone: Option<String>,
}
impl<'a> GetSinglesendStatsExportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<String> {
        let mut r = self.client.client.get("/v3/marketing/stats/singlesends/export");
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.timezone {
            r = r.push_query("timezone", &unwrapped.to_string());
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
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSinglesendStatRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub aggregated_by: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub timezone: Option<String>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub group_by: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetSinglesendStatRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendsResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/stats/singlesends/{id}", id = self.id));
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_query("start_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.timezone {
            r = r.push_query("timezone", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.group_by {
            for item in unwrapped {
                r = r.push_query("group_by[]", &item.to_string());
            }
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
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_owned());
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
    pub fn group_by(
        mut self,
        group_by: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .group_by = Some(
            group_by.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSinglesendLinkStatRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub group_by: Option<Vec<String>>,
    pub ab_variation_id: Option<String>,
    pub ab_phase_id: Option<String>,
    pub id: String,
}
impl<'a> GetSinglesendLinkStatRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SinglesendsLinkStatsResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/marketing/stats/singlesends/{id}/links", id = self.id));
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("page_size", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.group_by {
            for item in unwrapped {
                r = r.push_query("group_by[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.ab_variation_id {
            r = r.push_query("ab_variation_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ab_phase_id {
            r = r.push_query("ab_phase_id", &unwrapped.to_string());
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
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
    pub fn group_by(
        mut self,
        group_by: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .group_by = Some(
            group_by.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn ab_variation_id(mut self, ab_variation_id: &str) -> Self {
        self.ab_variation_id = Some(ab_variation_id.to_owned());
        self
    }
    pub fn ab_phase_id(mut self, ab_phase_id: &str) -> Self {
        self.ab_phase_id = Some(ab_phase_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostMarketingTestSendEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub custom_unsubscribe_url: Option<String>,
    pub emails: Vec<String>,
    pub from_address: Option<String>,
    pub sender_id: Option<i64>,
    pub suppression_group_id: Option<i64>,
    pub template_id: String,
    pub version_id_override: Option<String>,
}
impl<'a> PostMarketingTestSendEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/marketing/test/send_email");
        if let Some(ref unwrapped) = self.custom_unsubscribe_url {
            r = r.push_json(json!({ "custom_unsubscribe_url" : unwrapped }));
        }
        r = r.push_json(json!({ "emails" : self.emails }));
        if let Some(ref unwrapped) = self.from_address {
            r = r.push_json(json!({ "from_address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.sender_id {
            r = r.push_json(json!({ "sender_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.suppression_group_id {
            r = r.push_json(json!({ "suppression_group_id" : unwrapped }));
        }
        r = r.push_json(json!({ "template_id" : self.template_id }));
        if let Some(ref unwrapped) = self.version_id_override {
            r = r.push_json(json!({ "version_id_override" : unwrapped }));
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
    pub fn custom_unsubscribe_url(mut self, custom_unsubscribe_url: &str) -> Self {
        self.custom_unsubscribe_url = Some(custom_unsubscribe_url.to_owned());
        self
    }
    pub fn from_address(mut self, from_address: &str) -> Self {
        self.from_address = Some(from_address.to_owned());
        self
    }
    pub fn sender_id(mut self, sender_id: i64) -> Self {
        self.sender_id = Some(sender_id);
        self
    }
    pub fn suppression_group_id(mut self, suppression_group_id: i64) -> Self {
        self.suppression_group_id = Some(suppression_group_id);
        self
    }
    pub fn version_id_override(mut self, version_id_override: &str) -> Self {
        self.version_id_override = Some(version_id_override.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMessagesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub query: String,
    pub limit: Option<f64>,
    pub authorization: String,
}
impl<'a> GetMessagesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/messages");
        r = r.push_query("query", &self.query.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = r.header("Authorization", &self.authorization.to_string());
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
    pub fn limit(mut self, limit: f64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostV3MessagesDownloadRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub query: Option<String>,
    pub authorization: String,
}
impl<'a> PostV3MessagesDownloadRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/messages/download");
        if let Some(ref unwrapped) = self.query {
            r = r.push_query("query", &unwrapped.to_string());
        }
        r = r.header("Authorization", &self.authorization.to_string());
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
    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetV3MessagesDownloadDownloadUuidRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub authorization: String,
    pub download_uuid: String,
}
impl<'a> GetV3MessagesDownloadDownloadUuidRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/messages/download/{download_uuid}", download_uuid = self
                    .download_uuid
                ),
            );
        r = r.header("Authorization", &self.authorization.to_string());
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetV3MessagesMsgIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub authorization: String,
    pub msg_id: String,
}
impl<'a> GetV3MessagesMsgIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/messages/{msg_id}", msg_id = self.msg_id));
        r = r.header("Authorization", &self.authorization.to_string());
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetPartnerSettingsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetPartnerSettingsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/partner_settings");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetPartnerSettingsNewRelicRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetPartnerSettingsNewRelicRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PartnerSettingsNewRelic> {
        let mut r = self.client.client.get("/v3/partner_settings/new_relic");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchPartnerSettingsNewRelicRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enable_subuser_statistics: Option<bool>,
    pub enabled: Option<bool>,
    pub license_key: Option<String>,
}
impl<'a> PatchPartnerSettingsNewRelicRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PartnerSettingsNewRelic> {
        let mut r = self.client.client.patch("/v3/partner_settings/new_relic");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enable_subuser_statistics {
            r = r.push_json(json!({ "enable_subuser_statistics" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.license_key {
            r = r.push_json(json!({ "license_key" : unwrapped }));
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
    pub fn enable_subuser_statistics(mut self, enable_subuser_statistics: bool) -> Self {
        self.enable_subuser_statistics = Some(enable_subuser_statistics);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn license_key(mut self, license_key: &str) -> Self {
        self.license_key = Some(license_key.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetScopesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetScopesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/scopes");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetV3ScopesRequestsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
impl<'a> GetV3ScopesRequestsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/scopes/requests");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteV3ScopesRequestsRequestIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub request_id: String,
}
impl<'a> DeleteV3ScopesRequestsRequestIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/scopes/requests/{request_id}", request_id = self.request_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchV3ScopesRequestsApproveIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub request_id: String,
}
impl<'a> PatchV3ScopesRequestsApproveIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/scopes/requests/{request_id}/approve", request_id = self
                    .request_id
                ),
            );
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetV3SendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetV3SendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/senders");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub address: String,
    pub address2: String,
    pub city: String,
    pub country: String,
    pub from: serde_json::Value,
    pub nickname: String,
    pub reply_to: serde_json::Value,
    pub state: String,
    pub zip: String,
}
impl<'a> PostSendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SenderId> {
        let mut r = self.client.client.post("/v3/senders");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "address" : self.address }));
        r = r.push_json(json!({ "address_2" : self.address2 }));
        r = r.push_json(json!({ "city" : self.city }));
        r = r.push_json(json!({ "country" : self.country }));
        r = r.push_json(json!({ "from" : self.from }));
        r = r.push_json(json!({ "nickname" : self.nickname }));
        r = r.push_json(json!({ "reply_to" : self.reply_to }));
        r = r.push_json(json!({ "state" : self.state }));
        r = r.push_json(json!({ "zip" : self.zip }));
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
pub struct PostSendersRequired<'a> {
    pub address: &'a str,
    pub address2: &'a str,
    pub city: &'a str,
    pub country: &'a str,
    pub from: serde_json::Value,
    pub nickname: &'a str,
    pub reply_to: serde_json::Value,
    pub state: &'a str,
    pub zip: &'a str,
}
impl<'a> PostSendersRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetV3SendersSenderIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub sender_id: i64,
}
impl<'a> GetV3SendersSenderIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SenderId> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/senders/{sender_id}", sender_id = self.sender_id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteV3SendersSenderIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub sender_id: i64,
}
impl<'a> DeleteV3SendersSenderIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/senders/{sender_id}", sender_id = self.sender_id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchV3SendersSenderIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub sender_id: i64,
    pub address: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub from: Option<serde_json::Value>,
    pub nickname: Option<String>,
    pub reply_to: Option<serde_json::Value>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl<'a> PatchV3SendersSenderIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SenderId> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/senders/{sender_id}", sender_id = self.sender_id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address2 {
            r = r.push_json(json!({ "address_2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.city {
            r = r.push_json(json!({ "city" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country {
            r = r.push_json(json!({ "country" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.from {
            r = r.push_json(json!({ "from" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.nickname {
            r = r.push_json(json!({ "nickname" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.reply_to {
            r = r.push_json(json!({ "reply_to" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.state {
            r = r.push_json(json!({ "state" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.zip {
            r = r.push_json(json!({ "zip" : unwrapped }));
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
    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_owned());
        self
    }
    pub fn address2(mut self, address2: &str) -> Self {
        self.address2 = Some(address2.to_owned());
        self
    }
    pub fn city(mut self, city: &str) -> Self {
        self.city = Some(city.to_owned());
        self
    }
    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }
    pub fn from(mut self, from: serde_json::Value) -> Self {
        self.from = Some(from);
        self
    }
    pub fn nickname(mut self, nickname: &str) -> Self {
        self.nickname = Some(nickname.to_owned());
        self
    }
    pub fn reply_to(mut self, reply_to: serde_json::Value) -> Self {
        self.reply_to = Some(reply_to);
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn zip(mut self, zip: &str) -> Self {
        self.zip = Some(zip.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostV3SendersSenderIdResendVerificationRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub sender_id: i64,
}
impl<'a> PostV3SendersSenderIdResendVerificationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/senders/{sender_id}/resend_verification", sender_id = self
                    .sender_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSsoCertificatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub enabled: Option<bool>,
    pub integration_id: String,
    pub public_certificate: String,
}
impl<'a> PostSsoCertificatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoCertificateBody> {
        let mut r = self.client.client.post("/v3/sso/certificates");
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        r = r.push_json(json!({ "integration_id" : self.integration_id }));
        r = r.push_json(json!({ "public_certificate" : self.public_certificate }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSsoCertificatesCertIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub cert_id: String,
}
impl<'a> GetSsoCertificatesCertIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoCertificateBody> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/sso/certificates/{cert_id}", cert_id = self.cert_id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSsoCertificatesCertIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub cert_id: String,
}
impl<'a> DeleteSsoCertificatesCertIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoCertificateBody> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/sso/certificates/{cert_id}", cert_id = self.cert_id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSsoCertificatesCertIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub cert_id: String,
    pub enabled: Option<bool>,
    pub integration_id: Option<String>,
    pub public_certificate: Option<String>,
}
impl<'a> PatchSsoCertificatesCertIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/sso/certificates/{cert_id}", cert_id = self.cert_id));
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.integration_id {
            r = r.push_json(json!({ "integration_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.public_certificate {
            r = r.push_json(json!({ "public_certificate" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn integration_id(mut self, integration_id: &str) -> Self {
        self.integration_id = Some(integration_id.to_owned());
        self
    }
    pub fn public_certificate(mut self, public_certificate: &str) -> Self {
        self.public_certificate = Some(public_certificate.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSsoIntegrationsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub si: Option<bool>,
}
impl<'a> GetSsoIntegrationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<SsoIntegration>> {
        let mut r = self.client.client.get("/v3/sso/integrations");
        if let Some(ref unwrapped) = self.si {
            r = r.push_query("si", &unwrapped.to_string());
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
    pub fn si(mut self, si: bool) -> Self {
        self.si = Some(si);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSsoIntegrationsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub completed_integration: Option<bool>,
    pub enabled: bool,
    pub entity_id: String,
    pub name: String,
    pub signin_url: String,
    pub signout_url: String,
}
impl<'a> PostSsoIntegrationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoIntegration> {
        let mut r = self.client.client.post("/v3/sso/integrations");
        if let Some(ref unwrapped) = self.completed_integration {
            r = r.push_json(json!({ "completed_integration" : unwrapped }));
        }
        r = r.push_json(json!({ "enabled" : self.enabled }));
        r = r.push_json(json!({ "entity_id" : self.entity_id }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "signin_url" : self.signin_url }));
        r = r.push_json(json!({ "signout_url" : self.signout_url }));
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
    pub fn completed_integration(mut self, completed_integration: bool) -> Self {
        self.completed_integration = Some(completed_integration);
        self
    }
}
pub struct PostSsoIntegrationsRequired<'a> {
    pub enabled: bool,
    pub entity_id: &'a str,
    pub name: &'a str,
    pub signin_url: &'a str,
    pub signout_url: &'a str,
}
impl<'a> PostSsoIntegrationsRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSsoIntegrationsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub si: Option<bool>,
    pub id: String,
}
impl<'a> GetSsoIntegrationsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoIntegration> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/sso/integrations/{id}", id = self.id));
        if let Some(ref unwrapped) = self.si {
            r = r.push_query("si", &unwrapped.to_string());
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
    pub fn si(mut self, si: bool) -> Self {
        self.si = Some(si);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSsoIntegrationsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> DeleteSsoIntegrationsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/sso/integrations/{id}", id = self.id));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSsoIntegrationsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub si: Option<bool>,
    pub id: String,
    pub completed_integration: Option<bool>,
    pub enabled: bool,
    pub entity_id: String,
    pub name: String,
    pub signin_url: String,
    pub signout_url: String,
}
impl<'a> PatchSsoIntegrationsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoIntegration> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/sso/integrations/{id}", id = self.id));
        if let Some(ref unwrapped) = self.si {
            r = r.push_query("si", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.completed_integration {
            r = r.push_json(json!({ "completed_integration" : unwrapped }));
        }
        r = r.push_json(json!({ "enabled" : self.enabled }));
        r = r.push_json(json!({ "entity_id" : self.entity_id }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "signin_url" : self.signin_url }));
        r = r.push_json(json!({ "signout_url" : self.signout_url }));
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
    pub fn si(mut self, si: bool) -> Self {
        self.si = Some(si);
        self
    }
    pub fn completed_integration(mut self, completed_integration: bool) -> Self {
        self.completed_integration = Some(completed_integration);
        self
    }
}
pub struct PatchSsoIntegrationsIdRequired<'a> {
    pub id: &'a str,
    pub enabled: bool,
    pub entity_id: &'a str,
    pub name: &'a str,
    pub signin_url: &'a str,
    pub signout_url: &'a str,
}
impl<'a> PatchSsoIntegrationsIdRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSsoIntegrationsIntegrationIdCertificatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub integration_id: String,
}
impl<'a> GetSsoIntegrationsIntegrationIdCertificatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<SsoCertificateBody>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/sso/integrations/{integration_id}/certificates", integration_id
                    = self.integration_id
                ),
            );
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSsoTeammatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub email: String,
    pub first_name: String,
    pub is_admin: bool,
    pub is_read_only: bool,
    pub last_name: String,
    pub scopes: Vec<String>,
}
impl<'a> PostSsoTeammatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoTeammateResponse> {
        let mut r = self.client.client.post("/v3/sso/teammates");
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "first_name" : self.first_name }));
        r = r.push_json(json!({ "is_admin" : self.is_admin }));
        r = r.push_json(json!({ "is_read_only" : self.is_read_only }));
        r = r.push_json(json!({ "last_name" : self.last_name }));
        r = r.push_json(json!({ "scopes" : self.scopes }));
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
pub struct PostSsoTeammatesRequired<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub is_admin: bool,
    pub is_read_only: bool,
    pub last_name: &'a str,
    pub scopes: &'a [&'a str],
}
impl<'a> PostSsoTeammatesRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSsoTeammatesUsernameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: String,
    pub first_name: Option<String>,
    pub is_admin: Option<bool>,
    pub last_name: Option<String>,
    pub scopes: Option<Vec<String>>,
}
impl<'a> PatchSsoTeammatesUsernameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SsoTeammatesPatchResponse> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/sso/teammates/{username}", username = self.username));
        if let Some(ref unwrapped) = self.first_name {
            r = r.push_json(json!({ "first_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_admin {
            r = r.push_json(json!({ "is_admin" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.last_name {
            r = r.push_json(json!({ "last_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.scopes {
            r = r.push_json(json!({ "scopes" : unwrapped }));
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
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }
    pub fn is_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = Some(is_admin);
        self
    }
    pub fn last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
        self
    }
    pub fn scopes(mut self, scopes: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.scopes = Some(scopes.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
}
impl<'a> GetStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/stats");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
impl<'a> GetSubusersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Subuser>> {
        let mut r = self.client.client.get("/v3/subusers");
        if let Some(ref unwrapped) = self.username {
            r = r.push_query("username", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSubusersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub email: String,
    pub ips: Vec<String>,
    pub password: String,
    pub username: String,
}
impl<'a> PostSubusersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubuserPost> {
        let mut r = self.client.client.post("/v3/subusers");
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "ips" : self.ips }));
        r = r.push_json(json!({ "password" : self.password }));
        r = r.push_json(json!({ "username" : self.username }));
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
pub struct PostSubusersRequired<'a> {
    pub email: &'a str,
    pub ips: &'a [&'a str],
    pub password: &'a str,
    pub username: &'a str,
}
impl<'a> PostSubusersRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersReputationsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub usernames: Option<String>,
}
impl<'a> GetSubusersReputationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/subusers/reputations");
        if let Some(ref unwrapped) = self.usernames {
            r = r.push_query("usernames", &unwrapped.to_string());
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
    pub fn usernames(mut self, usernames: &str) -> Self {
        self.usernames = Some(usernames.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub subusers: String,
    pub start_date: String,
    pub end_date: Option<String>,
}
impl<'a> GetSubusersStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<CategoryStats>> {
        let mut r = self.client.client.get("/v3/subusers/stats");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        r = r.push_query("subusers", &self.subusers.to_string());
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersStatsMonthlyRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub date: String,
    pub subuser: Option<String>,
    pub sort_by_metric: Option<String>,
    pub sort_by_direction: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
impl<'a> GetSubusersStatsMonthlyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubuserStats> {
        let mut r = self.client.client.get("/v3/subusers/stats/monthly");
        r = r.push_query("date", &self.date.to_string());
        if let Some(ref unwrapped) = self.subuser {
            r = r.push_query("subuser", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_by_metric {
            r = r.push_query("sort_by_metric", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_by_direction {
            r = r.push_query("sort_by_direction", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn subuser(mut self, subuser: &str) -> Self {
        self.subuser = Some(subuser.to_owned());
        self
    }
    pub fn sort_by_metric(mut self, sort_by_metric: &str) -> Self {
        self.sort_by_metric = Some(sort_by_metric.to_owned());
        self
    }
    pub fn sort_by_direction(mut self, sort_by_direction: &str) -> Self {
        self.sort_by_direction = Some(sort_by_direction.to_owned());
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersStatsSumsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub sort_by_direction: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub aggregated_by: Option<String>,
    pub sort_by_metric: Option<String>,
}
impl<'a> GetSubusersStatsSumsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CategoryStats> {
        let mut r = self.client.client.get("/v3/subusers/stats/sums");
        if let Some(ref unwrapped) = self.sort_by_direction {
            r = r.push_query("sort_by_direction", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_by_metric {
            r = r.push_query("sort_by_metric", &unwrapped.to_string());
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
    pub fn sort_by_direction(mut self, sort_by_direction: &str) -> Self {
        self.sort_by_direction = Some(sort_by_direction.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
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
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn sort_by_metric(mut self, sort_by_metric: &str) -> Self {
        self.sort_by_metric = Some(sort_by_metric.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSubusersSubuserNameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
}
impl<'a> DeleteSubusersSubuserNameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v3/subusers/{subuser_name}", subuser_name = self.subuser_name),
            );
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchSubusersSubuserNameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
    pub disabled: Option<bool>,
}
impl<'a> PatchSubusersSubuserNameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(
                &format!("/v3/subusers/{subuser_name}", subuser_name = self.subuser_name),
            );
        if let Some(ref unwrapped) = self.disabled {
            r = r.push_json(json!({ "disabled" : unwrapped }));
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
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutSubusersSubuserNameIpsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
    pub body: serde_json::Value,
}
impl<'a> PutSubusersSubuserNameIpsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/v3/subusers/{subuser_name}/ips", subuser_name = self.subuser_name
                ),
            );
        r = r.push_json(json!({ "body" : self.body }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersSubuserNameMonitorRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
}
impl<'a> GetSubusersSubuserNameMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Monitor> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/subusers/{subuser_name}/monitor", subuser_name = self
                    .subuser_name
                ),
            );
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutSubusersSubuserNameMonitorRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
    pub email: String,
    pub frequency: f64,
}
impl<'a> PutSubusersSubuserNameMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Monitor> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/v3/subusers/{subuser_name}/monitor", subuser_name = self
                    .subuser_name
                ),
            );
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "frequency" : self.frequency }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostSubusersSubuserNameMonitorRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
    pub email: String,
    pub frequency: f64,
}
impl<'a> PostSubusersSubuserNameMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Monitor> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/subusers/{subuser_name}/monitor", subuser_name = self
                    .subuser_name
                ),
            );
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "frequency" : self.frequency }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSubusersSubuserNameMonitorRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub subuser_name: String,
}
impl<'a> DeleteSubusersSubuserNameMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/subusers/{subuser_name}/monitor", subuser_name = self
                    .subuser_name
                ),
            );
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubusersSubuserNameStatsMonthlyRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub date: String,
    pub sort_by_metric: Option<String>,
    pub sort_by_direction: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub subuser_name: String,
}
impl<'a> GetSubusersSubuserNameStatsMonthlyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubuserStats> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/subusers/{subuser_name}/stats/monthly", subuser_name = self
                    .subuser_name
                ),
            );
        r = r.push_query("date", &self.date.to_string());
        if let Some(ref unwrapped) = self.sort_by_metric {
            r = r.push_query("sort_by_metric", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort_by_direction {
            r = r.push_query("sort_by_direction", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn sort_by_metric(mut self, sort_by_metric: &str) -> Self {
        self.sort_by_metric = Some(sort_by_metric.to_owned());
        self
    }
    pub fn sort_by_direction(mut self, sort_by_direction: &str) -> Self {
        self.sort_by_direction = Some(sort_by_direction.to_owned());
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionBlocksRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetSuppressionBlocksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BlocksResponse> {
        let mut r = self.client.client.get("/v3/suppression/blocks");
        if let Some(ref unwrapped) = self.start_time {
            r = r.push_query("start_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.push_query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionBlocksRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub delete_all: Option<bool>,
    pub emails: Option<Vec<String>>,
}
impl<'a> DeleteSuppressionBlocksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/suppression/blocks");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.delete_all {
            r = r.push_json(json!({ "delete_all" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.emails {
            r = r.push_json(json!({ "emails" : unwrapped }));
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
    pub fn delete_all(mut self, delete_all: bool) -> Self {
        self.delete_all = Some(delete_all);
        self
    }
    pub fn emails(mut self, emails: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.emails = Some(emails.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionBlocksEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> GetSuppressionBlocksEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BlocksResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/suppression/blocks/{email}", email = self.email));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionBlocksEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> DeleteSuppressionBlocksEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/suppression/blocks/{email}", email = self.email));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionBouncesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub accept: String,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetSuppressionBouncesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<BounceResponse>> {
        let mut r = self.client.client.get("/v3/suppression/bounces");
        if let Some(ref unwrapped) = self.start_time {
            r = r.push_query("start_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.push_query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        r = r.header("Accept", &self.accept.to_string());
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
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionBouncesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub delete_all: Option<bool>,
    pub emails: Option<Vec<String>>,
}
impl<'a> DeleteSuppressionBouncesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/suppression/bounces");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.delete_all {
            r = r.push_json(json!({ "delete_all" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.emails {
            r = r.push_json(json!({ "emails" : unwrapped }));
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
    pub fn delete_all(mut self, delete_all: bool) -> Self {
        self.delete_all = Some(delete_all);
        self
    }
    pub fn emails(mut self, emails: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.emails = Some(emails.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionBouncesClassificationsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub accept: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetSuppressionBouncesClassificationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/suppression/bounces/classifications");
        r = r.header("Accept", &self.accept.to_string());
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_query("start_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionsBouncesClassificationsClassificationRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub accept: String,
    pub classification: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetSuppressionsBouncesClassificationsClassificationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/suppression/bounces/classifications/{classification}",
                    classification = self.classification
                ),
            );
        r = r.header("Accept", &self.accept.to_string());
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_query("start_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionBouncesEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> GetSuppressionBouncesEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<BounceResponse>> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/suppression/bounces/{email}", email = self.email));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionBouncesEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub email_address: String,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> DeleteSuppressionBouncesEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/suppression/bounces/{email}", email = self.email));
        r = r.push_query("email_address", &self.email_address.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionInvalidEmailsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetSuppressionInvalidEmailsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<InvalidEmail>> {
        let mut r = self.client.client.get("/v3/suppression/invalid_emails");
        if let Some(ref unwrapped) = self.start_time {
            r = r.push_query("start_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.push_query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionInvalidEmailsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub delete_all: Option<bool>,
    pub emails: Option<Vec<String>>,
}
impl<'a> DeleteSuppressionInvalidEmailsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/suppression/invalid_emails");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.delete_all {
            r = r.push_json(json!({ "delete_all" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.emails {
            r = r.push_json(json!({ "emails" : unwrapped }));
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
    pub fn delete_all(mut self, delete_all: bool) -> Self {
        self.delete_all = Some(delete_all);
        self
    }
    pub fn emails(mut self, emails: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.emails = Some(emails.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionInvalidEmailsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> GetSuppressionInvalidEmailsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<InvalidEmail>> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/suppression/invalid_emails/{email}", email = self.email));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionInvalidEmailsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> DeleteSuppressionInvalidEmailsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v3/suppression/invalid_emails/{email}", email = self.email),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionSpamReportsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetSuppressionSpamReportsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SpamReportsResponse> {
        let mut r = self.client.client.get("/v3/suppression/spam_reports");
        if let Some(ref unwrapped) = self.start_time {
            r = r.push_query("start_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.push_query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionSpamReportsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub delete_all: Option<bool>,
    pub emails: Option<Vec<String>>,
}
impl<'a> DeleteSuppressionSpamReportsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/suppression/spam_reports");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.delete_all {
            r = r.push_json(json!({ "delete_all" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.emails {
            r = r.push_json(json!({ "emails" : unwrapped }));
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
    pub fn delete_all(mut self, delete_all: bool) -> Self {
        self.delete_all = Some(delete_all);
        self
    }
    pub fn emails(mut self, emails: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.emails = Some(emails.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionSpamReportsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> GetSuppressionSpamReportsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SpamReportsResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/suppression/spam_reports/{email}", email = self.email));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteSuppressionSpamReportsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
}
impl<'a> DeleteSuppressionSpamReportsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v3/suppression/spam_reports/{email}", email = self.email),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuppressionUnsubscribesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetSuppressionUnsubscribesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/suppression/unsubscribes");
        if let Some(ref unwrapped) = self.start_time {
            r = r.push_query("start_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.push_query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetV3TeammatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetV3TeammatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/teammates");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostV3TeammatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: String,
    pub is_admin: bool,
    pub scopes: Vec<String>,
}
impl<'a> PostV3TeammatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/teammates");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "is_admin" : self.is_admin }));
        r = r.push_json(json!({ "scopes" : self.scopes }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetV3TeammatesPendingRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetV3TeammatesPendingRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/teammates/pending");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteV3TeammatesPendingTokenRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub token: String,
}
impl<'a> DeleteV3TeammatesPendingTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/teammates/pending/{token}", token = self.token));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostV3TeammatesPendingTokenResendRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub token: String,
}
impl<'a> PostV3TeammatesPendingTokenResendRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/teammates/pending/{token}/resend", token = self.token));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetV3TeammatesUsernameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub username: String,
}
impl<'a> GetV3TeammatesUsernameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/teammates/{username}", username = self.username));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteV3TeammatesUsernameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub username: String,
}
impl<'a> DeleteV3TeammatesUsernameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/teammates/{username}", username = self.username));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchV3TeammatesUsernameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub username: String,
    pub is_admin: bool,
    pub scopes: Vec<String>,
}
impl<'a> PatchV3TeammatesUsernameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/teammates/{username}", username = self.username));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "is_admin" : self.is_admin }));
        r = r.push_json(json!({ "scopes" : self.scopes }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTemplatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub generations: Option<String>,
    pub page_size: f64,
    pub page_token: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetTemplatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/templates");
        if let Some(ref unwrapped) = self.generations {
            r = r.push_query("generations", &unwrapped.to_string());
        }
        r = r.push_query("page_size", &self.page_size.to_string());
        if let Some(ref unwrapped) = self.page_token {
            r = r.push_query("page_token", &unwrapped.to_string());
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
    pub fn generations(mut self, generations: &str) -> Self {
        self.generations = Some(generations.to_owned());
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostTemplatesRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub generation: Option<String>,
    pub name: String,
}
impl<'a> PostTemplatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionalTemplate> {
        let mut r = self.client.client.post("/v3/templates");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.generation {
            r = r.push_json(json!({ "generation" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
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
    pub fn generation(mut self, generation: &str) -> Self {
        self.generation = Some(generation.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTemplatesTemplateIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
}
impl<'a> GetTemplatesTemplateIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionalTemplate> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/v3/templates/{template_id}", template_id = self.template_id),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostTemplatesTemplateIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
    pub name: Option<String>,
}
impl<'a> PostTemplatesTemplateIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionalTemplate> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/v3/templates/{template_id}", template_id = self.template_id),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteTemplatesTemplateIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
}
impl<'a> DeleteTemplatesTemplateIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v3/templates/{template_id}", template_id = self.template_id),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchTemplatesTemplateIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
    pub name: Option<String>,
}
impl<'a> PatchTemplatesTemplateIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionalTemplate> {
        let mut r = self
            .client
            .client
            .patch(
                &format!("/v3/templates/{template_id}", template_id = self.template_id),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTemplatesTemplateIdVersionsVersionIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
    pub version_id: String,
}
impl<'a> GetTemplatesTemplateIdVersionsVersionIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionalTemplateVersionOutput> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/templates/{template_id}/versions/{version_id}", template_id =
                    self.template_id, version_id = self.version_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteTemplatesTemplateIdVersionsVersionIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
    pub version_id: String,
}
impl<'a> DeleteTemplatesTemplateIdVersionsVersionIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/templates/{template_id}/versions/{version_id}", template_id =
                    self.template_id, version_id = self.version_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchTemplatesTemplateIdVersionsVersionIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
    pub version_id: String,
    pub active: Option<i64>,
    pub editor: Option<String>,
    pub generate_plain_content: Option<bool>,
    pub html_content: Option<String>,
    pub name: String,
    pub plain_content: Option<String>,
    pub subject: String,
    pub test_data: Option<String>,
}
impl<'a> PatchTemplatesTemplateIdVersionsVersionIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionalTemplateVersionOutput> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/templates/{template_id}/versions/{version_id}", template_id =
                    self.template_id, version_id = self.version_id
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
pub struct PatchTemplatesTemplateIdVersionsVersionIdRequired<'a> {
    pub template_id: &'a str,
    pub version_id: &'a str,
    pub name: &'a str,
    pub subject: &'a str,
}
impl<'a> PatchTemplatesTemplateIdVersionsVersionIdRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostTemplatesTemplateIdVersionsVersionIdActivateRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub template_id: String,
    pub version_id: String,
}
impl<'a> PostTemplatesTemplateIdVersionsVersionIdActivateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionalTemplateVersionOutput> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/templates/{template_id}/versions/{version_id}/activate",
                    template_id = self.template_id, version_id = self.version_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTrackingSettingsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetTrackingSettingsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/tracking_settings");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTrackingSettingsClickRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetTrackingSettingsClickRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ClickTracking> {
        let mut r = self.client.client.get("/v3/tracking_settings/click");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchTrackingSettingsClickRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
}
impl<'a> PatchTrackingSettingsClickRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ClickTracking> {
        let mut r = self.client.client.patch("/v3/tracking_settings/click");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTrackingSettingsGoogleAnalyticsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetTrackingSettingsGoogleAnalyticsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<GoogleAnalyticsSettings> {
        let mut r = self.client.client.get("/v3/tracking_settings/google_analytics");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchTrackingSettingsGoogleAnalyticsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
    pub utm_campaign: Option<String>,
    pub utm_content: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_source: Option<String>,
    pub utm_term: Option<String>,
}
impl<'a> PatchTrackingSettingsGoogleAnalyticsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<GoogleAnalyticsSettings> {
        let mut r = self.client.client.patch("/v3/tracking_settings/google_analytics");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_campaign {
            r = r.push_json(json!({ "utm_campaign" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_content {
            r = r.push_json(json!({ "utm_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_medium {
            r = r.push_json(json!({ "utm_medium" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_source {
            r = r.push_json(json!({ "utm_source" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.utm_term {
            r = r.push_json(json!({ "utm_term" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn utm_campaign(mut self, utm_campaign: &str) -> Self {
        self.utm_campaign = Some(utm_campaign.to_owned());
        self
    }
    pub fn utm_content(mut self, utm_content: &str) -> Self {
        self.utm_content = Some(utm_content.to_owned());
        self
    }
    pub fn utm_medium(mut self, utm_medium: &str) -> Self {
        self.utm_medium = Some(utm_medium.to_owned());
        self
    }
    pub fn utm_source(mut self, utm_source: &str) -> Self {
        self.utm_source = Some(utm_source.to_owned());
        self
    }
    pub fn utm_term(mut self, utm_term: &str) -> Self {
        self.utm_term = Some(utm_term.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTrackingSettingsOpenRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetTrackingSettingsOpenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/tracking_settings/open");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchTrackingSettingsOpenRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
}
impl<'a> PatchTrackingSettingsOpenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.patch("/v3/tracking_settings/open");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTrackingSettingsSubscriptionRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetTrackingSettingsSubscriptionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionTrackingSettings> {
        let mut r = self.client.client.get("/v3/tracking_settings/subscription");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchTrackingSettingsSubscriptionRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: Option<bool>,
    pub html_content: Option<String>,
    pub landing: Option<String>,
    pub plain_content: Option<String>,
    pub replace: Option<String>,
    pub url: Option<String>,
}
impl<'a> PatchTrackingSettingsSubscriptionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionTrackingSettings> {
        let mut r = self.client.client.patch("/v3/tracking_settings/subscription");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.enabled {
            r = r.push_json(json!({ "enabled" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.html_content {
            r = r.push_json(json!({ "html_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.landing {
            r = r.push_json(json!({ "landing" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.plain_content {
            r = r.push_json(json!({ "plain_content" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.replace {
            r = r.push_json(json!({ "replace" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.url {
            r = r.push_json(json!({ "url" : unwrapped }));
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
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    pub fn html_content(mut self, html_content: &str) -> Self {
        self.html_content = Some(html_content.to_owned());
        self
    }
    pub fn landing(mut self, landing: &str) -> Self {
        self.landing = Some(landing.to_owned());
        self
    }
    pub fn plain_content(mut self, plain_content: &str) -> Self {
        self.plain_content = Some(plain_content.to_owned());
        self
    }
    pub fn replace(mut self, replace: &str) -> Self {
        self.replace = Some(replace.to_owned());
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserAccountRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/user/account");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserCreditsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserCreditsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/user/credits");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/user/email");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutUserEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub email: Option<String>,
}
impl<'a> PutUserEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.put("/v3/user/email");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_json(json!({ "email" : unwrapped }));
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
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutUserPasswordRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub new_password: String,
    pub old_password: String,
}
impl<'a> PutUserPasswordRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.put("/v3/user/password");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "new_password" : self.new_password }));
        r = r.push_json(json!({ "old_password" : self.old_password }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserProfileRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserProfileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/user/profile");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserProfileRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub address: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub state: Option<String>,
    pub website: Option<String>,
    pub zip: Option<String>,
}
impl<'a> PatchUserProfileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UserProfile> {
        let mut r = self.client.client.patch("/v3/user/profile");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address2 {
            r = r.push_json(json!({ "address2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.city {
            r = r.push_json(json!({ "city" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.company {
            r = r.push_json(json!({ "company" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country {
            r = r.push_json(json!({ "country" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.first_name {
            r = r.push_json(json!({ "first_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.last_name {
            r = r.push_json(json!({ "last_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.phone {
            r = r.push_json(json!({ "phone" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.state {
            r = r.push_json(json!({ "state" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.website {
            r = r.push_json(json!({ "website" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.zip {
            r = r.push_json(json!({ "zip" : unwrapped }));
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
    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_owned());
        self
    }
    pub fn address2(mut self, address2: &str) -> Self {
        self.address2 = Some(address2.to_owned());
        self
    }
    pub fn city(mut self, city: &str) -> Self {
        self.city = Some(city.to_owned());
        self
    }
    pub fn company(mut self, company: &str) -> Self {
        self.company = Some(company.to_owned());
        self
    }
    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }
    pub fn last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
        self
    }
    pub fn phone(mut self, phone: &str) -> Self {
        self.phone = Some(phone.to_owned());
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn website(mut self, website: &str) -> Self {
        self.website = Some(website.to_owned());
        self
    }
    pub fn zip(mut self, zip: &str) -> Self {
        self.zip = Some(zip.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserScheduledSendsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserScheduledSendsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<UserScheduledSendStatus>> {
        let mut r = self.client.client.get("/v3/user/scheduled_sends");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostUserScheduledSendsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub batch_id: String,
    pub status: String,
}
impl<'a> PostUserScheduledSendsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UserScheduledSendStatus> {
        let mut r = self.client.client.post("/v3/user/scheduled_sends");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "batch_id" : self.batch_id }));
        r = r.push_json(json!({ "status" : self.status }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserScheduledSendsBatchIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub batch_id: String,
}
impl<'a> GetUserScheduledSendsBatchIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<UserScheduledSendStatus>> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/v3/user/scheduled_sends/{batch_id}", batch_id = self.batch_id),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteUserScheduledSendsBatchIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub batch_id: String,
}
impl<'a> DeleteUserScheduledSendsBatchIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/v3/user/scheduled_sends/{batch_id}", batch_id = self.batch_id),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserScheduledSendsBatchIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub batch_id: String,
    pub status: String,
}
impl<'a> PatchUserScheduledSendsBatchIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(
                &format!("/v3/user/scheduled_sends/{batch_id}", batch_id = self.batch_id),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "status" : self.status }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserSettingsEnforcedTlsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserSettingsEnforcedTlsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EnforcedTlsRequestResponse> {
        let mut r = self.client.client.get("/v3/user/settings/enforced_tls");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserSettingsEnforcedTlsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub require_tls: Option<bool>,
    pub require_valid_cert: Option<bool>,
    pub version: Option<f64>,
}
impl<'a> PatchUserSettingsEnforcedTlsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EnforcedTlsRequestResponse> {
        let mut r = self.client.client.patch("/v3/user/settings/enforced_tls");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.require_tls {
            r = r.push_json(json!({ "require_tls" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.require_valid_cert {
            r = r.push_json(json!({ "require_valid_cert" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.version {
            r = r.push_json(json!({ "version" : unwrapped }));
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
    pub fn require_tls(mut self, require_tls: bool) -> Self {
        self.require_tls = Some(require_tls);
        self
    }
    pub fn require_valid_cert(mut self, require_valid_cert: bool) -> Self {
        self.require_valid_cert = Some(require_valid_cert);
        self
    }
    pub fn version(mut self, version: f64) -> Self {
        self.version = Some(version);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserUsernameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserUsernameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/user/username");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutUserUsernameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub username: Option<String>,
}
impl<'a> PutUserUsernameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.put("/v3/user/username");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.username {
            r = r.push_json(json!({ "username" : unwrapped }));
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
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserWebhooksEventSettingsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserWebhooksEventSettingsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EventWebhookResponse> {
        let mut r = self.client.client.get("/v3/user/webhooks/event/settings");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserWebhooksEventSettingsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub bounce: bool,
    pub click: bool,
    pub deferred: bool,
    pub delivered: bool,
    pub dropped: bool,
    pub enabled: bool,
    pub group_resubscribe: bool,
    pub group_unsubscribe: bool,
    pub oauth_client_id: Option<String>,
    pub oauth_client_secret: Option<String>,
    pub oauth_token_url: Option<String>,
    pub open: bool,
    pub processed: bool,
    pub spam_report: bool,
    pub unsubscribe: bool,
    pub url: String,
}
impl<'a> PatchUserWebhooksEventSettingsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EventWebhookResponse> {
        let mut r = self.client.client.patch("/v3/user/webhooks/event/settings");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "bounce" : self.bounce }));
        r = r.push_json(json!({ "click" : self.click }));
        r = r.push_json(json!({ "deferred" : self.deferred }));
        r = r.push_json(json!({ "delivered" : self.delivered }));
        r = r.push_json(json!({ "dropped" : self.dropped }));
        r = r.push_json(json!({ "enabled" : self.enabled }));
        r = r.push_json(json!({ "group_resubscribe" : self.group_resubscribe }));
        r = r.push_json(json!({ "group_unsubscribe" : self.group_unsubscribe }));
        if let Some(ref unwrapped) = self.oauth_client_id {
            r = r.push_json(json!({ "oauth_client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.oauth_client_secret {
            r = r.push_json(json!({ "oauth_client_secret" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.oauth_token_url {
            r = r.push_json(json!({ "oauth_token_url" : unwrapped }));
        }
        r = r.push_json(json!({ "open" : self.open }));
        r = r.push_json(json!({ "processed" : self.processed }));
        r = r.push_json(json!({ "spam_report" : self.spam_report }));
        r = r.push_json(json!({ "unsubscribe" : self.unsubscribe }));
        r = r.push_json(json!({ "url" : self.url }));
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
    pub fn oauth_client_id(mut self, oauth_client_id: &str) -> Self {
        self.oauth_client_id = Some(oauth_client_id.to_owned());
        self
    }
    pub fn oauth_client_secret(mut self, oauth_client_secret: &str) -> Self {
        self.oauth_client_secret = Some(oauth_client_secret.to_owned());
        self
    }
    pub fn oauth_token_url(mut self, oauth_token_url: &str) -> Self {
        self.oauth_token_url = Some(oauth_token_url.to_owned());
        self
    }
}
pub struct PatchUserWebhooksEventSettingsRequired<'a> {
    pub bounce: bool,
    pub click: bool,
    pub deferred: bool,
    pub delivered: bool,
    pub dropped: bool,
    pub enabled: bool,
    pub group_resubscribe: bool,
    pub group_unsubscribe: bool,
    pub open: bool,
    pub processed: bool,
    pub spam_report: bool,
    pub unsubscribe: bool,
    pub url: &'a str,
}
impl<'a> PatchUserWebhooksEventSettingsRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserWebhooksEventSettingsSignedRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserWebhooksEventSettingsSignedRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/user/webhooks/event/settings/signed");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserWebhooksEventSettingsSignedRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub enabled: bool,
}
impl<'a> PatchUserWebhooksEventSettingsSignedRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.patch("/v3/user/webhooks/event/settings/signed");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "enabled" : self.enabled }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostUserWebhooksEventTestRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub oauth_client_id: Option<String>,
    pub oauth_client_secret: Option<String>,
    pub oauth_token_url: Option<String>,
    pub url: Option<String>,
}
impl<'a> PostUserWebhooksEventTestRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/v3/user/webhooks/event/test");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.oauth_client_id {
            r = r.push_json(json!({ "oauth_client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.oauth_client_secret {
            r = r.push_json(json!({ "oauth_client_secret" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.oauth_token_url {
            r = r.push_json(json!({ "oauth_token_url" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.url {
            r = r.push_json(json!({ "url" : unwrapped }));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
    pub fn oauth_client_id(mut self, oauth_client_id: &str) -> Self {
        self.oauth_client_id = Some(oauth_client_id.to_owned());
        self
    }
    pub fn oauth_client_secret(mut self, oauth_client_secret: &str) -> Self {
        self.oauth_client_secret = Some(oauth_client_secret.to_owned());
        self
    }
    pub fn oauth_token_url(mut self, oauth_token_url: &str) -> Self {
        self.oauth_token_url = Some(oauth_token_url.to_owned());
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserWebhooksParseSettingsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserWebhooksParseSettingsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/user/webhooks/parse/settings");
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostUserWebhooksParseSettingsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub hostname: Option<String>,
    pub send_raw: Option<bool>,
    pub spam_check: Option<bool>,
    pub url: Option<String>,
}
impl<'a> PostUserWebhooksParseSettingsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ParseSetting> {
        let mut r = self.client.client.post("/v3/user/webhooks/parse/settings");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.hostname {
            r = r.push_json(json!({ "hostname" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.send_raw {
            r = r.push_json(json!({ "send_raw" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.spam_check {
            r = r.push_json(json!({ "spam_check" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.url {
            r = r.push_json(json!({ "url" : unwrapped }));
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
    pub fn hostname(mut self, hostname: &str) -> Self {
        self.hostname = Some(hostname.to_owned());
        self
    }
    pub fn send_raw(mut self, send_raw: bool) -> Self {
        self.send_raw = Some(send_raw);
        self
    }
    pub fn spam_check(mut self, spam_check: bool) -> Self {
        self.spam_check = Some(spam_check);
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserWebhooksParseSettingsHostnameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub hostname: String,
}
impl<'a> GetUserWebhooksParseSettingsHostnameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ParseSetting> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/user/webhooks/parse/settings/{hostname}", hostname = self
                    .hostname
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteUserWebhooksParseSettingsHostnameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub hostname: String,
}
impl<'a> DeleteUserWebhooksParseSettingsHostnameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/user/webhooks/parse/settings/{hostname}", hostname = self
                    .hostname
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchUserWebhooksParseSettingsHostnameRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub hostname: String,
    pub send_raw: Option<bool>,
    pub spam_check: Option<bool>,
    pub url: Option<String>,
}
impl<'a> PatchUserWebhooksParseSettingsHostnameRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ParseSetting> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/user/webhooks/parse/settings/{hostname}", hostname = self
                    .hostname
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.send_raw {
            r = r.push_json(json!({ "send_raw" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.spam_check {
            r = r.push_json(json!({ "spam_check" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.url {
            r = r.push_json(json!({ "url" : unwrapped }));
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
    pub fn send_raw(mut self, send_raw: bool) -> Self {
        self.send_raw = Some(send_raw);
        self
    }
    pub fn spam_check(mut self, spam_check: bool) -> Self {
        self.spam_check = Some(spam_check);
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUserWebhooksParseStatsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<String>,
    pub offset: Option<String>,
    pub aggregated_by: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetUserWebhooksParseStatsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/v3/user/webhooks/parse/stats");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.aggregated_by {
            r = r.push_query("aggregated_by", &unwrapped.to_string());
        }
        r = r.push_query("start_date", &self.start_date.to_string());
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_query("end_date", &unwrapped.to_string());
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
    pub fn limit(mut self, limit: &str) -> Self {
        self.limit = Some(limit.to_owned());
        self
    }
    pub fn offset(mut self, offset: &str) -> Self {
        self.offset = Some(offset.to_owned());
        self
    }
    pub fn aggregated_by(mut self, aggregated_by: &str) -> Self {
        self.aggregated_by = Some(aggregated_by.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostValidationsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub email: String,
    pub source: Option<String>,
}
impl<'a> PostValidationsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/v3/validations/email");
        r = r.push_json(json!({ "email" : self.email }));
        if let Some(ref unwrapped) = self.source {
            r = r.push_json(json!({ "source" : unwrapped }));
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
    pub fn source(mut self, source: &str) -> Self {
        self.source = Some(source.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetVerifiedSendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<f64>,
    pub last_seen_id: Option<f64>,
    pub id: Option<i64>,
}
impl<'a> GetVerifiedSendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/verified_senders");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.last_seen_id {
            r = r.push_query("lastSeenID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.id {
            r = r.push_query("id", &unwrapped.to_string());
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
    pub fn limit(mut self, limit: f64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn last_seen_id(mut self, last_seen_id: f64) -> Self {
        self.last_seen_id = Some(last_seen_id);
        self
    }
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostVerifiedSendersRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub address: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub from_email: String,
    pub from_name: Option<String>,
    pub nickname: String,
    pub reply_to: String,
    pub reply_to_name: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl<'a> PostVerifiedSendersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<VerifiedSenderResponseSchema> {
        let mut r = self.client.client.post("/v3/verified_senders");
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address2 {
            r = r.push_json(json!({ "address2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.city {
            r = r.push_json(json!({ "city" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country {
            r = r.push_json(json!({ "country" : unwrapped }));
        }
        r = r.push_json(json!({ "from_email" : self.from_email }));
        if let Some(ref unwrapped) = self.from_name {
            r = r.push_json(json!({ "from_name" : unwrapped }));
        }
        r = r.push_json(json!({ "nickname" : self.nickname }));
        r = r.push_json(json!({ "reply_to" : self.reply_to }));
        if let Some(ref unwrapped) = self.reply_to_name {
            r = r.push_json(json!({ "reply_to_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.state {
            r = r.push_json(json!({ "state" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.zip {
            r = r.push_json(json!({ "zip" : unwrapped }));
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
    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_owned());
        self
    }
    pub fn address2(mut self, address2: &str) -> Self {
        self.address2 = Some(address2.to_owned());
        self
    }
    pub fn city(mut self, city: &str) -> Self {
        self.city = Some(city.to_owned());
        self
    }
    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }
    pub fn from_name(mut self, from_name: &str) -> Self {
        self.from_name = Some(from_name.to_owned());
        self
    }
    pub fn reply_to_name(mut self, reply_to_name: &str) -> Self {
        self.reply_to_name = Some(reply_to_name.to_owned());
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn zip(mut self, zip: &str) -> Self {
        self.zip = Some(zip.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetVerifiedSendersDomainsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetVerifiedSendersDomainsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/verified_senders/domains");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostVerifiedSendersResendIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> PostVerifiedSendersResendIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/verified_senders/resend/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetVerifiedSendersStepsCompletedRequest<'a> {
    pub(crate) client: &'a SendgridClient,
}
impl<'a> GetVerifiedSendersStepsCompletedRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v3/verified_senders/steps_completed");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetVerifiedSendersVerifyTokenRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub token: String,
}
impl<'a> GetVerifiedSendersVerifyTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/verified_senders/verify/{token}", token = self.token));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteVerifiedSendersIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
}
impl<'a> DeleteVerifiedSendersIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/verified_senders/{id}", id = self.id));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchVerifiedSendersIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub id: String,
    pub address: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub from_email: String,
    pub from_name: Option<String>,
    pub nickname: String,
    pub reply_to: String,
    pub reply_to_name: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl<'a> PatchVerifiedSendersIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<VerifiedSenderResponseSchema> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/verified_senders/{id}", id = self.id));
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address2 {
            r = r.push_json(json!({ "address2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.city {
            r = r.push_json(json!({ "city" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country {
            r = r.push_json(json!({ "country" : unwrapped }));
        }
        r = r.push_json(json!({ "from_email" : self.from_email }));
        if let Some(ref unwrapped) = self.from_name {
            r = r.push_json(json!({ "from_name" : unwrapped }));
        }
        r = r.push_json(json!({ "nickname" : self.nickname }));
        r = r.push_json(json!({ "reply_to" : self.reply_to }));
        if let Some(ref unwrapped) = self.reply_to_name {
            r = r.push_json(json!({ "reply_to_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.state {
            r = r.push_json(json!({ "state" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.zip {
            r = r.push_json(json!({ "zip" : unwrapped }));
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
    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_owned());
        self
    }
    pub fn address2(mut self, address2: &str) -> Self {
        self.address2 = Some(address2.to_owned());
        self
    }
    pub fn city(mut self, city: &str) -> Self {
        self.city = Some(city.to_owned());
        self
    }
    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }
    pub fn from_name(mut self, from_name: &str) -> Self {
        self.from_name = Some(from_name.to_owned());
        self
    }
    pub fn reply_to_name(mut self, reply_to_name: &str) -> Self {
        self.reply_to_name = Some(reply_to_name.to_owned());
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn zip(mut self, zip: &str) -> Self {
        self.zip = Some(zip.to_owned());
        self
    }
}
pub struct PatchVerifiedSendersIdRequired<'a> {
    pub id: &'a str,
    pub from_email: &'a str,
    pub nickname: &'a str,
    pub reply_to: &'a str,
}
impl<'a> PatchVerifiedSendersIdRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelDnsEmailRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub domain_id: i64,
    pub email: String,
    pub link_id: i64,
    pub message: Option<String>,
}
impl<'a> PostWhitelabelDnsEmailRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/v3/whitelabel/dns/email");
        r = r.push_json(json!({ "domain_id" : self.domain_id }));
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "link_id" : self.link_id }));
        if let Some(ref unwrapped) = self.message {
            r = r.push_json(json!({ "message" : unwrapped }));
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
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_owned());
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelDomainsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub automatic_security: Option<bool>,
    pub custom_dkim_selector: Option<String>,
    pub custom_spf: Option<bool>,
    pub default: Option<bool>,
    pub domain: String,
    pub ips: Option<Vec<String>>,
    pub subdomain: Option<String>,
    pub username: Option<String>,
}
impl<'a> PostWhitelabelDomainsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AuthenticationDomain> {
        let mut r = self.client.client.post("/v3/whitelabel/domains");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.automatic_security {
            r = r.push_json(json!({ "automatic_security" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_dkim_selector {
            r = r.push_json(json!({ "custom_dkim_selector" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_spf {
            r = r.push_json(json!({ "custom_spf" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.default {
            r = r.push_json(json!({ "default" : unwrapped }));
        }
        r = r.push_json(json!({ "domain" : self.domain }));
        if let Some(ref unwrapped) = self.ips {
            r = r.push_json(json!({ "ips" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subdomain {
            r = r.push_json(json!({ "subdomain" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.username {
            r = r.push_json(json!({ "username" : unwrapped }));
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
    pub fn automatic_security(mut self, automatic_security: bool) -> Self {
        self.automatic_security = Some(automatic_security);
        self
    }
    pub fn custom_dkim_selector(mut self, custom_dkim_selector: &str) -> Self {
        self.custom_dkim_selector = Some(custom_dkim_selector.to_owned());
        self
    }
    pub fn custom_spf(mut self, custom_spf: bool) -> Self {
        self.custom_spf = Some(custom_spf);
        self
    }
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
    pub fn ips(mut self, ips: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ips = Some(ips.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn subdomain(mut self, subdomain: &str) -> Self {
        self.subdomain = Some(subdomain.to_owned());
        self
    }
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelDomainsDefaultRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub domain: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetWhitelabelDomainsDefaultRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthentication200Response> {
        let mut r = self.client.client.get("/v3/whitelabel/domains/default");
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
    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelDomainsSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: String,
}
impl<'a> GetWhitelabelDomainsSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthenticationDomainSpf> {
        let mut r = self.client.client.get("/v3/whitelabel/domains/subuser");
        r = r.push_query("username", &self.username.to_string());
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteWhitelabelDomainsSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: Option<String>,
}
impl<'a> DeleteWhitelabelDomainsSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/whitelabel/domains/subuser");
        if let Some(ref unwrapped) = self.username {
            r = r.push_query("username", &unwrapped.to_string());
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
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelDomainsDomainIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub domain_id: String,
}
impl<'a> GetWhitelabelDomainsDomainIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AuthenticationDomain> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v3/whitelabel/domains/{domain_id}", domain_id = self.domain_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteWhitelabelDomainsDomainIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub domain_id: String,
}
impl<'a> DeleteWhitelabelDomainsDomainIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/whitelabel/domains/{domain_id}", domain_id = self.domain_id
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchWhitelabelDomainsDomainIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub domain_id: String,
    pub custom_spf: Option<bool>,
    pub default: Option<bool>,
}
impl<'a> PatchWhitelabelDomainsDomainIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthentication200Response> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/v3/whitelabel/domains/{domain_id}", domain_id = self.domain_id
                ),
            );
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.custom_spf {
            r = r.push_json(json!({ "custom_spf" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.default {
            r = r.push_json(json!({ "default" : unwrapped }));
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
    pub fn custom_spf(mut self, custom_spf: bool) -> Self {
        self.custom_spf = Some(custom_spf);
        self
    }
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelDomainsDomainIdSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub domain_id: i64,
    pub username: String,
}
impl<'a> PostWhitelabelDomainsDomainIdSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthenticationDomainSpf> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/whitelabel/domains/{domain_id}/subuser", domain_id = self
                    .domain_id
                ),
            );
        r = r.push_json(json!({ "username" : self.username }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelDomainsIdIpsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: i64,
    pub ip: String,
}
impl<'a> PostWhitelabelDomainsIdIpsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthenticationDomainSpf> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/whitelabel/domains/{id}/ips", id = self.id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "ip" : self.ip }));
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteWhitelabelDomainsIdIpsIpRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: i64,
    pub ip: String,
}
impl<'a> DeleteWhitelabelDomainsIdIpsIpRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DomainAuthenticationDomainSpf> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v3/whitelabel/domains/{id}/ips/{ip}", id = self.id, ip = self.ip
                ),
            );
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelDomainsIdValidateRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: i64,
}
impl<'a> PostWhitelabelDomainsIdValidateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/whitelabel/domains/{id}/validate", id = self.id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelIpsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub ip: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetWhitelabelIpsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<ReverseDns>> {
        let mut r = self.client.client.get("/v3/whitelabel/ips");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_query("offset", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ip {
            r = r.push_query("ip", &unwrapped.to_string());
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
    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = Some(ip.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelIpsRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub domain: String,
    pub ip: String,
    pub subdomain: Option<String>,
}
impl<'a> PostWhitelabelIpsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ReverseDns> {
        let mut r = self.client.client.post("/v3/whitelabel/ips");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "domain" : self.domain }));
        r = r.push_json(json!({ "ip" : self.ip }));
        if let Some(ref unwrapped) = self.subdomain {
            r = r.push_json(json!({ "subdomain" : unwrapped }));
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
    pub fn subdomain(mut self, subdomain: &str) -> Self {
        self.subdomain = Some(subdomain.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelIpsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: String,
}
impl<'a> GetWhitelabelIpsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ReverseDns> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/whitelabel/ips/{id}", id = self.id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteWhitelabelIpsIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: String,
}
impl<'a> DeleteWhitelabelIpsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/whitelabel/ips/{id}", id = self.id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelIpsIdValidateRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: String,
}
impl<'a> PostWhitelabelIpsIdValidateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/whitelabel/ips/{id}/validate", id = self.id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelLinksRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub limit: Option<i64>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetWhitelabelLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<LinkBranding200Response>> {
        let mut r = self.client.client.get("/v3/whitelabel/links");
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelLinksRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub default: Option<bool>,
    pub domain: String,
    pub subdomain: Option<String>,
}
impl<'a> PostWhitelabelLinksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkBranding200Response> {
        let mut r = self.client.client.post("/v3/whitelabel/links");
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.default {
            r = r.push_json(json!({ "default" : unwrapped }));
        }
        r = r.push_json(json!({ "domain" : self.domain }));
        if let Some(ref unwrapped) = self.subdomain {
            r = r.push_json(json!({ "subdomain" : unwrapped }));
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
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
    pub fn subdomain(mut self, subdomain: &str) -> Self {
        self.subdomain = Some(subdomain.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelLinksDefaultRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub domain: Option<String>,
    pub on_behalf_of: Option<String>,
}
impl<'a> GetWhitelabelLinksDefaultRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkBranding200Response> {
        let mut r = self.client.client.get("/v3/whitelabel/links/default");
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
    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_owned());
        self
    }
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelLinksSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: String,
}
impl<'a> GetWhitelabelLinksSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkBranding200Response> {
        let mut r = self.client.client.get("/v3/whitelabel/links/subuser");
        r = r.push_query("username", &self.username.to_string());
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteWhitelabelLinksSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub username: String,
}
impl<'a> DeleteWhitelabelLinksSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.delete("/v3/whitelabel/links/subuser");
        r = r.push_query("username", &self.username.to_string());
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWhitelabelLinksIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: i64,
}
impl<'a> GetWhitelabelLinksIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkBranding200Response> {
        let mut r = self
            .client
            .client
            .get(&format!("/v3/whitelabel/links/{id}", id = self.id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteWhitelabelLinksIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: i64,
}
impl<'a> DeleteWhitelabelLinksIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/v3/whitelabel/links/{id}", id = self.id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchWhitelabelLinksIdRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: i64,
    pub default: Option<bool>,
}
impl<'a> PatchWhitelabelLinksIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkBranding200Response> {
        let mut r = self
            .client
            .client
            .patch(&format!("/v3/whitelabel/links/{id}", id = self.id));
        if let Some(ref unwrapped) = self.on_behalf_of {
            r = r.header("on-behalf-of", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.default {
            r = r.push_json(json!({ "default" : unwrapped }));
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
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelLinksIdValidateRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub on_behalf_of: Option<String>,
    pub id: i64,
}
impl<'a> PostWhitelabelLinksIdValidateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/v3/whitelabel/links/{id}/validate", id = self.id));
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
    pub fn on_behalf_of(mut self, on_behalf_of: &str) -> Self {
        self.on_behalf_of = Some(on_behalf_of.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostWhitelabelLinksLinkIdSubuserRequest<'a> {
    pub(crate) client: &'a SendgridClient,
    pub link_id: i64,
    pub username: Option<String>,
}
impl<'a> PostWhitelabelLinksLinkIdSubuserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkBranding200Response> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/v3/whitelabel/links/{link_id}/subuser", link_id = self.link_id
                ),
            );
        if let Some(ref unwrapped) = self.username {
            r = r.push_json(json!({ "username" : unwrapped }));
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
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
        self
    }
}
