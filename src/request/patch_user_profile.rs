use serde_json::json;
use crate::model::*;
use crate::SendgridClient;
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
