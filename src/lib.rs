//! [`SendgridClient`](struct.SendgridClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct SendgridClient {
    pub(crate) client: httpclient::Client,
    authentication: SendgridAuthentication,
}
impl SendgridClient {
    pub fn from_env() -> Self {
        let url = "https://api.sendgrid.com".to_string();
        Self {
            client: httpclient::Client::new(Some(url)),
            authentication: SendgridAuthentication::from_env(),
        }
    }
}
impl SendgridClient {
    pub fn new(url: &str, authentication: SendgridAuthentication) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        Self { client, authentication }
    }
    pub fn with_authentication(
        mut self,
        authentication: SendgridAuthentication,
    ) -> Self {
        self.authentication = authentication;
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            SendgridAuthentication::Authorization { authorization } => {
                r = r.header("Authorization", authorization);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**Retrieve all recent access attempts

**This endpoint allows you to retrieve a list of all of the IP addresses that recently attempted to access your account either through the User Interface or the API.***/
    pub fn get_access_settings_activity(
        &self,
    ) -> request::GetAccessSettingsActivityRequest {
        request::GetAccessSettingsActivityRequest {
            client: &self,
            limit: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve a list of currently allowed IPs

**This endpoint allows you to retrieve a list of IP addresses that are currently allowed to access your account.**

Each IP address returned to you will have `created_at` and `updated_at` dates. Each IP will also be associated with an `id` that can be used to remove the address from your allow list.*/
    pub fn get_access_settings_whitelist(
        &self,
    ) -> request::GetAccessSettingsWhitelistRequest {
        request::GetAccessSettingsWhitelistRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Add one or more IPs to the allow list

**This endpoint allows you to add one or more allowed IP addresses.**

To allow one or more IP addresses, pass them to this endpoint in an array. Once an IP address is added to your allow list, it will be assigned an `id` that can be used to remove the address. You can retrieve the ID associated with an IP using the "Retrieve a list of currently allowed IPs" endpoint.*/
    pub fn post_access_settings_whitelist(
        &self,
        ips: Vec<serde_json::Value>,
    ) -> request::PostAccessSettingsWhitelistRequest {
        request::PostAccessSettingsWhitelistRequest {
            client: &self,
            on_behalf_of: None,
            ips,
        }
    }
    /**Remove one or more IPs from the allow list

**This endpoint allows you to remove one or more IP addresses from your list of allowed addresses.**

To remove one or more IP addresses, pass this endpoint an array containing the ID(s) associated with the IP(s) you intend to remove. You can retrieve the IDs associated with your allowed IP addresses using the "Retrieve a list of currently allowed IPs" endpoint.

It is possible to remove your own IP address, which will block access to your account. You will need to submit a [support ticket](https://sendgrid.com/docs/ui/account-and-settings/support/) if this happens. For this reason, it is important to double check that you are removing only the IPs you intend to remove when using this endpoint.*/
    pub fn delete_access_settings_whitelist(
        &self,
    ) -> request::DeleteAccessSettingsWhitelistRequest {
        request::DeleteAccessSettingsWhitelistRequest {
            client: &self,
            on_behalf_of: None,
            ids: None,
        }
    }
    /**Retrieve a specific allowed IP

**This endpoint allows you to retreive a specific IP address that has been allowed to access your account.**

You must include the ID for the specific IP address you want to retrieve in your call. You can retrieve the IDs associated with your allowed IP addresses using the "Retrieve a  list of currently allowed IPs" endpoint.*/
    pub fn get_access_settings_whitelist_rule_id(
        &self,
        rule_id: &str,
    ) -> request::GetAccessSettingsWhitelistRuleIdRequest {
        request::GetAccessSettingsWhitelistRuleIdRequest {
            client: &self,
            on_behalf_of: None,
            rule_id: rule_id.to_owned(),
        }
    }
    /**Remove a specific IP from the allowed list

**This endpoint allows you to remove a specific IP address from your list of allowed addresses.**

When removing a specific IP address from your list, you must include the ID in your call.  You can retrieve the IDs associated with your allowed IP addresses using the "Retrieve a list of currently allowed IPs" endpoint.*/
    pub fn delete_access_settings_whitelist_rule_id(
        &self,
        rule_id: &str,
    ) -> request::DeleteAccessSettingsWhitelistRuleIdRequest {
        request::DeleteAccessSettingsWhitelistRuleIdRequest {
            client: &self,
            on_behalf_of: None,
            rule_id: rule_id.to_owned(),
        }
    }
    /**Retrieve all alerts

**This endpoint allows you to retrieve all of your alerts.**

Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
* Usage alerts allow you to set the threshold at which an alert will be sent.
* Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".

For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).*/
    pub fn get_alerts(&self) -> request::GetAlertsRequest {
        request::GetAlertsRequest {
            client: &self,
            authorization: None,
            on_behalf_of: None,
        }
    }
    /**Create a new Alert

**This endpoint allows you to create a new alert.**

Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics. There are two types of alerts that can be created with this endpoint:

* `usage_limit` allows you to set the threshold at which an alert will be sent.
* `stats_notification` allows you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".

For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).*/
    pub fn post_alerts(&self, type_: &str) -> request::PostAlertsRequest {
        request::PostAlertsRequest {
            client: &self,
            authorization: None,
            on_behalf_of: None,
            email_to: None,
            frequency: None,
            percentage: None,
            type_: type_.to_owned(),
        }
    }
    /**Retrieve a specific alert

**This endpoint allows you to retrieve a specific alert.**

Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
* Usage alerts allow you to set the threshold at which an alert will be sent.
* Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".

For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).*/
    pub fn get_alerts_alert_id(
        &self,
        alert_id: i64,
    ) -> request::GetAlertsAlertIdRequest {
        request::GetAlertsAlertIdRequest {
            client: &self,
            authorization: None,
            on_behalf_of: None,
            alert_id,
        }
    }
    /**Delete an alert

**This endpoint allows you to delete an alert.**

Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
* Usage alerts allow you to set the threshold at which an alert will be sent.
* Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".

For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).*/
    pub fn delete_alerts_alert_id(
        &self,
        alert_id: i64,
    ) -> request::DeleteAlertsAlertIdRequest {
        request::DeleteAlertsAlertIdRequest {
            client: &self,
            on_behalf_of: None,
            alert_id,
        }
    }
    /**Update an alert

**This endpoint allows you to update an alert.**

Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
* Usage alerts allow you to set the threshold at which an alert will be sent.
* Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".

For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).*/
    pub fn patch_alerts_alert_id(
        &self,
        alert_id: i64,
    ) -> request::PatchAlertsAlertIdRequest {
        request::PatchAlertsAlertIdRequest {
            client: &self,
            on_behalf_of: None,
            alert_id,
            email_to: None,
            frequency: None,
            percentage: None,
        }
    }
    /**Retrieve all API Keys belonging to the authenticated user

**This endpoint allows you to retrieve all API Keys that belong to the authenticated user.**

A successful response from this API will include all available API keys' names and IDs.

For security reasons, there is not a way to retrieve the key itself after it's created. If you lose your API key, you must create a new one. Only the "Create API keys" endpoint will return a key to you and only at the time of creation.

An `api_key_id` can be used to update or delete the key, as well as retrieve the key's details, such as its scopes.*/
    pub fn get_api_keys(&self) -> request::GetApiKeysRequest {
        request::GetApiKeysRequest {
            client: &self,
            limit: None,
            on_behalf_of: None,
        }
    }
    /**Create API keys

**This endpoint allows you to create a new API Key for the user.**

To create your initial SendGrid API Key, you should [use the SendGrid App](https://app.sendgrid.com/settings/api_keys). Once you have created a first key with scopes to manage additional API keys, you can use this API for all other key management.

> There is a limit of 100 API Keys on your account.

A JSON request body containing a `name` property is required when making requests to this endpoint. If the number of maximum keys, 100, is reached, a `403` status will be returned.

Though the `name` field is required, it does not need to be unique. A unique API key ID will be generated for each key you create and returned in the response body.

It is not necessary to pass a `scopes` field to the API when creating a key, but you should be aware that omitting the `scopes` field from your request will create a key with "Full Access" permissions by default.

See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes. An API key's scopes can be updated after creation using the "Update API keys" endpoint.*/
    pub fn create_api_keys(&self, name: &str) -> request::CreateApiKeysRequest {
        request::CreateApiKeysRequest {
            client: &self,
            on_behalf_of: None,
            name: name.to_owned(),
            scopes: None,
        }
    }
    /**Retrieve an existing API Key

**This endpoint allows you to retrieve a single API key using an `api_key_id`.**

The endpoint will return a key's name, ID, and scopes. If the API Key ID does not, exist a `404` status will be returned.

See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes. An API key's scopes can be updated after creation using the "Update API keys" endpoint.*/
    pub fn get_api_keys_api_key_id(
        &self,
        api_key_id: &str,
    ) -> request::GetApiKeysApiKeyIdRequest {
        request::GetApiKeysApiKeyIdRequest {
            client: &self,
            on_behalf_of: None,
            api_key_id: api_key_id.to_owned(),
        }
    }
    /**Update API key name and scopes

**This endpoint allows you to update the name and scopes of a given API key.**

You must pass this endpoint a JSON request body with a `name` field and a `scopes` array containing at least one scope. The `name` and `scopes` fields will be used to update the key associated with the `api_key_id` in the request URL.

If you need to update a key's scopes only, pass the `name` field with the key's existing name; the `name` will not be modified. If you need to update a key's name only, use the "Update API key name" endpoint.

See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes.*/
    pub fn put_api_keys_api_key_id(
        &self,
        api_key_id: &str,
        name: &str,
    ) -> request::PutApiKeysApiKeyIdRequest {
        request::PutApiKeysApiKeyIdRequest {
            client: &self,
            on_behalf_of: None,
            api_key_id: api_key_id.to_owned(),
            name: name.to_owned(),
            scopes: None,
        }
    }
    /**Delete API keys

**This endpoint allows you to revoke an existing API Key using an `api_key_id`**

Authentications using a revoked API Key will fail after after some small propogation delay. If the API Key ID does not exist, a `404` status will be returned.*/
    pub fn delete_api_keys_api_key_id(
        &self,
        api_key_id: &str,
    ) -> request::DeleteApiKeysApiKeyIdRequest {
        request::DeleteApiKeysApiKeyIdRequest {
            client: &self,
            on_behalf_of: None,
            api_key_id: api_key_id.to_owned(),
        }
    }
    /**Update API key name

**This endpoint allows you to update the name of an existing API Key.**

You must pass this endpoint a JSON request body with a `name` property, which will be used to rename the key associated with the `api_key_id` passed in the URL.*/
    pub fn patch_api_keys_api_key_id(
        &self,
        api_key_id: &str,
        name: &str,
    ) -> request::PatchApiKeysApiKeyIdRequest {
        request::PatchApiKeysApiKeyIdRequest {
            client: &self,
            on_behalf_of: None,
            api_key_id: api_key_id.to_owned(),
            name: name.to_owned(),
        }
    }
    /**Retrieve all suppression groups associated with the user.

**This endpoint allows you to retrieve a list of all suppression groups created by this user.**

This endpoint can also return information for multiple group IDs that you include in your request. To add a group ID to your request, simply append `?id=123456&id=123456`, with the appropriate group IDs.*/
    pub fn get_asm_groups(&self) -> request::GetAsmGroupsRequest {
        request::GetAsmGroupsRequest {
            client: &self,
            id: None,
            on_behalf_of: None,
        }
    }
    /**Create a new suppression group

**This endpoint allows you to create a new suppression group.**

To add an email address to the suppression group, [create a Suppression](https://sendgrid.api-docs.io/v3.0/suppressions-suppressions/add-suppressions-to-a-suppression-group).*/
    pub fn post_asm_groups(&self) -> request::PostAsmGroupsRequest {
        request::PostAsmGroupsRequest {
            client: &self,
            on_behalf_of: None,
            description: None,
            is_default: None,
            name: None,
        }
    }
    /**Get information on a single suppression group.

**This endpoint allows you to retrieve a single suppression group.***/
    pub fn get_asm_groups_group_id(
        &self,
        group_id: &str,
    ) -> request::GetAsmGroupsGroupIdRequest {
        request::GetAsmGroupsGroupIdRequest {
            client: &self,
            on_behalf_of: None,
            group_id: group_id.to_owned(),
        }
    }
    /**Delete a Suppression Group

**This endpoint allows you to delete a suppression group.**

If a recipient uses the "one-click unsubscribe" option on an email associated with a deleted group, that recipient will be added to the global suppression list.

Deleting a suppression group will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.*/
    pub fn delete_asm_groups_group_id(
        &self,
        group_id: &str,
    ) -> request::DeleteAsmGroupsGroupIdRequest {
        request::DeleteAsmGroupsGroupIdRequest {
            client: &self,
            on_behalf_of: None,
            group_id: group_id.to_owned(),
        }
    }
    /**Update a suppression group.

**This endpoint allows you to update or change a suppression group.***/
    pub fn patch_asm_groups_group_id(
        &self,
        args: request::PatchAsmGroupsGroupIdRequired,
    ) -> request::PatchAsmGroupsGroupIdRequest {
        request::PatchAsmGroupsGroupIdRequest {
            client: &self,
            on_behalf_of: None,
            group_id: args.group_id.to_owned(),
            description: args.description.to_owned(),
            is_default: args.is_default,
            name: args.name.to_owned(),
        }
    }
    /**Retrieve all suppressions for a suppression group

**This endpoint allows you to retrieve all suppressed email addresses belonging to the given group.***/
    pub fn get_asm_groups_group_id_suppressions(
        &self,
        group_id: &str,
    ) -> request::GetAsmGroupsGroupIdSuppressionsRequest {
        request::GetAsmGroupsGroupIdSuppressionsRequest {
            client: &self,
            on_behalf_of: None,
            group_id: group_id.to_owned(),
        }
    }
    /**Add suppressions to a suppression group

**This endpoint allows you to add email addresses to an unsubscribe group.**

If you attempt to add suppressions to a group that has been deleted or does not exist, the suppressions will be added to the global suppressions list.*/
    pub fn post_asm_groups_group_id_suppressions(
        &self,
        group_id: &str,
        recipient_emails: &[&str],
    ) -> request::PostAsmGroupsGroupIdSuppressionsRequest {
        request::PostAsmGroupsGroupIdSuppressionsRequest {
            client: &self,
            on_behalf_of: None,
            group_id: group_id.to_owned(),
            recipient_emails: recipient_emails.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Search for suppressions within a group

**This endpoint allows you to search a suppression group for multiple suppressions.**

When given a list of email addresses and a group ID, this endpoint will only return the email addresses that have been unsubscribed from the given group.*/
    pub fn post_asm_groups_group_id_suppressions_search(
        &self,
        group_id: &str,
        recipient_emails: &[&str],
    ) -> request::PostAsmGroupsGroupIdSuppressionsSearchRequest {
        request::PostAsmGroupsGroupIdSuppressionsSearchRequest {
            client: &self,
            on_behalf_of: None,
            group_id: group_id.to_owned(),
            recipient_emails: recipient_emails.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Delete a suppression from a suppression group

**This endpoint allows you to remove a suppressed email address from the given suppression group.**

Removing an address will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.*/
    pub fn delete_asm_groups_group_id_suppressions_email(
        &self,
        group_id: &str,
        email: &str,
    ) -> request::DeleteAsmGroupsGroupIdSuppressionsEmailRequest {
        request::DeleteAsmGroupsGroupIdSuppressionsEmailRequest {
            client: &self,
            on_behalf_of: None,
            group_id: group_id.to_owned(),
            email: email.to_owned(),
        }
    }
    /**Retrieve all suppressions

**This endpoint allows you to retrieve a list of all suppressions.***/
    pub fn get_asm_suppressions(&self) -> request::GetAsmSuppressionsRequest {
        request::GetAsmSuppressionsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Add recipient addresses to the global suppression group.

**This endpoint allows you to add one or more email addresses to the global suppressions group.***/
    pub fn post_asm_suppressions_global(
        &self,
        recipient_emails: &[&str],
    ) -> request::PostAsmSuppressionsGlobalRequest {
        request::PostAsmSuppressionsGlobalRequest {
            client: &self,
            on_behalf_of: None,
            recipient_emails: recipient_emails.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Retrieve a Global Suppression

**This endpoint allows you to retrieve a global suppression. You can also use this endpoint to confirm if an email address is already globally suppresed.**

If the email address you include in the URL path parameter `{email}` is already globally suppressed, the response will include that email address. If the address you enter for `{email}` is not globally suppressed, an empty JSON object `{}` will be returned.*/
    pub fn get_asm_suppressions_global_email(
        &self,
        email: &str,
    ) -> request::GetAsmSuppressionsGlobalEmailRequest {
        request::GetAsmSuppressionsGlobalEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Delete a Global Suppression

**This endpoint allows you to remove an email address from the global suppressions group.**

Deleting a suppression group will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.*/
    pub fn delete_asm_suppressions_global_email(
        &self,
        email: &str,
    ) -> request::DeleteAsmSuppressionsGlobalEmailRequest {
        request::DeleteAsmSuppressionsGlobalEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Retrieve all suppression groups for an email address

**This endpoint returns a list of all groups from which the given email address has been unsubscribed.***/
    pub fn get_asm_suppressions_email(
        &self,
        email: &str,
    ) -> request::GetAsmSuppressionsEmailRequest {
        request::GetAsmSuppressionsEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Retrieve email statistics by browser.

**This endpoint allows you to retrieve your email statistics segmented by browser type.**

**We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.

Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).*/
    pub fn get_browsers_stats(
        &self,
        start_date: &str,
    ) -> request::GetBrowsersStatsRequest {
        request::GetBrowsersStatsRequest {
            client: &self,
            browsers: None,
            on_behalf_of: None,
            limit: None,
            offset: None,
            aggregated_by: None,
            start_date: start_date.to_owned(),
            end_date: None,
        }
    }
    /**Retrieve all Campaigns

**This endpoint allows you to retrieve a list of all of your campaigns.**

Returns campaigns in reverse order they were created (newest first).

Returns an empty array if no campaigns exist.*/
    pub fn get_campaigns(&self) -> request::GetCampaignsRequest {
        request::GetCampaignsRequest {
            client: &self,
            limit: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Create a Campaign

**This endpoint allows you to create a campaign.**

In order to send or schedule the campaign, you will be required to provide a subject, sender ID, content (we suggest both html and plain text), and at least one list or segment ID. This information is not required when you create a campaign.*/
    pub fn post_campaigns(&self, title: &str) -> request::PostCampaignsRequest {
        request::PostCampaignsRequest {
            client: &self,
            on_behalf_of: None,
            categories: None,
            custom_unsubscribe_url: None,
            editor: None,
            html_content: None,
            ip_pool: None,
            list_ids: None,
            plain_content: None,
            segment_ids: None,
            sender_id: None,
            subject: None,
            suppression_group_id: None,
            title: title.to_owned(),
        }
    }
    /**Retrieve a single campaign

**This endpoint allows you to retrieve a specific campaign.***/
    pub fn get_campaigns_campaign_id(
        &self,
        campaign_id: i64,
    ) -> request::GetCampaignsCampaignIdRequest {
        request::GetCampaignsCampaignIdRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id,
        }
    }
    /**Delete a Campaign

**This endpoint allows you to delete a specific campaign.***/
    pub fn delete_campaigns_campaign_id(
        &self,
        campaign_id: i64,
    ) -> request::DeleteCampaignsCampaignIdRequest {
        request::DeleteCampaignsCampaignIdRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id,
        }
    }
    /**Update a Campaign

**This endpoint allows you to update a specific campaign.**

This is especially useful if you only set up the campaign using POST /campaigns, but didn't set many of the parameters.*/
    pub fn patch_campaigns_campaign_id(
        &self,
        args: request::PatchCampaignsCampaignIdRequired,
    ) -> request::PatchCampaignsCampaignIdRequest {
        request::PatchCampaignsCampaignIdRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id: args.campaign_id,
            categories: args.categories.iter().map(|&x| x.to_owned()).collect(),
            html_content: args.html_content.to_owned(),
            plain_content: args.plain_content.to_owned(),
            subject: args.subject.to_owned(),
            title: args.title.to_owned(),
        }
    }
    /**View Scheduled Time of a Campaign

**This endpoint allows you to retrieve the date and time that a campaign has been scheduled to be sent.***/
    pub fn get_campaigns_campaign_id_schedules(
        &self,
        campaign_id: i64,
    ) -> request::GetCampaignsCampaignIdSchedulesRequest {
        request::GetCampaignsCampaignIdSchedulesRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id,
        }
    }
    /**Schedule a Campaign

**This endpoint allows you to schedule a specific date and time for your campaign to be sent.**

If you have the flexibility, it's better to schedule mail for off-peak times. Most emails are scheduled and sent at the top of the hour or half hour. Scheduling email to avoid those times (for example, scheduling at 10:53) can result in lower deferral rates because it won't be going through our servers at the same times as everyone else's mail.*/
    pub fn post_campaigns_campaign_id_schedules(
        &self,
        campaign_id: i64,
        send_at: i64,
    ) -> request::PostCampaignsCampaignIdSchedulesRequest {
        request::PostCampaignsCampaignIdSchedulesRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id,
            send_at,
        }
    }
    /**Unschedule a Scheduled Campaign

**This endpoint allows you to unschedule a campaign that has already been scheduled to be sent.**

A successful unschedule will return a 204.
If the specified campaign is in the process of being sent, the only option is to cancel (a different method).*/
    pub fn delete_campaigns_campaign_id_schedules(
        &self,
        campaign_id: i64,
    ) -> request::DeleteCampaignsCampaignIdSchedulesRequest {
        request::DeleteCampaignsCampaignIdSchedulesRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id,
        }
    }
    /**Update a Scheduled Campaign

**This endpoint allows to you change the scheduled time and date for a campaign to be sent.***/
    pub fn patch_campaigns_campaign_id_schedules(
        &self,
        campaign_id: i64,
        send_at: i64,
    ) -> request::PatchCampaignsCampaignIdSchedulesRequest {
        request::PatchCampaignsCampaignIdSchedulesRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id,
            send_at,
        }
    }
    /**Send a Campaign

**This endpoint allows you to immediately send an existing campaign.**

Normally a POST request would have a body, but since this endpoint is telling us to send a resource that is already created, a request body is not needed.*/
    pub fn post_campaigns_campaign_id_schedules_now(
        &self,
        campaign_id: i64,
    ) -> request::PostCampaignsCampaignIdSchedulesNowRequest {
        request::PostCampaignsCampaignIdSchedulesNowRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id,
        }
    }
    /**Send a Test Campaign

**This endpoint allows you to send a test campaign.**

To send to multiple addresses, use an array for the JSON "to" value ["one@address","two@address"]*/
    pub fn post_campaigns_campaign_id_schedules_test(
        &self,
        campaign_id: i64,
        to: &str,
    ) -> request::PostCampaignsCampaignIdSchedulesTestRequest {
        request::PostCampaignsCampaignIdSchedulesTestRequest {
            client: &self,
            on_behalf_of: None,
            campaign_id,
            to: to.to_owned(),
        }
    }
    /**Retrieve all categories

**This endpoint allows you to retrieve a list of all of your categories.***/
    pub fn get_categories(&self) -> request::GetCategoriesRequest {
        request::GetCategoriesRequest {
            client: &self,
            limit: None,
            category: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve Email Statistics for Categories

**This endpoint allows you to retrieve all of your email statistics for each of your categories.**

If you do not define any query parameters, this endpoint will return a sum for each category in groups of 10.*/
    pub fn get_categories_stats(
        &self,
        start_date: &str,
        categories: &str,
    ) -> request::GetCategoriesStatsRequest {
        request::GetCategoriesStatsRequest {
            client: &self,
            start_date: start_date.to_owned(),
            end_date: None,
            categories: categories.to_owned(),
            aggregated_by: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve sums of email stats for each category.

**This endpoint allows you to retrieve the total sum of each email statistic for every category over the given date range.**

If you do not define any query parameters, this endpoint will return a sum for each category in groups of 10.*/
    pub fn get_categories_stats_sums(
        &self,
        start_date: &str,
    ) -> request::GetCategoriesStatsSumsRequest {
        request::GetCategoriesStatsSumsRequest {
            client: &self,
            sort_by_metric: None,
            sort_by_direction: None,
            start_date: start_date.to_owned(),
            end_date: None,
            limit: None,
            offset: None,
            aggregated_by: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve email statistics by client type.

**This endpoint allows you to retrieve your email statistics segmented by client type.**

**We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.

Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).*/
    pub fn get_clients_stats(
        &self,
        start_date: &str,
    ) -> request::GetClientsStatsRequest {
        request::GetClientsStatsRequest {
            client: &self,
            on_behalf_of: None,
            start_date: start_date.to_owned(),
            end_date: None,
            aggregated_by: None,
        }
    }
    /**Retrieve stats by a specific client type.

**This endpoint allows you to retrieve your email statistics segmented by a specific client type.**

**We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.

## Available Client Types
- phone
- tablet
- webmail
- desktop

Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).*/
    pub fn get_clients_client_type_stats(
        &self,
        start_date: &str,
        client_type: &str,
    ) -> request::GetClientsClientTypeStatsRequest {
        request::GetClientsClientTypeStatsRequest {
            client: &self,
            on_behalf_of: None,
            start_date: start_date.to_owned(),
            end_date: None,
            aggregated_by: None,
            client_type: client_type.to_owned(),
        }
    }
    /**Retrieve all custom fields

**This endpoint allows you to retrieve all custom fields.***/
    pub fn get_contactdb_custom_fields(
        &self,
    ) -> request::GetContactdbCustomFieldsRequest {
        request::GetContactdbCustomFieldsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Create a Custom Field

**This endpoint allows you to create a custom field.**

**You can create up to 120 custom fields.***/
    pub fn post_contactdb_custom_fields(
        &self,
    ) -> request::PostContactdbCustomFieldsRequest {
        request::PostContactdbCustomFieldsRequest {
            client: &self,
            on_behalf_of: None,
            name: None,
            type_: None,
        }
    }
    /**Retrieve a Custom Field

**This endpoint allows you to retrieve a custom field by ID.***/
    pub fn get_contactdb_custom_fields_custom_field_id(
        &self,
        custom_field_id: i64,
    ) -> request::GetContactdbCustomFieldsCustomFieldIdRequest {
        request::GetContactdbCustomFieldsCustomFieldIdRequest {
            client: &self,
            on_behalf_of: None,
            custom_field_id,
        }
    }
    /**Delete a Custom Field

**This endpoint allows you to delete a custom field by ID.***/
    pub fn delete_contactdb_custom_fields_custom_field_id(
        &self,
        custom_field_id: i64,
    ) -> request::DeleteContactdbCustomFieldsCustomFieldIdRequest {
        request::DeleteContactdbCustomFieldsCustomFieldIdRequest {
            client: &self,
            on_behalf_of: None,
            custom_field_id,
        }
    }
    /**Retrieve all lists

**This endpoint allows you to retrieve all of your recipient lists. If you don't have any lists, an empty array will be returned.***/
    pub fn get_contactdb_lists(&self) -> request::GetContactdbListsRequest {
        request::GetContactdbListsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Create a List

**This endpoint allows you to create a list for your recipients.***/
    pub fn post_contactdb_lists(
        &self,
        name: &str,
    ) -> request::PostContactdbListsRequest {
        request::PostContactdbListsRequest {
            client: &self,
            on_behalf_of: None,
            name: name.to_owned(),
        }
    }
    /**Delete Multiple lists

**This endpoint allows you to delete multiple recipient lists.***/
    pub fn delete_contactdb_lists(
        &self,
        body: serde_json::Value,
    ) -> request::DeleteContactdbListsRequest {
        request::DeleteContactdbListsRequest {
            client: &self,
            on_behalf_of: None,
            body,
        }
    }
    /**Retrieve a single list

**This endpoint allows you to retrieve a single recipient list.***/
    pub fn get_contactdb_lists_list_id(
        &self,
    ) -> request::GetContactdbListsListIdRequest {
        request::GetContactdbListsListIdRequest {
            client: &self,
            list_id: None,
            on_behalf_of: None,
        }
    }
    /**Delete a List

**This endpoint allows you to delete a specific recipient list with the given ID.***/
    pub fn delete_contactdb_lists_list_id(
        &self,
        list_id: &str,
    ) -> request::DeleteContactdbListsListIdRequest {
        request::DeleteContactdbListsListIdRequest {
            client: &self,
            delete_contacts: None,
            on_behalf_of: None,
            list_id: list_id.to_owned(),
        }
    }
    /**Update a List

**This endpoint allows you to update the name of one of your recipient lists.***/
    pub fn patch_contactdb_lists_list_id(
        &self,
        list_id: i64,
        name: &str,
    ) -> request::PatchContactdbListsListIdRequest {
        request::PatchContactdbListsListIdRequest {
            client: &self,
            list_id,
            on_behalf_of: None,
            name: name.to_owned(),
        }
    }
    /**Retrieve all recipients on a List

**This endpoint allows you to retrieve all recipients on the list with the given ID.***/
    pub fn get_contactdb_lists_list_id_recipients(
        &self,
        list_id: i64,
    ) -> request::GetContactdbListsListIdRecipientsRequest {
        request::GetContactdbListsListIdRecipientsRequest {
            client: &self,
            page: None,
            page_size: None,
            list_id,
            on_behalf_of: None,
        }
    }
    /**Add Multiple Recipients to a List

**This endpoint allows you to add multiple recipients to a list.**

Adds existing recipients to a list, passing in the recipient IDs to add. Recipient IDs should be passed exactly as they are returned from recipient endpoints.*/
    pub fn post_contactdb_lists_list_id_recipients(
        &self,
        list_id: i64,
        body: serde_json::Value,
    ) -> request::PostContactdbListsListIdRecipientsRequest {
        request::PostContactdbListsListIdRecipientsRequest {
            client: &self,
            on_behalf_of: None,
            list_id,
            body,
        }
    }
    /**Add a Single Recipient to a List

**This endpoint allows you to add a single recipient to a list.***/
    pub fn post_contactdb_lists_list_id_recipients_recipient_id(
        &self,
        list_id: i64,
        recipient_id: &str,
    ) -> request::PostContactdbListsListIdRecipientsRecipientIdRequest {
        request::PostContactdbListsListIdRecipientsRecipientIdRequest {
            client: &self,
            on_behalf_of: None,
            list_id,
            recipient_id: recipient_id.to_owned(),
        }
    }
    /**Delete a Single Recipient from a Single List

**This endpoint allows you to delete a single recipient from a list.***/
    pub fn delete_contactdb_lists_list_id_recipients_recipient_id(
        &self,
        list_id: i64,
        recipient_id: i64,
    ) -> request::DeleteContactdbListsListIdRecipientsRecipientIdRequest {
        request::DeleteContactdbListsListIdRecipientsRecipientIdRequest {
            client: &self,
            list_id,
            recipient_id,
            on_behalf_of: None,
        }
    }
    /**Retrieve recipients

**This endpoint allows you to retrieve all of your Marketing Campaigns recipients.**

Batch deletion of a page makes it possible to receive an empty page of recipients before reaching the end of
the list of recipients. To avoid this issue; iterate over pages until a 404 is retrieved.*/
    pub fn get_contactdb_recipients(&self) -> request::GetContactdbRecipientsRequest {
        request::GetContactdbRecipientsRequest {
            client: &self,
            page: None,
            page_size: None,
            on_behalf_of: None,
        }
    }
    /**Add recipients

**This endpoint allows you to add a Marketing Campaigns recipient.**

You can add custom field data as a parameter on this endpoint. We have provided an example using some of the default custom fields SendGrid provides.

The rate limit is three requests every 2 seconds. You can upload 1000  contacts per request. So the maximum upload rate is 1500 recipients per second.*/
    pub fn post_contactdb_recipients(
        &self,
        body: serde_json::Value,
    ) -> request::PostContactdbRecipientsRequest {
        request::PostContactdbRecipientsRequest {
            client: &self,
            on_behalf_of: None,
            body,
        }
    }
    /**Delete Recipients

**This endpoint allows you to deletes one or more recipients.**

The body of an API call to this endpoint must include an array of recipient IDs of the recipients you want to delete.*/
    pub fn delete_contactdb_recipients(
        &self,
        body: serde_json::Value,
    ) -> request::DeleteContactdbRecipientsRequest {
        request::DeleteContactdbRecipientsRequest {
            client: &self,
            on_behalf_of: None,
            body,
        }
    }
    /**Update Recipient

**This endpoint allows you to update one or more recipients.**

The body of an API call to this endpoint must include an array of one or more recipient objects.

It is of note that you can add custom field data as parameters on recipient objects. We have provided an example using some of the default custom fields SendGrid provides.*/
    pub fn patch_contactdb_recipients(
        &self,
        body: serde_json::Value,
    ) -> request::PatchContactdbRecipientsRequest {
        request::PatchContactdbRecipientsRequest {
            client: &self,
            on_behalf_of: None,
            body,
        }
    }
    /**Retrieve the count of billable recipients

**This endpoint allows you to retrieve the number of Marketing Campaigns recipients that you will be billed for.**

You are billed for marketing campaigns based on the highest number of recipients you have had in your account at one time. This endpoint will allow you to know the current billable count value.*/
    pub fn get_contactdb_recipients_billable_count(
        &self,
    ) -> request::GetContactdbRecipientsBillableCountRequest {
        request::GetContactdbRecipientsBillableCountRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Retrieve a Count of Recipients

**This endpoint allows you to retrieve the total number of Marketing Campaigns recipients.***/
    pub fn get_contactdb_recipients_count(
        &self,
    ) -> request::GetContactdbRecipientsCountRequest {
        request::GetContactdbRecipientsCountRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Search recipients

<p>
  Search using segment conditions without actually creating a segment.
  Body contains a JSON object with <code>conditions</code>, a list of conditions as described below, and an optional <code>list_id</code>, which is a valid list ID for a list to limit the search on.
</p>

<p>
  Valid operators for create and update depend on the type of the field for which you are searching.
</p>

<ul>
  <li>Dates:
    <ul>
      <li>"eq", "ne", "lt" (before), "gt" (after)
        <ul>
          <li>You may use MM/DD/YYYY for day granularity or an epoch for second granularity.</li>
        </ul>
      </li>
      <li>"empty", "not_empty"</li>
      <li>"is within"
        <ul>
          <li>You may use an <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a> or the # of days.</li>
        </ul>
      </li>
    </ul>
  </li>
  <li>Text: "contains", "eq" (is - matches the full field), "ne" (is not - matches any field where the entire field is not the condition value), "empty", "not_empty"</li>
  <li>Numbers: "eq", "lt", "gt", "empty", "not_empty"</li>
  <li>Email Clicks and Opens: "eq" (opened), "ne" (not opened)</li>
</ul>

<p>
  Field values must all be a string.
</p>

<p>
  Search conditions using "eq" or "ne" for email clicks and opens should provide a "field" of either <code>clicks.campaign_identifier</code> or <code>opens.campaign_identifier</code>.
  The condition value should be a string containing the id of a completed campaign.
</p>

<p>
  Search conditions list may contain multiple conditions, joined by an "and" or "or" in the "and_or" field.
  The first condition in the conditions list must have an empty "and_or", and subsequent conditions must all specify an "and_or".
</p>*/
    pub fn post_contactdb_recipients_search(
        &self,
        conditions: Vec<ContactdbSegmentsConditions>,
        list_id: i64,
    ) -> request::PostContactdbRecipientsSearchRequest {
        request::PostContactdbRecipientsSearchRequest {
            client: &self,
            conditions,
            list_id,
        }
    }
    /**Retrieve a single recipient

**This endpoint allows you to retrieve a single recipient by ID from your contact database.***/
    pub fn get_contactdb_recipients_recipient_id(
        &self,
        recipient_id: &str,
    ) -> request::GetContactdbRecipientsRecipientIdRequest {
        request::GetContactdbRecipientsRecipientIdRequest {
            client: &self,
            on_behalf_of: None,
            recipient_id: recipient_id.to_owned(),
        }
    }
    /**Delete a Recipient

**This endpoint allows you to delete a single recipient with the given ID from your contact database.**

> Use this to permanently delete your recipients from all of your contact lists and all segments if required by applicable law.*/
    pub fn delete_contactdb_recipients_recipient_id(
        &self,
        recipient_id: &str,
    ) -> request::DeleteContactdbRecipientsRecipientIdRequest {
        request::DeleteContactdbRecipientsRecipientIdRequest {
            client: &self,
            on_behalf_of: None,
            recipient_id: recipient_id.to_owned(),
        }
    }
    /**Retrieve the lists that a recipient is on

**This endpoint allows you to retrieve the lists that a given recipient belongs to.**

Each recipient can be on many lists. This endpoint gives you all of the lists that any one recipient has been added to.*/
    pub fn get_contactdb_recipients_recipient_id_lists(
        &self,
        recipient_id: &str,
    ) -> request::GetContactdbRecipientsRecipientIdListsRequest {
        request::GetContactdbRecipientsRecipientIdListsRequest {
            client: &self,
            on_behalf_of: None,
            recipient_id: recipient_id.to_owned(),
        }
    }
    /**Retrieve reserved fields

**This endpoint allows you to list all fields that are reserved and can't be used for custom field names.***/
    pub fn get_contactdb_reserved_fields(
        &self,
    ) -> request::GetContactdbReservedFieldsRequest {
        request::GetContactdbReservedFieldsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Retrieve all segments

**This endpoint allows you to retrieve all of your segments.***/
    pub fn get_contactdb_segments(&self) -> request::GetContactdbSegmentsRequest {
        request::GetContactdbSegmentsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Create a Segment

**This endpoint allows you to create a new segment.**


  Valid operators for create and update depend on the type of the field for which you are searching.

**Dates**
- "eq", "ne", "lt" (before), "gt" (after)
    - You may use MM/DD/YYYY for day granularity or an epoch for second granularity.
- "empty", "not_empty"
- "is within"
    - You may use an [ISO 8601 date format](https://en.wikipedia.org/wiki/ISO_8601) or the # of days.

**Text**
- "contains"
- "eq" (is/equals - matches the full field)
- "ne" (is not/not equals - matches any field where the entire field is not the condition value)
- "empty"
- "not_empty"

**Numbers**
- "eq" (is/equals)
- "lt" (is less than)
- "gt" (is greater than)
- "empty"
- "not_empty"

**Email Clicks and Opens**
- "eq" (opened)
- "ne" (not opened)

All field values must be a string.


Conditions using "eq" or "ne" for email clicks and opens should provide a "field" of either `clicks.campaign_identifier` or `opens.campaign_identifier`.
The condition value should be a string containing the id of a completed campaign.


The conditions list may contain multiple conditions, joined by an "and" or "or" in the "and_or" field.

The first condition in the conditions list must have an empty "and_or", and subsequent conditions must all specify an "and_or".*/
    pub fn post_contactdb_segments(
        &self,
        conditions: Vec<ContactdbSegmentsConditions>,
        name: &str,
    ) -> request::PostContactdbSegmentsRequest {
        request::PostContactdbSegmentsRequest {
            client: &self,
            on_behalf_of: None,
            conditions,
            list_id: None,
            name: name.to_owned(),
            recipient_count: None,
        }
    }
    /**Retrieve a segment

**This endpoint allows you to retrieve a single segment with the given ID.***/
    pub fn get_contactdb_segments_segment_id(
        &self,
        segment_id: i64,
    ) -> request::GetContactdbSegmentsSegmentIdRequest {
        request::GetContactdbSegmentsSegmentIdRequest {
            client: &self,
            segment_id,
            on_behalf_of: None,
        }
    }
    /**Delete a segment

**This endpoint allows you to delete a segment from your recipients database.**

You also have the option to delete all the contacts from your Marketing Campaigns recipient database who were in this segment.*/
    pub fn delete_contactdb_segments_segment_id(
        &self,
        segment_id: &str,
    ) -> request::DeleteContactdbSegmentsSegmentIdRequest {
        request::DeleteContactdbSegmentsSegmentIdRequest {
            client: &self,
            delete_contacts: None,
            on_behalf_of: None,
            segment_id: segment_id.to_owned(),
        }
    }
    /**Update a segment

**This endpoint allows you to update a segment.***/
    pub fn patch_contactdb_segments_segment_id(
        &self,
        name: &str,
    ) -> request::PatchContactdbSegmentsSegmentIdRequest {
        request::PatchContactdbSegmentsSegmentIdRequest {
            client: &self,
            segment_id: None,
            on_behalf_of: None,
            conditions: None,
            list_id: None,
            name: name.to_owned(),
        }
    }
    /**Retrieve recipients on a segment

**This endpoint allows you to retrieve all of the recipients in a segment with the given ID.***/
    pub fn get_contactdb_segments_segment_id_recipients(
        &self,
        segment_id: i64,
    ) -> request::GetContactdbSegmentsSegmentIdRecipientsRequest {
        request::GetContactdbSegmentsSegmentIdRecipientsRequest {
            client: &self,
            page: None,
            page_size: None,
            on_behalf_of: None,
            segment_id,
        }
    }
    /**Get Recipient Upload Status

**This endpoint allows you to check the upload status of a Marketing Campaigns recipient.***/
    pub fn get_contactdb_status(&self) -> request::GetContactdbStatusRequest {
        request::GetContactdbStatusRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**List Designs

**This endpoint allows you to retrieve a list of designs already stored in your Design Library**.

A GET request to `/designs` will return a list of your existing designs. This endpoint will not return the pre-built Twilio SendGrid designs. Pre-built designs can be retrieved using the `/designs/pre-builts` endpoint, which is detailed below.

By default, you will receive 100 results per request; however, you can modify the number of results returned by passing an integer to the `page_size` query parameter.*/
    pub fn list_designs(&self) -> request::ListDesignsRequest {
        request::ListDesignsRequest {
            client: &self,
            page_size: None,
            page_token: None,
            summary: None,
        }
    }
    /**Create Design

**This endpoint allows you to create a new design**.

You can add a new design by passing data, including a string of HTML email content, to `/designs`. When creating designs from scratch, be aware of the styling constraints inherent to many email clients. For a list of best practices, see our guide to [Cross-Platform Email Design](https://sendgrid.com/docs/ui/sending-email/cross-platform-html-design/).

The Design Library can also convert your design’s HTML elements into drag and drop modules that are editable in the Designs Library user interface. For more, visit the [Design and Code Editor documentation](https://sendgrid.com/docs/ui/sending-email/editor/#drag--drop-markup).

Because the `/designs` endpoint makes it easy to add designs, you can create a design with your preferred tooling or migrate designs you already own without relying on the Design Library UI.*/
    pub fn post_design(
        &self,
        args: request::PostDesignRequired,
    ) -> request::PostDesignRequest {
        request::PostDesignRequest {
            client: &self,
            editor: args.editor.to_owned(),
            name: args.name.to_owned(),
            categories: args.categories.iter().map(|&x| x.to_owned()).collect(),
            generate_plain_content: args.generate_plain_content,
            subject: args.subject.to_owned(),
            html_content: args.html_content.to_owned(),
            plain_content: args.plain_content.to_owned(),
        }
    }
    /**List SendGrid Pre-built Designs

**This endpoint allows you to retrieve a list of pre-built designs provided by Twilio SendGrid**.

Unlike the `/designs` endpoint where *your* designs are stored, a GET request made to `designs/pre-builts` will retrieve a list of the pre-built Twilio SendGrid designs. This endpoint will not return the designs stored in your Design Library.

By default, you will receive 100 results per request; however, you can modify the number of results returned by passing an integer to the `page_size` query parameter.

This endpoint is useful for retrieving the IDs of Twilio SendGrid designs that you want to duplicate and modify.*/
    pub fn list_sendgrid_pre_built_designs(
        &self,
    ) -> request::ListSendgridPreBuiltDesignsRequest {
        request::ListSendgridPreBuiltDesignsRequest {
            client: &self,
            page_size: None,
            page_token: None,
            summary: None,
        }
    }
    /**Get SendGrid Pre-built Design

**This endpoint allows you to retrieve a single pre-built design**.

A GET request to `/designs/pre-builts/{id}` will retrieve details about a specific pre-built design.

This endpoint is valuable when retrieving details about a pre-built design that you wish to duplicate and modify.*/
    pub fn get_sendgrid_pre_built_design(
        &self,
        id: &str,
    ) -> request::GetSendgridPreBuiltDesignRequest {
        request::GetSendgridPreBuiltDesignRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Duplicate SendGrid Pre-built Design

**This endpoint allows you to duplicate one of the pre-built Twilio SendGrid designs**.

Like duplicating one of your existing designs, you are not required to pass any data in the body of a request to this endpoint. If you choose to leave the `name` field blank, your duplicate will be assigned the name of the design it was copied from with the text "Duplicate: " prepended to it. This name change is only a convenience, as the duplicate design will be assigned a unique ID that differentiates it from your other designs. You can retrieve the IDs for Twilio SendGrid pre-built designs using the "List SendGrid Pre-built Designs" endpoint.

You can modify your duplicate’s name at the time of creation by passing an updated value to the `name` field when making the initial request.
More on retrieving design IDs can be found above.*/
    pub fn post_sendgrid_pre_built_design(
        &self,
        id: &str,
    ) -> request::PostSendgridPreBuiltDesignRequest {
        request::PostSendgridPreBuiltDesignRequest {
            client: &self,
            id: id.to_owned(),
            editor: None,
            name: None,
        }
    }
    /**Get Design

**This endpoint allows you to retrieve a single design**.

A GET request to `/designs/{id}` will retrieve details about a specific design in your Design Library.

This endpoint is valuable when retrieving information stored in a field that you wish to update using a PATCH request.*/
    pub fn get_design(&self, id: &str) -> request::GetDesignRequest {
        request::GetDesignRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Duplicate Design

**This endpoint allows you to duplicate one of your existing designs**.

Modifying an existing design is often the easiest way to create something new.

You are not required to pass any data in the body of a request to this endpoint. If you choose to leave the `name` field blank, your duplicate will be assigned the name of the design it was copied from with the text "Duplicate: " prepended to it. This name change is only a convenience, as the duplicate will be assigned a unique ID that differentiates it from your other designs.

You can modify your duplicate’s name at the time of creation by passing an updated value to the `name` field when making the initial request.
More on retrieving design IDs can be found below.*/
    pub fn post_design_dup(&self, id: &str) -> request::PostDesignDupRequest {
        request::PostDesignDupRequest {
            client: &self,
            id: id.to_owned(),
            editor: None,
            name: None,
        }
    }
    /**Delete Design

**This endpoint allows you to delete a single design**.

Be sure to check the ID of the design you intend to delete before making this request; deleting a design is a permanent action.*/
    pub fn delete_design(&self, id: &str) -> request::DeleteDesignRequest {
        request::DeleteDesignRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Update Design

**This endpoint allows you to edit a design**.

The Design API supports PATCH requests, which allow you to make partial updates to a single design. Passing data to a specific field will update only the data stored in that field; all other fields will be unaltered.

For example, updating a design's name requires that you make a PATCH request to this endpoint with data specified for the `name` field only.

```
{
    "name": "<Updated Name>"
}
```*/
    pub fn put_design(&self, id: &str) -> request::PutDesignRequest {
        request::PutDesignRequest {
            client: &self,
            id: id.to_owned(),
            categories: None,
            generate_plain_content: None,
            html_content: None,
            name: None,
            plain_content: None,
            subject: None,
        }
    }
    /**Retrieve email statistics by device type.

**This endpoint allows you to retrieve your email statistics segmented by the device type.**

**We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.

## Available Device Types
| **Device** | **Description** | **Example** |
|---|---|---|
| Desktop | Email software on desktop computer. | I.E., Outlook, Sparrow, or Apple Mail. |
| Webmail |	A web-based email client. | I.E., Yahoo, Google, AOL, or Outlook.com. |
| Phone | A smart phone. | iPhone, Android, Blackberry, etc.
| Tablet | A tablet computer. | iPad, android based tablet, etc. |
| Other | An unrecognized device. |

Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).*/
    pub fn get_devices_stats(
        &self,
        start_date: &str,
    ) -> request::GetDevicesStatsRequest {
        request::GetDevicesStatsRequest {
            client: &self,
            on_behalf_of: None,
            limit: None,
            offset: None,
            aggregated_by: None,
            start_date: start_date.to_owned(),
            end_date: None,
        }
    }
    /**Retrieve email statistics by country and state/province.

**This endpoint allows you to retrieve your email statistics segmented by country and state/province.**

**We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.

Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [User Guide](https://sendgrid.com/docs/User_Guide/Statistics/index.html).*/
    pub fn get_geo_stats(&self, start_date: &str) -> request::GetGeoStatsRequest {
        request::GetGeoStatsRequest {
            client: &self,
            country: None,
            on_behalf_of: None,
            limit: None,
            offset: None,
            aggregated_by: None,
            start_date: start_date.to_owned(),
            end_date: None,
        }
    }
    /**Retrieve all IP addresses

**This endpoint allows you to retrieve a list of all assigned and unassigned IPs.**

Response includes warm up status, pools, assigned subusers, and reverse DNS info. The start_date field corresponds to when warmup started for that IP.

A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.*/
    pub fn get_ips(&self) -> request::GetIpsRequest {
        request::GetIpsRequest {
            client: &self,
            ip: None,
            exclude_whitelabels: None,
            limit: None,
            offset: None,
            subuser: None,
            sort_by_direction: None,
        }
    }
    /**Add IPs

**This endpoint is for adding a(n) IP Address(es) to your account.***/
    pub fn post_ips(&self, count: i64) -> request::PostIpsRequest {
        request::PostIpsRequest {
            client: &self,
            count,
            subusers: None,
            warmup: None,
        }
    }
    /**Retrieve all assigned IPs

**This endpoint allows you to retrieve only assigned IP addresses.**

A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.*/
    pub fn get_ips_assigned(&self) -> request::GetIpsAssignedRequest {
        request::GetIpsAssignedRequest {
            client: &self,
        }
    }
    /**Retrieve all IP pools

**This endpoint allows you to get all of your IP pools.***/
    pub fn get_ips_pools(&self) -> request::GetIpsPoolsRequest {
        request::GetIpsPoolsRequest {
            client: &self,
        }
    }
    /**Create an IP pool

**This endpoint allows you to create an IP pool.**

Before you can create an IP pool, you need to activate the IP in your SendGrid account:

1. Log into your SendGrid account.
1. Navigate to **Settings** and then select **IP Addresses**.
1. Find the IP address you want to activate and then click **Edit**.
1. Check **Allow my account to send mail using this IP address**.
1. Click **Save**.*/
    pub fn post_ips_pools(&self, name: &str) -> request::PostIpsPoolsRequest {
        request::PostIpsPoolsRequest {
            client: &self,
            name: name.to_owned(),
        }
    }
    /**Retrieve all the IPs in a specified pool

**This endpoint allows you to get all of the IP addresses that are in a specific IP pool.***/
    pub fn get_ips_pools_pool_name(
        &self,
        pool_name: &str,
    ) -> request::GetIpsPoolsPoolNameRequest {
        request::GetIpsPoolsPoolNameRequest {
            client: &self,
            pool_name: pool_name.to_owned(),
        }
    }
    /**Rename an IP pool

**This endpoint allows you to update the name of an IP pool.***/
    pub fn put_ips_pools_pool_name(
        &self,
        pool_name: &str,
    ) -> request::PutIpsPoolsPoolNameRequest {
        request::PutIpsPoolsPoolNameRequest {
            client: &self,
            pool_name: pool_name.to_owned(),
            name: None,
        }
    }
    /**Delete an IP pool

**This endpoint allows you to delete an IP pool.***/
    pub fn delete_ips_pools_pool_name(
        &self,
        pool_name: &str,
    ) -> request::DeleteIpsPoolsPoolNameRequest {
        request::DeleteIpsPoolsPoolNameRequest {
            client: &self,
            pool_name: pool_name.to_owned(),
        }
    }
    /**Add an IP address to a pool

**This endpoint allows you to add an IP address to an IP pool.**

You can add the same IP address to multiple pools. It may take up to 60 seconds for your IP address to be added to a pool after your request is made.

Before you can add an IP to a pool, you need to activate it in your SendGrid account:

1. Log into your SendGrid account.
1. Navigate to **Settings** and then select **IP Addresses**.
1. Find the IP address you want to activate and then click **Edit**.
1. Check **Allow my account to send mail using this IP address**.
1. Click **Save**.

You can retrieve all of your available IP addresses from the "Retrieve all IP addresses" endpoint.*/
    pub fn post_ips_pools_pool_name_ips(
        &self,
        pool_name: &str,
    ) -> request::PostIpsPoolsPoolNameIpsRequest {
        request::PostIpsPoolsPoolNameIpsRequest {
            client: &self,
            pool_name: pool_name.to_owned(),
            ip: None,
        }
    }
    /**Remove an IP address from a pool

**This endpoint allows you to remove an IP address from an IP pool.***/
    pub fn delete_ips_pools_pool_name_ips_ip(
        &self,
        pool_name: &str,
        ip: &str,
    ) -> request::DeleteIpsPoolsPoolNameIpsIpRequest {
        request::DeleteIpsPoolsPoolNameIpsIpRequest {
            client: &self,
            pool_name: pool_name.to_owned(),
            ip: ip.to_owned(),
        }
    }
    /**Get remaining IPs count

**This endpoint gets amount of IP Addresses that can still be created during a given period and the price of those IPs.***/
    pub fn get_ips_remaining(&self) -> request::GetIpsRemainingRequest {
        request::GetIpsRemainingRequest {
            client: &self,
        }
    }
    /**Retrieve all IPs currently in warmup

**This endpoint allows you to retrieve all of your IP addresses that are currently warming up.***/
    pub fn get_ips_warmup(&self) -> request::GetIpsWarmupRequest {
        request::GetIpsWarmupRequest {
            client: &self,
        }
    }
    /**Start warming up an IP address

**This endpoint allows you to put an IP address into warmup mode.***/
    pub fn post_ips_warmup(&self) -> request::PostIpsWarmupRequest {
        request::PostIpsWarmupRequest {
            client: &self,
            ip: None,
        }
    }
    /**Retrieve the warmup status for a specific IP address

**This endpoint allows you to retrieve the warmup status for a specific IP address.**

You can retrieve all of your warming IPs using the "Retrieve all IPs currently in warmup" endpoint.*/
    pub fn get_ips_warmup_ip_address(
        &self,
        ip_address: &str,
    ) -> request::GetIpsWarmupIpAddressRequest {
        request::GetIpsWarmupIpAddressRequest {
            client: &self,
            ip_address: ip_address.to_owned(),
        }
    }
    /**Stop warming up an IP address

**This endpoint allows you to remove an IP address from warmup mode.**

Your request will return a 204 status code if the specified IP was successfully removed from warmup mode. To retrieve details of the IP’s warmup status *before* removing it from warmup mode, call the  "Retrieve the warmpup status for a specific IP address" endpoint.*/
    pub fn delete_ips_warmup_ip_address(
        &self,
        ip_address: &str,
    ) -> request::DeleteIpsWarmupIpAddressRequest {
        request::DeleteIpsWarmupIpAddressRequest {
            client: &self,
            ip_address: ip_address.to_owned(),
        }
    }
    /**Retrieve all IP pools an IP address belongs to

**This endpoint allows you to see which IP pools a particular IP address has been added to.**

The same IP address can be added to multiple IP pools.

A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.*/
    pub fn get_ips_ip_address(
        &self,
        ip_address: &str,
    ) -> request::GetIpsIpAddressRequest {
        request::GetIpsIpAddressRequest {
            client: &self,
            ip_address: ip_address.to_owned(),
        }
    }
    /**Create a batch ID

**This endpoint allows you to generate a new batch ID.**

Once a `batch_id` is created, you can associate it with a scheduled send using the `/mail/send` endpoint. Passing the `batch_id` as a field in the `/mail/send` request body will assign the ID to the send you are creating.

Once an ID is associated with a scheduled send, the send can be accessed and its send status can be modified using the `batch_id`.*/
    pub fn post_mail_batch(&self) -> request::PostMailBatchRequest {
        request::PostMailBatchRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Validate batch ID

**This endpoint allows you to validate a batch ID.**

When you pass a valid `batch_id` to this endpoint, it will return a `200` status code and the batch ID itself.

If you pass an invalid `batch_id` to the endpoint, you will receive a `400` level status code and an error message.

A `batch_id` does not need to be assigned to a scheduled send to be considered valid. A successful response means only that the `batch_id` has been created, but it does not indicate that it has been associated with a send.*/
    pub fn get_mail_batch_batch_id(
        &self,
        batch_id: &str,
    ) -> request::GetMailBatchBatchIdRequest {
        request::GetMailBatchBatchIdRequest {
            client: &self,
            on_behalf_of: None,
            batch_id: batch_id.to_owned(),
        }
    }
    /**v3 Mail Send

The Mail Send endpoint allows you to send email over SendGrid’s v3 Web API, the most recent version of our API. If you are looking for documentation about the v2 Mail Send endpoint, see our [v2 API Reference](https://sendgrid.com/docs/API_Reference/Web_API/mail.html).

## Helper Libraries

Twilio SendGrid provides libraries to help you quickly and easily integrate with the v3 Web API in 7 different languages:

* [C#](https://github.com/sendgrid/sendgrid-csharp)
* [Go](https://github.com/sendgrid/sendgrid-go)
* [Java](https://github.com/sendgrid/sendgrid-java)
* [Node JS](https://github.com/sendgrid/sendgrid-nodejs)
* [PHP](https://github.com/sendgrid/sendgrid-php)
* [Python](https://github.com/sendgrid/sendgrid-python)
* [Ruby](https://github.com/sendgrid/sendgrid-ruby)

## Dynamic Transactional Templates and Handlebars

In order to send a dynamic template, specify the template ID with the `template_id` parameter.

To specify handlebar substitutions, define your substitutions in the request JSON with this syntax:

```
"dynamic_template_data": {
      "guest": "Jane Doe",
      "partysize": "4",
      "english": true,
      "date": "April 1st, 2021"
    }
```

For more information about Dynamic Transactional Templates and Handlebars, see our documentation and reference pages.

* [How to send an email with Dynamic Transactional Templates
](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/)
* [Using Handlebars](https://sendgrid.com/docs/for-developers/sending-email/using-handlebars/)

## Mail Body Compression

Mail body compression is available to some high volume accounts. Talk to your CSM if you are interested in this functionality. Mail body compression works by setting up a JSON payload as defined on this page, then compressing it with gzip (the gzip file can be no more than 30mb).

To use mail body compression:

1. Add a `Content-Encoding` header, with a value of `gzip`.
   a. `Content-Encoding: gzip`
2. Send the gzip as a data-binary.
   a. `--data-binary '@data.json.gz'
`

## Multiple Reply-To Emails

Using `reply_to_list` allows senders to include more than one recipient email address to receive reply and/or bounce messages from the recipient of the email.

### Usage Considerations

* `reply_to` is mutually exclusive with `reply_to_list`. If both are used, then the API call will be rejected.
* The `reply_to_list` object, when used, must at least have an email parameter and may also contain a name parameter.
* Each email address in the `reply_to_list` should be unique.
* There is a limit of 1000 `reply_to_list` emails per mail/send request.
* In SMTP calls, we will omit any invalid emails.

### Possible 400 Error Messages

* `reply_to` is mutually exclusive with `reply_to_list`.
* The `reply_to_list` object, when used, must at least have an email parameter and may also contain a name parameter.
* Each email address in the `reply_to_list` should be unique.
* There is a limit of X `reply_to` emails per mail/send request.
* The `reply_to_list` email does not contain a valid address.
* The `reply_to_list` email exceeds the maximum total length of X characters.
* The `reply_to_list` email parameter is required.*/
    pub fn post_mail_send(
        &self,
        args: request::PostMailSendRequired,
    ) -> request::PostMailSendRequest {
        request::PostMailSendRequest {
            client: &self,
            asm: None,
            attachments: None,
            batch_id: None,
            categories: None,
            content: args.content,
            custom_args: None,
            from: args.from,
            headers: None,
            ip_pool_name: None,
            mail_settings: None,
            personalizations: args.personalizations,
            reply_to: None,
            reply_to_list: None,
            send_at: None,
            subject: args.subject.to_owned(),
            template_id: None,
            tracking_settings: None,
        }
    }
    /**Retrieve all mail settings

**This endpoint allows you to retrieve a list of all mail settings.**

Each setting will be returned with an `enabled` status set to `true` or `false` and a short description that explains what the setting does.*/
    pub fn get_mail_settings(&self) -> request::GetMailSettingsRequest {
        request::GetMailSettingsRequest {
            client: &self,
            limit: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve address whitelist mail settings

**This endpoint allows you to retrieve your current email address whitelist settings.**

The Address Whitelist setting allows you to specify email addresses or domains for which mail should never be suppressed.

For example, if you own the domain `example.com`, and one or more of your recipients use `email@example.com` addresses, placing `example.com` in the address whitelist setting instructs Twilio SendGrid to ignore all bounces, blocks, and unsubscribes logged for that domain. In other words, all bounces, blocks, and unsubscribes will still be sent to `example.com` as if they were sent under normal sending conditions.*/
    pub fn get_mail_settings_address_whitelist(
        &self,
    ) -> request::GetMailSettingsAddressWhitelistRequest {
        request::GetMailSettingsAddressWhitelistRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update address whitelist mail settings

**This endpoint allows you to update your current email address whitelist settings.**

You can select whether or not this setting should be enabled by assigning the `enabled` field a `true` or `false` value.

Passing only the `enabled` field to this endpoint will not alter your current `list` of whitelist entries. However, any modifications to your `list` of entries will overwrite the entire list. For this reason, you must included all existing entries you wish to retain in your `list` in addition to any new entries you intend to add. To remove one or more `list` entries, pass a `list` with only the entries you wish to retain.

You should not add generic domains such as `gmail.com` or `yahoo.com`  in your `list` because your emails will not honor recipients' unsubscribes. This may cause a legal violation of [CAN-SPAM](https://sendgrid.com/docs/glossary/can-spam/) and could damage your sending reputation.

The Address Whitelist setting allows you to specify email addresses or domains for which mail should never be suppressed.

For example, if you own the domain `example.com`, and one or more of your recipients use `email@example.com` addresses, placing `example.com` in the address whitelist setting instructs Twilio SendGrid to ignore all bounces, blocks, and unsubscribes logged for that domain. In other words, all bounces, blocks, and unsubscribes will still be sent to `example.com` as if they were sent under normal sending conditions.*/
    pub fn patch_mail_settings_address_whitelist(
        &self,
    ) -> request::PatchMailSettingsAddressWhitelistRequest {
        request::PatchMailSettingsAddressWhitelistRequest {
            client: &self,
            on_behalf_of: None,
            enabled: None,
            list: None,
        }
    }
    /**Retrieve bounce purge mail settings

**This endpoint allows you to retrieve your current bounce and purge settings.**

The Bounce Perge setting allows you to set a schedule that Twilio SendGrid will use to automatically delete contacts from your soft and hard bounce suppression lists.

A hard bounce occurs when an email message has been returned to the sender because the recipient's address is invalid. A hard bounce might occur because the domain name doesn't exist or because the recipient is unknown.

A soft bounce occurs when an email message reaches the recipient's mail server but is bounced back undelivered before it actually reaches the recipient. A soft bounce might occur because the recipient's inbox is full.

You can also manage this setting in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings). You can manage your bounces manually using the [Bounces API](https://sendgrid.api-docs.io/v3.0/bounces-api) or the [Bounces menu in the Twilio SendGrid App](https://app.sendgrid.com/suppressions/bounces).*/
    pub fn get_mail_settings_bounce_purge(
        &self,
    ) -> request::GetMailSettingsBouncePurgeRequest {
        request::GetMailSettingsBouncePurgeRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update bounce purge mail settings

**This endpoint allows you to update your current bounce and purge settings.**

The Bounce Perge setting allows you to set a schedule that Twilio SendGrid will use to automatically delete contacts from your soft and hard bounce suppression lists. The schedule is set in full days by assigning the number of days, an integer, to the `soft_bounces` and/or `hard_bounces` fields.

A hard bounce occurs when an email message has been returned to the sender because the recipient's address is invalid. A hard bounce might occur because the domain name doesn't exist or because the recipient is unknown.

A soft bounce occurs when an email message reaches the recipient's mail server but is bounced back undelivered before it actually reaches the recipient. A soft bounce might occur because the recipient's inbox is full.

You can also manage this setting in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings). You can manage your bounces manually using the [Bounces API](https://sendgrid.api-docs.io/v3.0/bounces-api) or the [Bounces menu in the Twilio SendGrid App](https://app.sendgrid.com/suppressions/bounces).*/
    pub fn patch_mail_settings_bounce_purge(
        &self,
    ) -> request::PatchMailSettingsBouncePurgeRequest {
        request::PatchMailSettingsBouncePurgeRequest {
            client: &self,
            on_behalf_of: None,
            enabled: None,
            hard_bounces: None,
            soft_bounces: None,
        }
    }
    /**Retrieve footer mail settings

**This endpoint allows you to retrieve your current Footer mail settings.**

The Footer setting will insert a custom footer at the bottom of your text and HTML email message bodies.

You can insert your HTML or plain text directly using the "Update footer mail settings" endpoint, or you can create the footer using the [Mail Settings menu in the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).*/
    pub fn get_mail_settings_footer(&self) -> request::GetMailSettingsFooterRequest {
        request::GetMailSettingsFooterRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update footer mail settings

**This endpoint allows you to update your current Footer mail settings.**

The Footer setting will insert a custom footer at the bottom of your text and HTML email message bodies.

You can insert your HTML or plain text directly using this endpoint, or you can create the footer using the [Mail Settings menu in the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).*/
    pub fn patch_mail_settings_footer(&self) -> request::PatchMailSettingsFooterRequest {
        request::PatchMailSettingsFooterRequest {
            client: &self,
            on_behalf_of: None,
            enabled: None,
            html_content: None,
            plain_content: None,
        }
    }
    /**Retrieve forward bounce mail settings

**This endpoint allows you to retrieve your current bounce forwarding mail settings.**

Enabling the Forward Bounce setting allows you to specify `email` addresses to which bounce reports will be forwarded. This endpoint returns the email address you have set to receive forwarded bounces and an `enabled` status indicating if the setting is active.*/
    pub fn get_mail_settings_forward_bounce(
        &self,
    ) -> request::GetMailSettingsForwardBounceRequest {
        request::GetMailSettingsForwardBounceRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update forward bounce mail settings

**This endpoint allows you to update your current bounce forwarding mail settings.**

Enabling the Forward Bounce setting allows you to specify an `email` address to which bounce reports will be forwarded.

You can also configure the Forward Spam mail settings in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).*/
    pub fn patch_mail_settings_forward_bounce(
        &self,
    ) -> request::PatchMailSettingsForwardBounceRequest {
        request::PatchMailSettingsForwardBounceRequest {
            client: &self,
            on_behalf_of: None,
            email: None,
            enabled: None,
        }
    }
    /**Retrieve forward spam mail settings

**This endpoint allows you to retrieve your current Forward Spam mail settings.**

Enabling the Forward Spam setting allows you to specify `email` addresses to which spam reports will be forwarded. This endpoint returns any email address(es) you have set to receive forwarded spam and an `enabled` status indicating if the setting is active.*/
    pub fn get_mail_settings_forward_spam(
        &self,
    ) -> request::GetMailSettingsForwardSpamRequest {
        request::GetMailSettingsForwardSpamRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update forward spam mail settings

**This endpoint allows you to update your current Forward Spam mail settings.**

Enabling the Forward Spam setting allows you to specify `email` addresses to which spam reports will be forwarded. You can set multiple addresses by passing this endpoint a comma separated list of emails in a single string.

```
{
  "email": "address1@example.com, address2@exapmle.com",
  "enabled": true
}
```

The Forward Spam setting may also be used to receive emails sent to `abuse@` and `postmaster@` role addresses if you have authenticated your domain.

For example, if you authenticated `example.com` as your root domain and set a custom return path of `sub` for that domain, you could turn on Forward Spam, and any emails sent to `abuse@sub.example.com` or `postmaster@sub.example.com` would be forwarded to the email address you entered in the `email` field.

You can authenticate your domain using the "Authenticate a domain" endpoint or in the [Sender Authentication section of the Twilio SendGrid App](https://app.sendgrid.com/settings/sender_auth). You can also configure the Forward Spam mail settings in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).*/
    pub fn patch_mail_settings_forward_spam(
        &self,
    ) -> request::PatchMailSettingsForwardSpamRequest {
        request::PatchMailSettingsForwardSpamRequest {
            client: &self,
            on_behalf_of: None,
            email: None,
            enabled: None,
        }
    }
    /**Retrieve legacy template mail settings

**This endpoint allows you to retrieve your current legacy email template settings.**

This setting refers to our original email templates. We currently support more fully featured [Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).

The legacy email template setting wraps an HTML template around your email content. This can be useful for sending out marketing email and/or other HTML formatted messages. For instructions on using legacy templates, see how to ["Create and Edit Legacy Transactional Templates](https://sendgrid.com/docs/ui/sending-email/create-and-edit-legacy-transactional-templates/). For help migrating to our current template system, see ["Migrating from Legacy Templates"](https://sendgrid.com/docs/ui/sending-email/migrating-from-legacy-templates/).*/
    pub fn get_mail_settings_template(&self) -> request::GetMailSettingsTemplateRequest {
        request::GetMailSettingsTemplateRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update template mail settings

**This endpoint allows you to update your current legacy email template settings.**

This setting refers to our original email templates. We currently support more fully featured [Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).

The legacy email template setting wraps an HTML template around your email content. This can be useful for sending out marketing email and/or other HTML formatted messages. For instructions on using legacy templates, see how to ["Create and Edit Legacy Transactional Templates](https://sendgrid.com/docs/ui/sending-email/create-and-edit-legacy-transactional-templates/). For help migrating to our current template system, see ["Migrating from Legacy Templates"](https://sendgrid.com/docs/ui/sending-email/migrating-from-legacy-templates/).*/
    pub fn patch_mail_settings_template(
        &self,
    ) -> request::PatchMailSettingsTemplateRequest {
        request::PatchMailSettingsTemplateRequest {
            client: &self,
            on_behalf_of: None,
            enabled: None,
            html_content: None,
        }
    }
    /**Retrieve email statistics by mailbox provider.

**This endpoint allows you to retrieve your email statistics segmented by recipient mailbox provider.**

**We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.

Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).*/
    pub fn get_mailbox_providers_stats(
        &self,
        start_date: &str,
    ) -> request::GetMailboxProvidersStatsRequest {
        request::GetMailboxProvidersStatsRequest {
            client: &self,
            mailbox_providers: None,
            on_behalf_of: None,
            limit: None,
            offset: None,
            aggregated_by: None,
            start_date: start_date.to_owned(),
            end_date: None,
        }
    }
    /**Get Sample Contacts

**This endpoint will return up to 50 of the most recent contacts uploaded or attached to a list**.

This list will then be sorted by email address.

The full contact count is also returned.

Please note that pagination of the contacts has been deprecated.

Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.*/
    pub fn get_mc_contats(&self) -> request::GetMcContatsRequest {
        request::GetMcContatsRequest {
            client: &self,
        }
    }
    /**Add or Update a Contact

**This endpoint allows the [upsert](https://en.wiktionary.org/wiki/upsert) (insert or update) of up to 30,000 contacts, or 6MB of data, whichever is lower**.

Because the creation and update of contacts is an asynchronous process, the response will not contain immediate feedback on the processing of your upserted contacts. Rather, it will contain an HTTP 202 response indicating the contacts are queued for processing or an HTTP 4XX error containing validation errors. Should you wish to get the resulting contact's ID or confirm your contacts have been updated or added, you can use the "Get Contacts by Emails" endpoint.

Please note that custom fields need to have been already created if you wish to set their values for the contacts being upserted. To do this, please use the "Create Custom Field Definition" endpoint.

You will see a `job_id` in the response to your request. This can be used to check the status of your upsert job. To do so, please use the "Import Contacts Status" endpoint.

If the contact already exists in the system, any entries submitted via this endpoint will update the existing contact. The contact to update will be determined only by the `email` field and any fields omitted from the request will remain as they were. A contact's ID cannot be used to update the contact.

The email field will be changed to all lower-case. If a contact is added with an email that exists but contains capital letters, the existing contact with the all lower-case email will be updated.*/
    pub fn put_mc_contacts(
        &self,
        contacts: Vec<ContactRequest>,
    ) -> request::PutMcContactsRequest {
        request::PutMcContactsRequest {
            client: &self,
            contacts,
            list_ids: None,
        }
    }
    /**Delete Contacts

**This endpoint can be used to delete one or more contacts**.

The query parameter `ids` must set to a comma-separated list of contact IDs for bulk contact deletion.

The query parameter `delete_all_contacts` must be set to `"true"` to delete **all** contacts.

You must set either `ids` or `delete_all_contacts`.

Deletion jobs are processed asynchronously.

Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.*/
    pub fn delete_mc_contacts(&self) -> request::DeleteMcContactsRequest {
        request::DeleteMcContactsRequest {
            client: &self,
            delete_all_contacts: None,
            ids: None,
        }
    }
    /**Get Batched Contacts by IDs

**This endpoint is used to retrieve a set of contacts identified by their IDs.**

This can be more efficient endpoint to get contacts than making a series of individual `GET` requests to the "Get a Contact by ID" endpoint.

You can supply up to 100 IDs. Pass them into the `ids` field in your request body as an array or one or more strings.

Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.*/
    pub fn post_marketing_contacts_batch(
        &self,
        ids: &[&str],
    ) -> request::PostMarketingContactsBatchRequest {
        request::PostMarketingContactsBatchRequest {
            client: &self,
            ids: ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Get Total Contact Count

**This endpoint returns the total number of contacts you have stored.**


Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.*/
    pub fn get_mc_contacts_count(&self) -> request::GetMcContactsCountRequest {
        request::GetMcContactsCountRequest {
            client: &self,
        }
    }
    /**Get All Existing Exports

**Use this endpoint to retrieve details of all current exported jobs**.

It will return an array of objects, each of which records an export job in flight or recently completed.

Each object's `export_type` field will tell you which kind of export it is and its `status` field will indicate what stage of processing it has reached. Exports which are `ready` will be accompanied by a `urls` field which lists the URLs of the export's downloadable files — there will be more than one if you specified a maximum file size in your initial export request.

Use this endpoint if you have exports in flight but do not know their IDs, which are required for the "Export Contacts Status" endpoint.*/
    pub fn get_marketing_contacts_exports(
        &self,
    ) -> request::GetMarketingContactsExportsRequest {
        request::GetMarketingContactsExportsRequest {
            client: &self,
        }
    }
    /**Export Contacts

**Use this endpoint to export lists or segments of contacts**.

If you would just like to have a link to the exported list sent to your email set the `notifications.email` option to `true` in the `POST` payload.

If you would like to download the list, take the `id` that is returned and use the "Export Contacts Status" endpoint to get the `urls`. Once you have the list of URLs, make a `GET` request to each URL provided to download your CSV file(s).

You specify the segements and or/contact lists you wish to export by providing the relevant IDs in, respectively, the `segment_ids` and `list_ids` fields in the request body.

The lists will be provided in either JSON or CSV files. To specify which of these you would required, set the request body `file_type` field to `json` or `csv`.

You can also specify a maximum file size (in MB). If the export file is larger than this, it will be split into multiple files.*/
    pub fn post_mc_contacts_exports(&self) -> request::PostMcContactsExportsRequest {
        request::PostMcContactsExportsRequest {
            client: &self,
            file_type: None,
            list_ids: None,
            max_file_size: None,
            notifications: None,
            segment_ids: None,
        }
    }
    /**Export Contacts Status

**This endpoint can be used to check the status of a contact export job**.

To use this call, you will need the `id` from the "Export Contacts" call.

If you would like to download a list, take the `id` that is returned from the "Export Contacts" endpoint and make an API request here to get the `urls`. Once you have the list of URLs, make a `GET` request on each URL to download your CSV file(s).

Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.*/
    pub fn get_mc_contacts_exports_id(
        &self,
        id: &str,
    ) -> request::GetMcContactsExportsIdRequest {
        request::GetMcContactsExportsIdRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Import Contacts

**This endpoint allows a CSV upload containing up to one million contacts or 5GB of data, whichever is smaller.**

Imports take place asynchronously: the endpoint returns a URL (`upload_uri`) and HTTP headers (`upload_headers`) which can subsequently be used to `PUT` a file of contacts to be  imported into our system.

Uploaded CSV files may also be [gzip-compressed](https://en.wikipedia.org/wiki/Gzip).

In either case, you must include the field `file_type` with the value `csv` in your request body.

The `field_mappings` paramter is a respective list of field definition IDs to map the uploaded CSV columns to. It allows you to use CSVs where one or more columns are skipped (`null`) or remapped to the contact field.

For example, if `field_mappings` is set to `[null, "w1", "_rf1"]`, this means skip column 0, map column 1 to the custom field with the ID `w1`, and map column 2 to the reserved field with the ID `_rf1`. See the "Get All Field Definitions" endpoint to fetch your custom and reserved field IDs to use with `field_mappings`.

Once you recieve the response body you can then initiate a **second** API call where you use the supplied URL and HTTP header to upload your file. For example:

`curl --upload-file "file/path.csv" "URL_GIVEN" -H 'HEADER_GIVEN'`

If you'd like to monitor the status of your import job, use the `job_id` and the "Import Contacts Status" endpoint.

Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.*/
    pub fn put_mc_contacts_imports(
        &self,
        field_mappings: Vec<serde_json::Value>,
        file_type: &str,
    ) -> request::PutMcContactsImportsRequest {
        request::PutMcContactsImportsRequest {
            client: &self,
            field_mappings,
            file_type: file_type.to_owned(),
            list_ids: None,
        }
    }
    /**Import Contacts Status

**This endpoint can be used to check the status of a contact import job**.

Use the `job_id` from the "Import Contacts," "Add or Update a Contact," or "Delete Contacts" endpoints as the `id` in the path parameter.

If there is an error with your `PUT` request, download the `errors_url` file and open it to view more details.

The job `status` field indicates whether the job is `pending`, `completed`, `errored`, or `failed`.

Pending means not started. Completed means finished without any errors. Errored means finished with some errors. Failed means finshed with all errors, or the job was entirely unprocessable: for example, if you attempt to import file format we do not support.

The `results` object will have fields depending on the job type.

Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.*/
    pub fn get_marketing_contacts_imports_id(
        &self,
        id: &str,
    ) -> request::GetMarketingContactsImportsIdRequest {
        request::GetMarketingContactsImportsIdRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Search Contacts

**Use this endpoint to locate contacts**.

The request body's `query` field accepts valid [SGQL](https://sendgrid.com/docs/for-developers/sending-email/segmentation-query-language/) for searching for a contact.

Because contact emails are stored in lower case, using SGQL to search by email address requires the provided email address to be in lower case. The SGQL `lower()` function can be used for this.

Only the first 50 contacts that meet the search criteria will be returned.

If the query takes longer than 20 seconds, a `408 Request Timeout` status will be returned.

Formatting the `created_at` and `updated_at` values as Unix timestamps is deprecated. Instead they are returned as ISO format as string.*/
    pub fn post_mc_contacts_search(
        &self,
        query: &str,
    ) -> request::PostMcContactsSearchRequest {
        request::PostMcContactsSearchRequest {
            client: &self,
            query: query.to_owned(),
        }
    }
    /**Get Contacts by Emails

**This endpoint allows you to retrieve up to 100 contacts matching the searched `email` address(es), including any `alternate_emails`.**

Email addresses are unique to a contact, meaning this endpoint can treat an email address as a primary key to search by. The contact object associated with the address, whether it is their `email` or one of their `alternate_emails` will be returned if matched.

Email addresses in the search request do not need to match the case in which they're stored, but the email addresses in the result will be all lower case. Empty strings are excluded from the search and will not be returned.

This endpoint should be used in place of the "Search Contacts" endpoint when you can provide exact email addresses and do not need to include other [Segmentation Query Language (SGQL)](https://sendgrid.com/docs/for-developers/sending-email/segmentation-query-language/) filters when searching.

If you need to access a large percentage of your contacts, we recommend exporting your contacts with the "Export Contacts" endpoint and filtering the results client side.

This endpoint returns a `200` status code when any contacts match the address(es) you supplied. When searching multiple addresses in a single request, it is possible that some addresses will match a contact while others will not. When a partially successful search like this is made, the matching contacts are returned in an object and an error message is returned for the email address(es) that are not found.

This endpoint returns a `404` status code when no contacts are found for the provided email address(es).

A `400` status code is returned if any searched addresses are invalid.

Twilio SendGrid recommends exporting your contacts regularly as a backup to avoid issues or lost data.*/
    pub fn post_marketing_contacts_search_emails(
        &self,
        emails: &[&str],
    ) -> request::PostMarketingContactsSearchEmailsRequest {
        request::PostMarketingContactsSearchEmailsRequest {
            client: &self,
            emails: emails.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Get a Contact by ID

**This endpoint returns the full details and all fields for the specified contact**.

The "Get Contacts by Emails" endpoint can be used to get the ID of a contact.*/
    pub fn get_mc_contacts_id(&self, id: &str) -> request::GetMcContactsIdRequest {
        request::GetMcContactsIdRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Get All Field Definitions

**This endpoint retrieves all defined Custom Fields and Reserved Fields.***/
    pub fn get_mc_field_definitions(&self) -> request::GetMcFieldDefinitionsRequest {
        request::GetMcFieldDefinitionsRequest {
            client: &self,
        }
    }
    /**Create Custom Field Definition

**This endpoint creates a new custom field definition.**

Custom field definitions are created with the given `name` and `field_type`. Although field names are stored in a case-sensitive manner, all field names must be case-insensitively unique. This means you may create a field named `CamelCase` or `camelcase`, but not both. Additionally, a Custom Field name cannot collide with any Reserved Field names. You should save the returned `id` value in order to update or delete the field at a later date. You can have up to 120 custom fields.

The custom field name should be created using only alphanumeric characters (A-Z and 0-9) and underscores (\_). Custom fields can only begin with letters  A-Z or underscores (_). The field type can be date, text, or number fields. The field type is important for creating segments from your contact database.

**Note: Creating a custom field that begins with a number will cause issues with sending in Marketing Campaigns.***/
    pub fn post_mc_field_definitions(
        &self,
        field_type: &str,
        name: &str,
    ) -> request::PostMcFieldDefinitionsRequest {
        request::PostMcFieldDefinitionsRequest {
            client: &self,
            field_type: field_type.to_owned(),
            name: name.to_owned(),
        }
    }
    /**Delete Custom Field Definition

**This endpoint deletes a defined Custom Field.**

You cand delete only Custom Fields; Reserved Fields cannot be deleted.*/
    pub fn delete_mc_field_definitions_custom_field_id(
        &self,
        custom_field_id: &str,
    ) -> request::DeleteMcFieldDefinitionsCustomFieldIdRequest {
        request::DeleteMcFieldDefinitionsCustomFieldIdRequest {
            client: &self,
            custom_field_id: custom_field_id.to_owned(),
        }
    }
    /**Update Custom Field Definition

**This endopoint allows you to update a defined Custom Field.**

Only your Custom fields can be modified; Reserved Fields cannot be updated.*/
    pub fn patch_mc_field_definitions_custom_field_id(
        &self,
        custom_field_id: &str,
        name: &str,
    ) -> request::PatchMcFieldDefinitionsCustomFieldIdRequest {
        request::PatchMcFieldDefinitionsCustomFieldIdRequest {
            client: &self,
            custom_field_id: custom_field_id.to_owned(),
            name: name.to_owned(),
        }
    }
    /**Get All Lists

**This endpoint returns an array of all of your contact lists.***/
    pub fn get_mc_lists(&self) -> request::GetMcListsRequest {
        request::GetMcListsRequest {
            client: &self,
            page_size: None,
            page_token: None,
        }
    }
    /**Create List

**This endpoint creates a new contacts list.**

Once you create a list, you can use the UI to [trigger an automation](https://sendgrid.com/docs/ui/sending-email/getting-started-with-automation/#create-an-automation) every time you add a new contact to the list.

A link to the newly created object is in `_metadata`.*/
    pub fn post_mc_lists(&self, name: &str) -> request::PostMcListsRequest {
        request::PostMcListsRequest {
            client: &self,
            name: name.to_owned(),
        }
    }
    /**Get a List by ID

**This endpoint returns data about a specific list.**

Setting the optional parameter `contact_sample=true` returns the `contact_sample` in the response body. Up to fifty of the most recent contacts uploaded or attached to a list will be returned, sorted alphabetically, by email address.

The full contact count is also returned.*/
    pub fn get_mc_lists_id(&self, id: &str) -> request::GetMcListsIdRequest {
        request::GetMcListsIdRequest {
            client: &self,
            contact_sample: None,
            id: id.to_owned(),
        }
    }
    /**Delete a list

**This endpoint allows you to deletes a specific list.**

Optionally, you can also delete contacts associated to the list. The query parameter, `delete_contacts=true`, will delete the list and start an asynchronous job to delete associated contacts.*/
    pub fn delete_lists_id(&self, id: &str) -> request::DeleteListsIdRequest {
        request::DeleteListsIdRequest {
            client: &self,
            delete_contacts: None,
            id: id.to_owned(),
        }
    }
    /**Update List

**This endpoint updates the name of a list.***/
    pub fn patch_mc_lists_id(&self, id: &str) -> request::PatchMcListsIdRequest {
        request::PatchMcListsIdRequest {
            client: &self,
            id: id.to_owned(),
            name: None,
        }
    }
    /**Remove Contacts from a List

**This endpoint allows you to remove contacts from a given list.**

The contacts will not be deleted. Only their list membership will be changed.*/
    pub fn delete_mc_lists_id_contacts(
        &self,
        contact_ids: &str,
        id: &str,
    ) -> request::DeleteMcListsIdContactsRequest {
        request::DeleteMcListsIdContactsRequest {
            client: &self,
            contact_ids: contact_ids.to_owned(),
            id: id.to_owned(),
        }
    }
    /**Get List Contact Count

**This endpoint returns the number of contacts on a specific list.***/
    pub fn get_mc_lists_id_contacts_count(
        &self,
        id: &str,
    ) -> request::GetMcListsIdContactsCountRequest {
        request::GetMcListsIdContactsCountRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Get List of Segments

**This endpoint allows you to retrieve a list of segments.**

The query param `parent_list_ids` is treated as a filter.  Any match will be returned.  Zero matches will return a response code of 200 with an empty `results` array.

`parent_list_ids` | `no_parent_list_id` | `ids` | `result`
-----------------:|:--------------------:|:-------------:|:-------------:
empty | false | empty | all segments values
list_ids | false | empty | segments filtered by list_ids values
list_ids |true | empty | segments filtered by list_ids and segments with no parent list_ids empty
empty | true | empty | segments with no parent list_ids
anything | anything | ids | segments with matching segment ids |*/
    pub fn get_marketing_segments(&self) -> request::GetMarketingSegmentsRequest {
        request::GetMarketingSegmentsRequest {
            client: &self,
            ids: None,
            parent_list_ids: None,
            no_parent_list_id: None,
        }
    }
    /**Create Segment

**This endpoint allows you to create a segment.***/
    pub fn post_marketing_segments(
        &self,
        args: request::PostMarketingSegmentsRequired,
    ) -> request::PostMarketingSegmentsRequest {
        request::PostMarketingSegmentsRequest {
            client: &self,
            name: args.name.to_owned(),
            parent_list_ids: args
                .parent_list_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
            query_dsl: args.query_dsl.to_owned(),
            parent_list_id: args.parent_list_id.to_owned(),
        }
    }
    /**Get List of Segments

**This endpoint allows you to retrieve a list of segments.**

The query param `parent_list_ids` is treated as a filter.  Any match will be returned.  Zero matches will return a response code of 200 with an empty `results` array.

`parent_list_ids` | `no_parent_list_id` | `ids` | `result`
-----------------:|:--------------------:|:-------------:|:-------------:
empty | false | empty | all segments values
list_ids | false | empty | segments filtered by list_ids values
list_ids |true | empty | segments filtered by list_ids and segments with no parent list_ids empty
empty | true | empty | segments with no parent list_ids
anything | anything | ids | segments with matching segment ids |*/
    pub fn get_segments(&self) -> request::GetSegmentsRequest {
        request::GetSegmentsRequest {
            client: &self,
            ids: None,
            parent_list_ids: None,
            no_parent_list_id: None,
        }
    }
    /**Create Segment

Segment `name` has to be unique. A user can not create a new segment with an existing segment name.*/
    pub fn post_segments(
        &self,
        name: &str,
        query_dsl: &str,
    ) -> request::PostSegmentsRequest {
        request::PostSegmentsRequest {
            client: &self,
            name: name.to_owned(),
            parent_list_ids: None,
            query_dsl: query_dsl.to_owned(),
        }
    }
    ///Get Segment by ID
    pub fn get_segments_segment_id(
        &self,
        segment_id: &str,
    ) -> request::GetSegmentsSegmentIdRequest {
        request::GetSegmentsSegmentIdRequest {
            client: &self,
            contacts_sample: None,
            segment_id: segment_id.to_owned(),
        }
    }
    /**Delete segment

**This endpoint allows you to delete a segment by ID.***/
    pub fn delete_segments_segment_id(
        &self,
        segment_id: &str,
    ) -> request::DeleteSegmentsSegmentIdRequest {
        request::DeleteSegmentsSegmentIdRequest {
            client: &self,
            segment_id: segment_id.to_owned(),
        }
    }
    /**Update Segment

Segment `name` has to be unique. A user can not create a new segment with an existing segment name.*/
    pub fn patch_segments_segment_id(
        &self,
        segment_id: &str,
    ) -> request::PatchSegmentsSegmentIdRequest {
        request::PatchSegmentsSegmentIdRequest {
            client: &self,
            segment_id: segment_id.to_owned(),
            name: None,
            query_dsl: None,
        }
    }
    /**Get Segment by ID

**This endpoint allows you to retrieve a single segment by ID.***/
    pub fn get_marketing_segments_segment_id(
        &self,
        segment_id: &str,
    ) -> request::GetMarketingSegmentsSegmentIdRequest {
        request::GetMarketingSegmentsSegmentIdRequest {
            client: &self,
            query_json: None,
            segment_id: segment_id.to_owned(),
        }
    }
    /**Delete Segment

**This endpoint allows you to delete a segment by `segment_id`.**

Note that deleting a segment does not delete the contacts associated with the segment by default. Contacts associated with a deleted segment will remain in your list of all contacts and any other segments they belong to.*/
    pub fn delete_marketing_segments_segment_id(
        &self,
        segment_id: &str,
    ) -> request::DeleteMarketingSegmentsSegmentIdRequest {
        request::DeleteMarketingSegmentsSegmentIdRequest {
            client: &self,
            segment_id: segment_id.to_owned(),
        }
    }
    /**Update Segment

**This endpoint allows you to update a segment.**

Segment `name` needs to be unique. A user can not update a segment name to an existing one.*/
    pub fn patch_marketing_segments_segment_id(
        &self,
        segment_id: &str,
        name: &str,
        query_dsl: &str,
    ) -> request::PatchMarketingSegmentsSegmentIdRequest {
        request::PatchMarketingSegmentsSegmentIdRequest {
            client: &self,
            segment_id: segment_id.to_owned(),
            name: name.to_owned(),
            parent_list_ids: None,
            query_dsl: query_dsl.to_owned(),
        }
    }
    /**Create a Sender Identity

**This endpoint allows you to create a new sender identity.**

*You may create up to 100 unique sender identities.*

Sender identities are required to be verified before use. If your domain has been authenticated, a new sender identity will auto verify on creation. Otherwise an email will be sent to the `from.email`.*/
    pub fn post_marketing_senders(
        &self,
        args: request::PostMarketingSendersRequired,
    ) -> request::PostMarketingSendersRequest {
        request::PostMarketingSendersRequest {
            client: &self,
            on_behalf_of: None,
            address: args.address.to_owned(),
            address2: None,
            city: args.city.to_owned(),
            country: args.country.to_owned(),
            from: args.from,
            nickname: args.nickname.to_owned(),
            reply_to: None,
            state: None,
            zip: None,
        }
    }
    /**Get All Single Sends

**This endpoint allows you to retrieve all your Single Sends.**

Returns all of your Single Sends with condensed details about each, including the Single Sends' IDs. For more details about an individual Single Send, pass the Single Send's ID to the `/marketing/singlesends/{id}` endpoint.*/
    pub fn get_marketing_singlesends(&self) -> request::GetMarketingSinglesendsRequest {
        request::GetMarketingSinglesendsRequest {
            client: &self,
            page_size: None,
            page_token: None,
        }
    }
    /**Create Single Send

**This endpoint allows you to create a new Single Send.**

Please note that if you are migrating from the previous version of Single Sends, you no longer need to pass a template ID with your request to this endpoint. Instead, you will pass all template data in the `email_config` object.*/
    pub fn post_marketing_singlesends(
        &self,
        name: &str,
    ) -> request::PostMarketingSinglesendsRequest {
        request::PostMarketingSinglesendsRequest {
            client: &self,
            categories: None,
            email_config: None,
            name: name.to_owned(),
            send_at: None,
            send_to: None,
        }
    }
    /**Bulk Delete Single Sends

**This endpoint allows you to delete multiple Single Sends using an array of Single Sends IDs.**

To first retrieve all your Single Sends' IDs, you can make a GET request to the `/marketing/singlensends` endpoint.

Please note that a DELETE request is permanent, and your Single Sends will not be recoverable after deletion.*/
    pub fn delete_marketing_singlesends(
        &self,
    ) -> request::DeleteMarketingSinglesendsRequest {
        request::DeleteMarketingSinglesendsRequest {
            client: &self,
            ids: None,
        }
    }
    /**Get All Categories

**This endpoint allows you to retrieve all the categories associated with your Single Sends.**

This endpoint will return your latest 1,000 categories.*/
    pub fn get_marketing_singlesends_categories(
        &self,
    ) -> request::GetMarketingSinglesendsCategoriesRequest {
        request::GetMarketingSinglesendsCategoriesRequest {
            client: &self,
        }
    }
    /**Get Single Sends Search

**This endpoint allows you to search for Single Sends based on specified criteria.**

You can search for Single Sends by passing a combination of values using the `name`, `status`, and `categories` request body fields.

For example, if you want to search for all Single Sends that are "drafts" or "scheduled" and also associated with the category "shoes," your request body may look like the example below.

```javascript
{
  "status": [
    "draft",
    "scheduled"
  ],
  "categories": [
    "shoes"
  ],
}
```*/
    pub fn post_marketing_singlesends_search(
        &self,
    ) -> request::PostMarketingSinglesendsSearchRequest {
        request::PostMarketingSinglesendsSearchRequest {
            client: &self,
            page_size: None,
            page_token: None,
            categories: None,
            name: None,
            status: None,
        }
    }
    /**Get Single Send by ID

**This endpoint allows you to retrieve details about one Single Send using a Single Send ID.**

You can retrieve all of your Single Sends by making a GET request to the `/marketing/singlesends` endpoint.*/
    pub fn get_marketing_singlesends_id(
        &self,
        id: &str,
    ) -> request::GetMarketingSinglesendsIdRequest {
        request::GetMarketingSinglesendsIdRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Duplicate Single Send

**This endpoint allows you to duplicate an existing Single Send using its Single Send ID.**

Duplicating a Single Send is useful when you want to create a Single Send but don't want to start from scratch. Once duplicated, you can update or edit the Single Send by making a PATCH request to the `/marketing/singlesends/{id}` endpoint.

If you leave the `name` field blank, your duplicate will be assigned the name of the Single Send it was copied from with the text “Copy of ” prepended to it. The `name` field length is limited to 100 characters, so the end of the new Single Send name, including “Copy of ”, will be trimmed if the name exceeds this limit.*/
    pub fn post_marketing_singlesends_id(
        &self,
        id: &str,
    ) -> request::PostMarketingSinglesendsIdRequest {
        request::PostMarketingSinglesendsIdRequest {
            client: &self,
            id: id.to_owned(),
            name: None,
        }
    }
    /**Delete Single Send by ID

**This endpoint allows you to delete one Single Send using a Single Send ID.**

To first retrieve all your Single Sends' IDs, you can make a GET request to the `/marketing/singlensends` endpoint.

Please note that a `DELETE` request is permanent, and your Single Send will not be recoverable after deletion.*/
    pub fn delete_marketing_singlesends_id(
        &self,
        id: &str,
    ) -> request::DeleteMarketingSinglesendsIdRequest {
        request::DeleteMarketingSinglesendsIdRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Update Single Send

**This endpoint allows you to update a Single Send using a Single Send ID.**

You only need to pass the fields you want to update. Any blank/missing fields will remain unaltered.*/
    pub fn patch_marketing_singlesends_id(
        &self,
        id: &str,
        name: &str,
    ) -> request::PatchMarketingSinglesendsIdRequest {
        request::PatchMarketingSinglesendsIdRequest {
            client: &self,
            id: id.to_owned(),
            categories: None,
            email_config: None,
            name: name.to_owned(),
            send_at: None,
            send_to: None,
        }
    }
    /**Schedule Single Send

**This endpoint allows you to schedule a Single Send for future delivery using a Single Send ID.**

To schedule a Single Send, you must pass a date string in ISO 8601 time format (yyyy-MM-ddTHH:mm:ssZ)  using the required `send_at` field. For example, the ISO 8601 format for 9:00 AM UTC on May 6, 2020 would be `2020-05-06T09:00:00Z`. You may also pass the string `"now"` to send the Single Send immediately.*/
    pub fn put_marketing_singlesends_id_schedule(
        &self,
        id: &str,
        send_at: &str,
    ) -> request::PutMarketingSinglesendsIdScheduleRequest {
        request::PutMarketingSinglesendsIdScheduleRequest {
            client: &self,
            id: id.to_owned(),
            send_at: send_at.to_owned(),
        }
    }
    /**Delete Single Send Schedule

**This endpoint allows you to cancel a scheduled Single Send using a Single Send ID.**

Making a DELETE request to this endpoint will cancel the scheduled sending of a Single Send. The request will not delete the Single Send itself. Deleting a Single Send can be done by passing a DELETE request to `/marketing/singlesends/{id}`.*/
    pub fn delete_marketing_singlesends_id_schedule(
        &self,
        id: &str,
    ) -> request::DeleteMarketingSinglesendsIdScheduleRequest {
        request::DeleteMarketingSinglesendsIdScheduleRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Get All Automation Stats

**This endpoint allows you to retrieve stats for all your Automations.**

By default, all of your Automations will be returned, but you can specify a selection by passing in a comma-separated list of Automation IDs as the value of the query string parameter `automation_ids`.

Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.

You can retrieve a specific page of responses with the `page_token` query string parameter.*/
    pub fn getall_automation_stats(&self) -> request::GetallAutomationStatsRequest {
        request::GetallAutomationStatsRequest {
            client: &self,
            automation_ids: None,
            page_size: None,
            page_token: None,
        }
    }
    /**Export Automation Stats

**This endpoint allows you to export Automation stats as CSV data**.

You can specify one Automation or many: include as many Automation IDs as you need, separating them with commas, as the value of the `ids` query string paramter.

The data is returned as plain text response but in CSV format, so your application making the call can present the information in whatever way is most appropriate, or just save the data as a `.csv` file.*/
    pub fn get_automations_stats_export(
        &self,
    ) -> request::GetAutomationsStatsExportRequest {
        request::GetAutomationsStatsExportRequest {
            client: &self,
            ids: None,
            timezone: None,
        }
    }
    /**Get Automation Stats by ID

**This endpoint allows you to retrieve stats for a single Automation using its ID.**

Multiple Automation IDs can be retrieved using the "Get All Automation Stats" endpoint. Once you have an ID, this endpoint will return detailed stats for the single automation specified.

You may constrain the stats returned using the `start_date` and `end_date` query string parameters. You can also use the `group_by` and `aggregated_by` query string parameters to further refine the stats returned.*/
    pub fn get_automation_stat(&self, id: &str) -> request::GetAutomationStatRequest {
        request::GetAutomationStatRequest {
            client: &self,
            group_by: None,
            step_ids: None,
            aggregated_by: None,
            start_date: None,
            end_date: None,
            timezone: None,
            page_size: None,
            page_token: None,
            id: id.to_owned(),
        }
    }
    /**Get Automation Click Tracking Stats by ID

**This endpoint lets you retrieve click-tracking stats for a single Automation**.

The stats returned list the URLs embedded in your Automation and the number of clicks each one received.

Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.

You can retrieve a specific page of responses with the `page_token` query string parameter.*/
    pub fn get_automation_link_stat(
        &self,
        id: &str,
    ) -> request::GetAutomationLinkStatRequest {
        request::GetAutomationLinkStatRequest {
            client: &self,
            group_by: None,
            step_ids: None,
            page_size: None,
            page_token: None,
            id: id.to_owned(),
        }
    }
    /**Get All Single Sends Stats

**This endpoint allows you to retrieve stats for all your Single Sends.**

By default, all of your Single Sends will be returned, but you can specify a selection by passing in a comma-separated list of Single Send IDs as the value of the query string parameter `singlesend_ids`.

Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.

You can retrieve a specific page of responses with the `page_token` query string parameter.*/
    pub fn getall_singlesend_stats(&self) -> request::GetallSinglesendStatsRequest {
        request::GetallSinglesendStatsRequest {
            client: &self,
            singlesend_ids: None,
            page_size: None,
            page_token: None,
        }
    }
    /**Export Single Send Stats

**This endpoint allows you to export Single Send stats as .CSV data**.

You can specify one Single Send or many: include as many Single Send IDs as you need, separating them with commas, as the value of the `ids` query string paramter.

The data is returned as plain text response but in .CSV format, so your application making the call can present the information in whatever way is most appropriate, or just save the data as a .csv file.*/
    pub fn get_singlesend_stats_export(
        &self,
    ) -> request::GetSinglesendStatsExportRequest {
        request::GetSinglesendStatsExportRequest {
            client: &self,
            ids: None,
            timezone: None,
        }
    }
    /**Get Single Send Stats by ID

**This endpoint allows you to retrieve stats for an individual Single Send using a Single Send ID.**

Multiple Single Send IDs can be retrieved using the "Get All Single Sends Stats" endpoint. Once you have an ID, this endpoint will return detailed stats for the Single Send specified.

You may constrain the stats returned using the `start_date` and `end_date` query string parameters. You can also use the `group_by` and `aggregated_by` query string parameters to further refine the stats returned.*/
    pub fn get_singlesend_stat(&self, id: &str) -> request::GetSinglesendStatRequest {
        request::GetSinglesendStatRequest {
            client: &self,
            aggregated_by: None,
            start_date: None,
            end_date: None,
            timezone: None,
            page_size: None,
            page_token: None,
            group_by: None,
            id: id.to_owned(),
        }
    }
    /**Get Single Send Click Tracking Stats by ID

**This endpoint lets you retrieve click-tracking stats for one Single Send**.

The stats returned list the URLs embedded in the specified Single Send and the number of clicks each one received.

Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.

You can retrieve a specific page of responses with the `page_token` query string parameter.*/
    pub fn get_singlesend_link_stat(
        &self,
        id: &str,
    ) -> request::GetSinglesendLinkStatRequest {
        request::GetSinglesendLinkStatRequest {
            client: &self,
            page_size: None,
            page_token: None,
            group_by: None,
            ab_variation_id: None,
            ab_phase_id: None,
            id: id.to_owned(),
        }
    }
    /**Send a Test Marketing Email

**This endpoint allows you to send a test marketing email to a list of email addresses**.

Before sending a marketing message, you can test it using this endpoint. You may specify up to **10 contacts** in the `emails` request body field. You must also specify a `template_id` and include either a `from_address` or `sender_id`. You can manage your templates with the [Twilio SendGrid App](https://mc.sendgrid.com/dynamic-templates) or the [Transactional Templates API](https://sendgrid.api-docs.io/v3.0/transactional-templates).

> Please note that this endpoint works with Dynamic Transactional Templates only. Legacy Transactional Templates will not be delivered.

For more information about managing Dynamic Transactional Templates, see [How to Send Email with Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).

You can also test your Single Sends in the [Twilio SendGrid Marketing Campaigns UI](https://mc.sendgrid.com/single-sends).*/
    pub fn post_marketing_test_send_email(
        &self,
        emails: &[&str],
        template_id: &str,
    ) -> request::PostMarketingTestSendEmailRequest {
        request::PostMarketingTestSendEmailRequest {
            client: &self,
            custom_unsubscribe_url: None,
            emails: emails.iter().map(|&x| x.to_owned()).collect(),
            from_address: None,
            sender_id: None,
            suppression_group_id: None,
            template_id: template_id.to_owned(),
            version_id_override: None,
        }
    }
    /**Filter all messages

Filter all messages to search your Email Activity. All queries must be [URL encoded](https://meyerweb.com/eric/tools/dencoder/), and use the following format:

`query={query_type}="{query_content}"`

 Once URL encoded, the previous query will look like this:

`query=type%3D%22query_content%22`

For example, to filter by a specific email, use the following query:

`query=to_email%3D%22example%40example.com%22`

Visit our [Query Reference section](https://docs.sendgrid.com/for-developers/sending-email/getting-started-email-activity-api#query-reference) to see a full list of basic query types and examples.*/
    pub fn get_messages(
        &self,
        query: &str,
        authorization: &str,
    ) -> request::GetMessagesRequest {
        request::GetMessagesRequest {
            client: &self,
            query: query.to_owned(),
            limit: None,
            authorization: authorization.to_owned(),
        }
    }
    /**Request CSV

This request will kick off a backend process to generate a CSV file. Once generated, the worker will then send an email for the user download the file. The link will expire in 3 days.

The CSV fill contain the last 1 million messages. This endpoint will be rate limited to 1 request every 12 hours (rate limit may change).

This endpoint is similar to the GET Single Message endpoint - the only difference is that /download is added to indicate that this is a CSV download requests but the same query is used to determine what the CSV should contain.*/
    pub fn post_v3_messages_download(
        &self,
        authorization: &str,
    ) -> request::PostV3MessagesDownloadRequest {
        request::PostV3MessagesDownloadRequest {
            client: &self,
            query: None,
            authorization: authorization.to_owned(),
        }
    }
    /**Download CSV

**This endpoint will return a presigned URL that can be used to download the CSV that was requested from the "Request a CSV" endpoint.***/
    pub fn get_v3_messages_download_download_uuid(
        &self,
        authorization: &str,
        download_uuid: &str,
    ) -> request::GetV3MessagesDownloadDownloadUuidRequest {
        request::GetV3MessagesDownloadDownloadUuidRequest {
            client: &self,
            authorization: authorization.to_owned(),
            download_uuid: download_uuid.to_owned(),
        }
    }
    /**Filter messages by message ID

Get all of the details about the specified message.*/
    pub fn get_v3_messages_msg_id(
        &self,
        authorization: &str,
        msg_id: &str,
    ) -> request::GetV3MessagesMsgIdRequest {
        request::GetV3MessagesMsgIdRequest {
            client: &self,
            authorization: authorization.to_owned(),
            msg_id: msg_id.to_owned(),
        }
    }
    /**Returns a list of all partner settings.

**This endpoint allows you to retrieve a list of all partner settings that you can enable.**

Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).*/
    pub fn get_partner_settings(&self) -> request::GetPartnerSettingsRequest {
        request::GetPartnerSettingsRequest {
            client: &self,
            limit: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Returns all New Relic partner settings.

**This endpoint allows you to retrieve your current New Relic partner settings.**

Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).

By integrating with New Relic, you can send your SendGrid email statistics to your New Relic Dashboard. If you enable this setting, your stats will be sent to New Relic every 5 minutes. You will need your New Relic License Key to enable this setting. For more information, please see our [SendGrid for New Relic documentation](https://sendgrid.com/docs/ui/analytics-and-reporting/tracking-stats-using-new-relic/).*/
    pub fn get_partner_settings_new_relic(
        &self,
    ) -> request::GetPartnerSettingsNewRelicRequest {
        request::GetPartnerSettingsNewRelicRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Updates New Relic partner settings.

**This endpoint allows you to update or change your New Relic partner settings.**

Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).

By integrating with New Relic, you can send your SendGrid email statistics to your New Relic Dashboard. If you enable this setting, your stats will be sent to New Relic every 5 minutes. You will need your New Relic License Key to enable this setting. For more information, please see our [SendGrid for New Relic documentation](https://sendgrid.com/docs/ui/analytics-and-reporting/tracking-stats-using-new-relic/).*/
    pub fn patch_partner_settings_new_relic(
        &self,
    ) -> request::PatchPartnerSettingsNewRelicRequest {
        request::PatchPartnerSettingsNewRelicRequest {
            client: &self,
            on_behalf_of: None,
            enable_subuser_statistics: None,
            enabled: None,
            license_key: None,
        }
    }
    /**Retrieve a list of scopes for which this user has access.

**This endpoint returns a list of all scopes that this user has access to.**

API Keys are used to authenticate with [SendGrid's v3 API](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization).

API Keys may be assigned certain permissions, or scopes, that limit which API endpoints they are able to access.

This endpoint returns all the scopes assigned to the key you use to authenticate with it. To retrieve the scopes assigned to another key, you can pass an API key ID to the "Retrieve an existing API key" endpoint.

For a more detailed explanation of how you can use API Key permissions, please visit our [API Keys documentation](https://sendgrid.com/docs/ui/account-and-settings/api-keys/).*/
    pub fn get_scopes(&self) -> request::GetScopesRequest {
        request::GetScopesRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Retrieve access requests

**This endpoint allows you to retrieve a list of all recent access requests.**

The Response Header's `link` parameter will include pagination info.*/
    pub fn get_v3_scopes_requests(&self) -> request::GetV3ScopesRequestsRequest {
        request::GetV3ScopesRequestsRequest {
            client: &self,
            limit: None,
            offset: None,
        }
    }
    /**Deny access request

**This endpoint allows you to deny an attempt to access your account.**

**Note:** Only teammate admins may delete a teammate's access request.*/
    pub fn delete_v3_scopes_requests_request_id(
        &self,
        request_id: &str,
    ) -> request::DeleteV3ScopesRequestsRequestIdRequest {
        request::DeleteV3ScopesRequestsRequestIdRequest {
            client: &self,
            request_id: request_id.to_owned(),
        }
    }
    /**Approve access request

**This endpoint allows you to approve an access attempt.**

**Note:** Only teammate admins may approve another teammate’s access request.*/
    pub fn patch_v3_scopes_requests_approve_id(
        &self,
        request_id: &str,
    ) -> request::PatchV3ScopesRequestsApproveIdRequest {
        request::PatchV3ScopesRequestsApproveIdRequest {
            client: &self,
            request_id: request_id.to_owned(),
        }
    }
    /**Get all Sender Identities

**This endpoint allows you to retrieve a list of all sender identities that have been created for your account.***/
    pub fn get_v3_senders(&self) -> request::GetV3SendersRequest {
        request::GetV3SendersRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Create a Sender Identity

**This endpoint allows you to create a new sender identity.**

You may create up to 100 unique sender identities.*/
    pub fn post_senders(
        &self,
        args: request::PostSendersRequired,
    ) -> request::PostSendersRequest {
        request::PostSendersRequest {
            client: &self,
            on_behalf_of: None,
            address: args.address.to_owned(),
            address2: args.address2.to_owned(),
            city: args.city.to_owned(),
            country: args.country.to_owned(),
            from: args.from,
            nickname: args.nickname.to_owned(),
            reply_to: args.reply_to,
            state: args.state.to_owned(),
            zip: args.zip.to_owned(),
        }
    }
    /**View a Sender Identity

**This endpoint allows you to retrieve a specific sender identity.***/
    pub fn get_v3_senders_sender_id(
        &self,
        sender_id: i64,
    ) -> request::GetV3SendersSenderIdRequest {
        request::GetV3SendersSenderIdRequest {
            client: &self,
            on_behalf_of: None,
            sender_id,
        }
    }
    /**Delete a Sender Identity

**This endoint allows you to delete one of your sender identities.***/
    pub fn delete_v3_senders_sender_id(
        &self,
        sender_id: i64,
    ) -> request::DeleteV3SendersSenderIdRequest {
        request::DeleteV3SendersSenderIdRequest {
            client: &self,
            on_behalf_of: None,
            sender_id,
        }
    }
    /**Update a Sender Identity

**This endpoint allows you to update a sender identity.**

Updates to `from.email` require re-verification.

Partial updates are allowed, but fields that are marked as "required" in the POST (create) endpoint must not be nil if that field is included in the PATCH request.*/
    pub fn patch_v3_senders_sender_id(
        &self,
        sender_id: i64,
    ) -> request::PatchV3SendersSenderIdRequest {
        request::PatchV3SendersSenderIdRequest {
            client: &self,
            on_behalf_of: None,
            sender_id,
            address: None,
            address2: None,
            city: None,
            country: None,
            from: None,
            nickname: None,
            reply_to: None,
            state: None,
            zip: None,
        }
    }
    /**Resend Sender Identity Verification

**This enpdoint allows you to resend a sender identity verification email.***/
    pub fn post_v3_senders_sender_id_resend_verification(
        &self,
        sender_id: i64,
    ) -> request::PostV3SendersSenderIdResendVerificationRequest {
        request::PostV3SendersSenderIdResendVerificationRequest {
            client: &self,
            on_behalf_of: None,
            sender_id,
        }
    }
    /**Create an SSO Certificate

**This endpoint allows you to create an SSO certificate.***/
    pub fn post_sso_certificates(
        &self,
        integration_id: &str,
        public_certificate: &str,
    ) -> request::PostSsoCertificatesRequest {
        request::PostSsoCertificatesRequest {
            client: &self,
            enabled: None,
            integration_id: integration_id.to_owned(),
            public_certificate: public_certificate.to_owned(),
        }
    }
    /**Get an SSO Certificate

**This endpoint allows you to retrieve an individual SSO certificate.***/
    pub fn get_sso_certificates_cert_id(
        &self,
        cert_id: &str,
    ) -> request::GetSsoCertificatesCertIdRequest {
        request::GetSsoCertificatesCertIdRequest {
            client: &self,
            cert_id: cert_id.to_owned(),
        }
    }
    /**Delete an SSO Certificate

**This endpoint allows you to delete an SSO certificate.**

You can retrieve a certificate's ID from the response provided by the "Get All SSO Integrations" endpoint.*/
    pub fn delete_sso_certificates_cert_id(
        &self,
        cert_id: &str,
    ) -> request::DeleteSsoCertificatesCertIdRequest {
        request::DeleteSsoCertificatesCertIdRequest {
            client: &self,
            cert_id: cert_id.to_owned(),
        }
    }
    /**Update SSO Certificate

**This endpoint allows you to update an existing certificate by ID.**

You can retrieve a certificate's ID from the response provided by the "Get All SSO Integrations" endpoint.*/
    pub fn patch_sso_certificates_cert_id(
        &self,
        cert_id: &str,
    ) -> request::PatchSsoCertificatesCertIdRequest {
        request::PatchSsoCertificatesCertIdRequest {
            client: &self,
            cert_id: cert_id.to_owned(),
            enabled: None,
            integration_id: None,
            public_certificate: None,
        }
    }
    /**Get All SSO Integrations

**This endpoint allows you to retrieve all SSO integrations tied to your Twilio SendGrid account.**

The IDs returned by this endpoint can be used by the APIs additional endpoints to modify your SSO integrations.*/
    pub fn get_sso_integrations(&self) -> request::GetSsoIntegrationsRequest {
        request::GetSsoIntegrationsRequest {
            client: &self,
            si: None,
        }
    }
    /**Create an SSO Integration

**This endpoint allows you to create an SSO integration.***/
    pub fn post_sso_integrations(
        &self,
        args: request::PostSsoIntegrationsRequired,
    ) -> request::PostSsoIntegrationsRequest {
        request::PostSsoIntegrationsRequest {
            client: &self,
            completed_integration: None,
            enabled: args.enabled,
            entity_id: args.entity_id.to_owned(),
            name: args.name.to_owned(),
            signin_url: args.signin_url.to_owned(),
            signout_url: args.signout_url.to_owned(),
        }
    }
    /**Get an SSO Integration

**This endpoint allows you to retrieve an SSO integration by ID.**

You can retrieve the IDs for your configurations from the response provided by the "Get All SSO Integrations" endpoint.*/
    pub fn get_sso_integrations_id(
        &self,
        id: &str,
    ) -> request::GetSsoIntegrationsIdRequest {
        request::GetSsoIntegrationsIdRequest {
            client: &self,
            si: None,
            id: id.to_owned(),
        }
    }
    /**Delete an SSO Integration

**This endpoint allows you to delete an IdP configuration by ID.**

You can retrieve the IDs for your configurations from the response provided by the "Get All SSO Integrations" endpoint.*/
    pub fn delete_sso_integrations_id(
        &self,
        id: &str,
    ) -> request::DeleteSsoIntegrationsIdRequest {
        request::DeleteSsoIntegrationsIdRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Update an SSO Integration

**This endpoint allows you to modify an exisiting SSO integration.**

You can retrieve the IDs for your configurations from the response provided by the "Get All SSO Integrations" endpoint.*/
    pub fn patch_sso_integrations_id(
        &self,
        args: request::PatchSsoIntegrationsIdRequired,
    ) -> request::PatchSsoIntegrationsIdRequest {
        request::PatchSsoIntegrationsIdRequest {
            client: &self,
            si: None,
            id: args.id.to_owned(),
            completed_integration: None,
            enabled: args.enabled,
            entity_id: args.entity_id.to_owned(),
            name: args.name.to_owned(),
            signin_url: args.signin_url.to_owned(),
            signout_url: args.signout_url.to_owned(),
        }
    }
    /**Get All SSO Certificates by Integration

**This endpoint allows you to retrieve all your IdP configurations by configuration ID.**

The `integration_id` expected by this endpoint is the `id` returned in the response by the "Get All SSO Integrations" endpoint.*/
    pub fn get_sso_integrations_integration_id_certificates(
        &self,
        integration_id: &str,
    ) -> request::GetSsoIntegrationsIntegrationIdCertificatesRequest {
        request::GetSsoIntegrationsIntegrationIdCertificatesRequest {
            client: &self,
            integration_id: integration_id.to_owned(),
        }
    }
    /**Create SSO Teammate

**This endpoint allows you to create an SSO Teammate.**

The email provided for this user will also function as the Teammate’s username.*/
    pub fn post_sso_teammates(
        &self,
        args: request::PostSsoTeammatesRequired,
    ) -> request::PostSsoTeammatesRequest {
        request::PostSsoTeammatesRequest {
            client: &self,
            email: args.email.to_owned(),
            first_name: args.first_name.to_owned(),
            is_admin: args.is_admin,
            is_read_only: args.is_read_only,
            last_name: args.last_name.to_owned(),
            scopes: args.scopes.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Edit an SSO Teammate

**This endpoint allows you to modify an existing SSO Teammate.**

To turn a teammate into an admin, the request body should contain the `is_admin` field set to `true`. Otherwise, set `is_admin` to false and pass in all the scopes that a teammate should have.

Only the parent user and Teammates with admin permissions can update another Teammate’s permissions. Admin users can only update permissions.*/
    pub fn patch_sso_teammates_username(
        &self,
        username: &str,
    ) -> request::PatchSsoTeammatesUsernameRequest {
        request::PatchSsoTeammatesUsernameRequest {
            client: &self,
            username: username.to_owned(),
            first_name: None,
            is_admin: None,
            last_name: None,
            scopes: None,
        }
    }
    /**Retrieve global email statistics

**This endpoint allows you to retrieve all of your global email statistics between a given date range.**

Parent accounts can see either aggregated stats for the parent account or aggregated stats for a subuser specified in the `on-behalf-of` header. Subuser accounts will see only their own stats.*/
    pub fn get_stats(&self, start_date: &str) -> request::GetStatsRequest {
        request::GetStatsRequest {
            client: &self,
            on_behalf_of: None,
            limit: None,
            offset: None,
            aggregated_by: None,
            start_date: start_date.to_owned(),
            end_date: None,
        }
    }
    /**List all Subusers

**This endpoint allows you to retrieve a list of all of your subusers.**

You can choose to retrieve specific subusers as well as limit the results that come back from the API.*/
    pub fn get_subusers(&self) -> request::GetSubusersRequest {
        request::GetSubusersRequest {
            client: &self,
            username: None,
            limit: None,
            offset: None,
        }
    }
    /**Create Subuser

**This endpoint allows you to create a new subuser.***/
    pub fn post_subusers(
        &self,
        args: request::PostSubusersRequired,
    ) -> request::PostSubusersRequest {
        request::PostSubusersRequest {
            client: &self,
            email: args.email.to_owned(),
            ips: args.ips.iter().map(|&x| x.to_owned()).collect(),
            password: args.password.to_owned(),
            username: args.username.to_owned(),
        }
    }
    /**Retrieve Subuser Reputations

**This endpoint allows you to request the reputations for your subusers.**

Subuser sender reputations give a good idea how well a sender is doing with regards to how recipients and recipient servers react to the mail that is being received. When a bounce, spam report, or other negative action happens on a sent email, it will affect your sender rating.*/
    pub fn get_subusers_reputations(&self) -> request::GetSubusersReputationsRequest {
        request::GetSubusersReputationsRequest {
            client: &self,
            usernames: None,
        }
    }
    /**Retrieve email statistics for your subusers.

**This endpoint allows you to retrieve the email statistics for the given subusers.**

You may retrieve statistics for up to 10 different subusers by including an additional _subusers_ parameter for each additional subuser.*/
    pub fn get_subusers_stats(
        &self,
        subusers: &str,
        start_date: &str,
    ) -> request::GetSubusersStatsRequest {
        request::GetSubusersStatsRequest {
            client: &self,
            limit: None,
            offset: None,
            aggregated_by: None,
            subusers: subusers.to_owned(),
            start_date: start_date.to_owned(),
            end_date: None,
        }
    }
    /**Retrieve monthly stats for all subusers

**This endpoint allows you to retrieve the monthly email statistics for all subusers over the given date range.**

When using the `sort_by_metric` to sort your stats by a specific metric, you can not sort by the following metrics:
`bounce_drops`, `deferred`, `invalid_emails`, `processed`, `spam_report_drops`, `spam_reports`, or `unsubscribe_drops`.*/
    pub fn get_subusers_stats_monthly(
        &self,
        date: &str,
    ) -> request::GetSubusersStatsMonthlyRequest {
        request::GetSubusersStatsMonthlyRequest {
            client: &self,
            date: date.to_owned(),
            subuser: None,
            sort_by_metric: None,
            sort_by_direction: None,
            limit: None,
            offset: None,
        }
    }
    /**Retrieve the totals for each email statistic metric for all subusers.

**This endpoint allows you to retrieve the total sums of each email statistic metric for all subusers over the given date range.***/
    pub fn get_subusers_stats_sums(
        &self,
        start_date: &str,
    ) -> request::GetSubusersStatsSumsRequest {
        request::GetSubusersStatsSumsRequest {
            client: &self,
            sort_by_direction: None,
            start_date: start_date.to_owned(),
            end_date: None,
            limit: None,
            offset: None,
            aggregated_by: None,
            sort_by_metric: None,
        }
    }
    /**Delete a subuser

**This endpoint allows you to delete a subuser.**

This is a permanent action. Once deleted, a subuser cannot be retrieved.*/
    pub fn delete_subusers_subuser_name(
        &self,
        subuser_name: &str,
    ) -> request::DeleteSubusersSubuserNameRequest {
        request::DeleteSubusersSubuserNameRequest {
            client: &self,
            subuser_name: subuser_name.to_owned(),
        }
    }
    /**Enable/disable a subuser

**This endpoint allows you to enable or disable a subuser.***/
    pub fn patch_subusers_subuser_name(
        &self,
        subuser_name: &str,
    ) -> request::PatchSubusersSubuserNameRequest {
        request::PatchSubusersSubuserNameRequest {
            client: &self,
            subuser_name: subuser_name.to_owned(),
            disabled: None,
        }
    }
    /**Update IPs assigned to a subuser

**This endpoint allows you update your subusers' assigned IP.**

Each subuser should be assigned to an IP address from which all of this subuser's mail will be sent. Often, this is the same IP as the parent account, but each subuser can have one or more of their own IP addresses as well.

More information:

* [How to request more IPs](https://sendgrid.com/docs/ui/account-and-settings/dedicated-ip-addresses/)
* [Setup Reverse DNS](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-reverse-dns/)*/
    pub fn put_subusers_subuser_name_ips(
        &self,
        subuser_name: &str,
        body: serde_json::Value,
    ) -> request::PutSubusersSubuserNameIpsRequest {
        request::PutSubusersSubuserNameIpsRequest {
            client: &self,
            subuser_name: subuser_name.to_owned(),
            body,
        }
    }
    ///Retrieve monitor settings for a subuser
    pub fn get_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
    ) -> request::GetSubusersSubuserNameMonitorRequest {
        request::GetSubusersSubuserNameMonitorRequest {
            client: &self,
            subuser_name: subuser_name.to_owned(),
        }
    }
    ///Update Monitor Settings for a subuser
    pub fn put_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
        email: &str,
        frequency: f64,
    ) -> request::PutSubusersSubuserNameMonitorRequest {
        request::PutSubusersSubuserNameMonitorRequest {
            client: &self,
            subuser_name: subuser_name.to_owned(),
            email: email.to_owned(),
            frequency,
        }
    }
    ///Create monitor settings
    pub fn post_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
        email: &str,
        frequency: f64,
    ) -> request::PostSubusersSubuserNameMonitorRequest {
        request::PostSubusersSubuserNameMonitorRequest {
            client: &self,
            subuser_name: subuser_name.to_owned(),
            email: email.to_owned(),
            frequency,
        }
    }
    ///Delete monitor settings
    pub fn delete_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
    ) -> request::DeleteSubusersSubuserNameMonitorRequest {
        request::DeleteSubusersSubuserNameMonitorRequest {
            client: &self,
            subuser_name: subuser_name.to_owned(),
        }
    }
    /**Retrieve the monthly email statistics for a single subuser

**This endpoint allows you to retrive the monthly email statistics for a specific subuser.**

When using the `sort_by_metric` to sort your stats by a specific metric, you can not sort by the following metrics:
`bounce_drops`, `deferred`, `invalid_emails`, `processed`, `spam_report_drops`, `spam_reports`, or `unsubscribe_drops`.*/
    pub fn get_subusers_subuser_name_stats_monthly(
        &self,
        date: &str,
        subuser_name: &str,
    ) -> request::GetSubusersSubuserNameStatsMonthlyRequest {
        request::GetSubusersSubuserNameStatsMonthlyRequest {
            client: &self,
            date: date.to_owned(),
            sort_by_metric: None,
            sort_by_direction: None,
            limit: None,
            offset: None,
            subuser_name: subuser_name.to_owned(),
        }
    }
    /**Retrieve all blocks

**This endpoint allows you to retrieve all email addresses that are currently on your blocks list. A maximum of 500 blocks will be returned per query. You can use the `offset` and `limit` parameters to retrieve more or less than 500 results.***/
    pub fn get_suppression_blocks(&self) -> request::GetSuppressionBlocksRequest {
        request::GetSuppressionBlocksRequest {
            client: &self,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Delete blocks

**This endpoint allows you to delete all email addresses on your blocks list.**

There are two options for deleting blocked emails:

1. You can delete all blocked emails by setting `delete_all` to `true` in the request body.
2. You can delete a selection of blocked emails by specifying the email addresses in the `emails` array of the request body.*/
    pub fn delete_suppression_blocks(&self) -> request::DeleteSuppressionBlocksRequest {
        request::DeleteSuppressionBlocksRequest {
            client: &self,
            on_behalf_of: None,
            delete_all: None,
            emails: None,
        }
    }
    /**Retrieve a specific block

**This endpoint allows you to retrieve a specific email address from your blocks list.***/
    pub fn get_suppression_blocks_email(
        &self,
        email: &str,
    ) -> request::GetSuppressionBlocksEmailRequest {
        request::GetSuppressionBlocksEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Delete a specific block

**This endpoint allows you to delete a specific email address from your blocks list.***/
    pub fn delete_suppression_blocks_email(
        &self,
        email: &str,
    ) -> request::DeleteSuppressionBlocksEmailRequest {
        request::DeleteSuppressionBlocksEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Retrieve all bounces

**This endpoint allows you to retrieve all of your bounces. A maximum of 500 bounces will be returned per query. You can use the `offset` and `limit` parameters to retrieve more or less than 500 results.***/
    pub fn get_suppression_bounces(
        &self,
        accept: &str,
    ) -> request::GetSuppressionBouncesRequest {
        request::GetSuppressionBouncesRequest {
            client: &self,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
            accept: accept.to_owned(),
            on_behalf_of: None,
        }
    }
    /**Delete bounces

**This endpoint allows you to delete all emails on your bounces list.**

There are two options for deleting bounced emails:

1. You can delete all bounced emails by setting `delete_all` to `true` in the request body.
2. You can delete a selection of bounced emails by specifying the email addresses in the `emails` array of the request body.

**WARNING:** You can not have both `emails` and `delete_all` set.*/
    pub fn delete_suppression_bounces(
        &self,
    ) -> request::DeleteSuppressionBouncesRequest {
        request::DeleteSuppressionBouncesRequest {
            client: &self,
            on_behalf_of: None,
            delete_all: None,
            emails: None,
        }
    }
    /**Retrieve bounce classification totals

This endpoint will return the total number of bounces by classification in descending order for each day. You can retrieve the bounce classification totals in CSV format by specifying `"text/csv"` in the Accept header.*/
    pub fn get_suppression_bounces_classifications(
        &self,
        accept: &str,
    ) -> request::GetSuppressionBouncesClassificationsRequest {
        request::GetSuppressionBouncesClassificationsRequest {
            client: &self,
            accept: accept.to_owned(),
            start_date: None,
            end_date: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve bounce classification over time by domain stats

This endpoint will return the number of bounces for the classification specified in descending order for each day. You can retrieve the bounce classification totals in CSV format by specifying `"text/csv"` in the Accept header.*/
    pub fn get_suppressions_bounces_classifications_classification(
        &self,
        accept: &str,
        classification: &str,
    ) -> request::GetSuppressionsBouncesClassificationsClassificationRequest {
        request::GetSuppressionsBouncesClassificationsClassificationRequest {
            client: &self,
            accept: accept.to_owned(),
            classification: classification.to_owned(),
            start_date: None,
            end_date: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve a Bounce

**This endpoint allows you to retrieve a specific bounce by email address.***/
    pub fn get_suppression_bounces_email(
        &self,
        email: &str,
    ) -> request::GetSuppressionBouncesEmailRequest {
        request::GetSuppressionBouncesEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Delete a bounce

**This endpoint allows you to remove an email address from your bounce list.***/
    pub fn delete_suppression_bounces_email(
        &self,
        email_address: &str,
        email: &str,
    ) -> request::DeleteSuppressionBouncesEmailRequest {
        request::DeleteSuppressionBouncesEmailRequest {
            client: &self,
            email_address: email_address.to_owned(),
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Retrieve all invalid emails

**This endpoint allows you to retrieve a list of all invalid email addresses.***/
    pub fn get_suppression_invalid_emails(
        &self,
    ) -> request::GetSuppressionInvalidEmailsRequest {
        request::GetSuppressionInvalidEmailsRequest {
            client: &self,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Delete invalid emails

**This endpoint allows you to remove email addresses from your invalid email address list.**

There are two options for deleting invalid email addresses:

1) You can delete all invalid email addresses by setting `delete_all` to true in the request body.
2) You can delete some invalid email addresses by specifying certain addresses in an array in the request body.*/
    pub fn delete_suppression_invalid_emails(
        &self,
    ) -> request::DeleteSuppressionInvalidEmailsRequest {
        request::DeleteSuppressionInvalidEmailsRequest {
            client: &self,
            on_behalf_of: None,
            delete_all: None,
            emails: None,
        }
    }
    /**Retrieve a specific invalid email

**This endpoint allows you to retrieve a specific invalid email addresses.***/
    pub fn get_suppression_invalid_emails_email(
        &self,
        email: &str,
    ) -> request::GetSuppressionInvalidEmailsEmailRequest {
        request::GetSuppressionInvalidEmailsEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Delete a specific invalid email

**This endpoint allows you to remove a specific email address from the invalid email address list.***/
    pub fn delete_suppression_invalid_emails_email(
        &self,
        email: &str,
    ) -> request::DeleteSuppressionInvalidEmailsEmailRequest {
        request::DeleteSuppressionInvalidEmailsEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Retrieve all spam reports

**This endpoint allows you to retrieve all spam reports.***/
    pub fn get_suppression_spam_reports(
        &self,
    ) -> request::GetSuppressionSpamReportsRequest {
        request::GetSuppressionSpamReportsRequest {
            client: &self,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Delete spam reports

**This endpoint allows you to delete your spam reports.**

Deleting a spam report will remove the suppression, meaning email will once again be sent to the previously suppressed address. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.

There are two options for deleting spam reports:

1. You can delete all spam reports by setting the `delete_all` field to `true` in the request body.
2. You can delete a list of select spam reports by specifying the email addresses in the `emails` array of the request body.*/
    pub fn delete_suppression_spam_reports(
        &self,
    ) -> request::DeleteSuppressionSpamReportsRequest {
        request::DeleteSuppressionSpamReportsRequest {
            client: &self,
            on_behalf_of: None,
            delete_all: None,
            emails: None,
        }
    }
    /**Retrieve a specific spam report

**This endpoint allows you to retrieve a specific spam report by email address.***/
    pub fn get_suppression_spam_reports_email(
        &self,
        email: &str,
    ) -> request::GetSuppressionSpamReportsEmailRequest {
        request::GetSuppressionSpamReportsEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Delete a specific spam report

**This endpoint allows you to delete a specific spam report by email address.**

Deleting a spam report will remove the suppression, meaning email will once again be sent to the previously suppressed address. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.*/
    pub fn delete_suppression_spam_reports_email(
        &self,
        email: &str,
    ) -> request::DeleteSuppressionSpamReportsEmailRequest {
        request::DeleteSuppressionSpamReportsEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
        }
    }
    /**Retrieve all global suppressions

**This endpoint allows you to retrieve a list of all email address that are globally suppressed.***/
    pub fn get_suppression_unsubscribes(
        &self,
    ) -> request::GetSuppressionUnsubscribesRequest {
        request::GetSuppressionUnsubscribesRequest {
            client: &self,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve all teammates

**This endpoint allows you to retrieve a list of all current Teammates.**

You can limit the number of results returned using the `limit` query paramater. To return results from a specific Teammate, use the `offset` paramter. The Response Headers will include pagination info.*/
    pub fn get_v3_teammates(&self) -> request::GetV3TeammatesRequest {
        request::GetV3TeammatesRequest {
            client: &self,
            limit: None,
            offset: None,
            on_behalf_of: None,
        }
    }
    /**Invite teammate

**This endpoint allows you to invite a Teammate to your account via email.**

You can set a Teammate's initial permissions using the `scopes` array in the request body. Teammate's will receive a minimum set of scopes from Twilio SendGrid that are necessary for the Teammate to function.

**Note:** A teammate invite will expire after 7 days, but you may resend the invitation at any time to reset the expiration date.*/
    pub fn post_v3_teammates(
        &self,
        email: &str,
        is_admin: bool,
        scopes: &[&str],
    ) -> request::PostV3TeammatesRequest {
        request::PostV3TeammatesRequest {
            client: &self,
            on_behalf_of: None,
            email: email.to_owned(),
            is_admin,
            scopes: scopes.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Retrieve all pending teammates

**This endpoint allows you to retrieve a list of all pending Teammate invitations.**

Each teammate invitation is valid for 7 days. Users may resend the invitation to refresh the expiration date.*/
    pub fn get_v3_teammates_pending(&self) -> request::GetV3TeammatesPendingRequest {
        request::GetV3TeammatesPendingRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Delete pending teammate

**This endpoint allows you to delete a pending teammate invite.***/
    pub fn delete_v3_teammates_pending_token(
        &self,
        token: &str,
    ) -> request::DeleteV3TeammatesPendingTokenRequest {
        request::DeleteV3TeammatesPendingTokenRequest {
            client: &self,
            on_behalf_of: None,
            token: token.to_owned(),
        }
    }
    /**Resend teammate invite

**This endpoint allows you to resend a Teammate invitation.**

Teammate invitations will expire after 7 days. Resending an invitation will reset the expiration date.*/
    pub fn post_v3_teammates_pending_token_resend(
        &self,
        token: &str,
    ) -> request::PostV3TeammatesPendingTokenResendRequest {
        request::PostV3TeammatesPendingTokenResendRequest {
            client: &self,
            on_behalf_of: None,
            token: token.to_owned(),
        }
    }
    /**Retrieve specific teammate

**This endpoint allows you to retrieve a specific Teammate by username.**

You can retrieve the username's for each of your Teammates using the "Retrieve all Teammates" endpoint.*/
    pub fn get_v3_teammates_username(
        &self,
        username: &str,
    ) -> request::GetV3TeammatesUsernameRequest {
        request::GetV3TeammatesUsernameRequest {
            client: &self,
            on_behalf_of: None,
            username: username.to_owned(),
        }
    }
    /**Delete teammate

**This endpoint allows you to delete a teammate.**

**Only the parent user or an admin teammate can delete another teammate.***/
    pub fn delete_v3_teammates_username(
        &self,
        username: &str,
    ) -> request::DeleteV3TeammatesUsernameRequest {
        request::DeleteV3TeammatesUsernameRequest {
            client: &self,
            on_behalf_of: None,
            username: username.to_owned(),
        }
    }
    /**Update teammate's permissions

**This endpoint allows you to update a teammate’s permissions.**

To turn a teammate into an admin, the request body should contain an `is_admin` set to `true`. Otherwise, set `is_admin` to `false` and pass in all the scopes that a teammate should have.

**Only the parent user or other admin teammates can update another teammate’s permissions.**

**Admin users can only update permissions.***/
    pub fn patch_v3_teammates_username(
        &self,
        username: &str,
        is_admin: bool,
        scopes: &[&str],
    ) -> request::PatchV3TeammatesUsernameRequest {
        request::PatchV3TeammatesUsernameRequest {
            client: &self,
            on_behalf_of: None,
            username: username.to_owned(),
            is_admin,
            scopes: scopes.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Retrieve paged transactional templates.

**This endpoint allows you to retrieve all transactional templates.***/
    pub fn get_templates(&self, page_size: f64) -> request::GetTemplatesRequest {
        request::GetTemplatesRequest {
            client: &self,
            generations: None,
            page_size,
            page_token: None,
            on_behalf_of: None,
        }
    }
    /**Create a transactional template.

**This endpoint allows you to create a transactional template.***/
    pub fn post_templates(&self, name: &str) -> request::PostTemplatesRequest {
        request::PostTemplatesRequest {
            client: &self,
            on_behalf_of: None,
            generation: None,
            name: name.to_owned(),
        }
    }
    /**Retrieve a single transactional template.

**This endpoint allows you to retrieve a single transactional template.***/
    pub fn get_templates_template_id(
        &self,
        template_id: &str,
    ) -> request::GetTemplatesTemplateIdRequest {
        request::GetTemplatesTemplateIdRequest {
            client: &self,
            on_behalf_of: None,
            template_id: template_id.to_owned(),
        }
    }
    /**Duplicate a transactional template.

**This endpoint allows you to duplicate a transactional template.***/
    pub fn post_templates_template_id(
        &self,
        template_id: &str,
    ) -> request::PostTemplatesTemplateIdRequest {
        request::PostTemplatesTemplateIdRequest {
            client: &self,
            on_behalf_of: None,
            template_id: template_id.to_owned(),
            name: None,
        }
    }
    /**Delete a template.

**This endpoint allows you to delete a transactional template.***/
    pub fn delete_templates_template_id(
        &self,
        template_id: &str,
    ) -> request::DeleteTemplatesTemplateIdRequest {
        request::DeleteTemplatesTemplateIdRequest {
            client: &self,
            on_behalf_of: None,
            template_id: template_id.to_owned(),
        }
    }
    /**Edit a transactional template.

**This endpoint allows you to edit the name of a transactional template.**

To edit the template itself, [create a new transactional template version](https://sendgrid.api-docs.io/v3.0/transactional-templates-versions/create-a-new-transactional-template-version).*/
    pub fn patch_templates_template_id(
        &self,
        template_id: &str,
    ) -> request::PatchTemplatesTemplateIdRequest {
        request::PatchTemplatesTemplateIdRequest {
            client: &self,
            on_behalf_of: None,
            template_id: template_id.to_owned(),
            name: None,
        }
    }
    /**Create a new transactional template version.

**This endpoint allows you to create a new version of a template.***/
    pub fn post_templates_template_id_versions(
        &self,
        template_id: &str,
        name: &str,
        subject: &str,
    ) -> request::PostTemplatesTemplateIdVersionsRequest {
        request::PostTemplatesTemplateIdVersionsRequest {
            client: &self,
            on_behalf_of: None,
            template_id: template_id.to_owned(),
            active: None,
            editor: None,
            generate_plain_content: None,
            html_content: None,
            name: name.to_owned(),
            plain_content: None,
            subject: subject.to_owned(),
            test_data: None,
        }
    }
    /**Retrieve a specific transactional template version.

**This endpoint allows you to retrieve a specific version of a template.***/
    pub fn get_templates_template_id_versions_version_id(
        &self,
        template_id: &str,
        version_id: &str,
    ) -> request::GetTemplatesTemplateIdVersionsVersionIdRequest {
        request::GetTemplatesTemplateIdVersionsVersionIdRequest {
            client: &self,
            on_behalf_of: None,
            template_id: template_id.to_owned(),
            version_id: version_id.to_owned(),
        }
    }
    /**Delete a transactional template version.

**This endpoint allows you to delete a transactional template version.***/
    pub fn delete_templates_template_id_versions_version_id(
        &self,
        template_id: &str,
        version_id: &str,
    ) -> request::DeleteTemplatesTemplateIdVersionsVersionIdRequest {
        request::DeleteTemplatesTemplateIdVersionsVersionIdRequest {
            client: &self,
            on_behalf_of: None,
            template_id: template_id.to_owned(),
            version_id: version_id.to_owned(),
        }
    }
    /**Edit a transactional template version.

**This endpoint allows you to edit the content of your template version.***/
    pub fn patch_templates_template_id_versions_version_id(
        &self,
        args: request::PatchTemplatesTemplateIdVersionsVersionIdRequired,
    ) -> request::PatchTemplatesTemplateIdVersionsVersionIdRequest {
        request::PatchTemplatesTemplateIdVersionsVersionIdRequest {
            client: &self,
            on_behalf_of: None,
            template_id: args.template_id.to_owned(),
            version_id: args.version_id.to_owned(),
            active: None,
            editor: None,
            generate_plain_content: None,
            html_content: None,
            name: args.name.to_owned(),
            plain_content: None,
            subject: args.subject.to_owned(),
            test_data: None,
        }
    }
    /**Activate a transactional template version.

**This endpoint allows you to activate a version of one of your templates.***/
    pub fn post_templates_template_id_versions_version_id_activate(
        &self,
        template_id: &str,
        version_id: &str,
    ) -> request::PostTemplatesTemplateIdVersionsVersionIdActivateRequest {
        request::PostTemplatesTemplateIdVersionsVersionIdActivateRequest {
            client: &self,
            on_behalf_of: None,
            template_id: template_id.to_owned(),
            version_id: version_id.to_owned(),
        }
    }
    /**Retrieve Tracking Settings

**This endpoint allows you to retrieve a list of all tracking settings on your account.***/
    pub fn get_tracking_settings(&self) -> request::GetTrackingSettingsRequest {
        request::GetTrackingSettingsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Retrieve Click Track Settings

**This endpoint allows you to retrieve your current click tracking setting.**

Click Tracking overrides all the links and URLs in your emails and points them to either SendGrid’s servers or the domain with which you branded your link. When a customer clicks a link, SendGrid tracks those [clicks](https://sendgrid.com/docs/glossary/clicks/).

Click tracking helps you understand how users are engaging with your communications. SendGrid can track up to 1000 links per email*/
    pub fn get_tracking_settings_click(
        &self,
    ) -> request::GetTrackingSettingsClickRequest {
        request::GetTrackingSettingsClickRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update Click Tracking Settings

**This endpoint allows you to enable or disable your current click tracking setting.**

Click Tracking overrides all the links and URLs in your emails and points them to either SendGrid’s servers or the domain with which you branded your link. When a customer clicks a link, SendGrid tracks those [clicks](https://sendgrid.com/docs/glossary/clicks/).

Click tracking helps you understand how users are engaging with your communications. SendGrid can track up to 1000 links per email*/
    pub fn patch_tracking_settings_click(
        &self,
    ) -> request::PatchTrackingSettingsClickRequest {
        request::PatchTrackingSettingsClickRequest {
            client: &self,
            on_behalf_of: None,
            enabled: None,
        }
    }
    /**Retrieve Google Analytics Settings

**This endpoint allows you to retrieve your current setting for Google Analytics.**


Google Analytics helps you understand how users got to your site and what they're doing there. For more information about using Google Analytics, please refer to [Google’s URL Builder](https://support.google.com/analytics/answer/1033867?hl=en) and their article on ["Best Practices for Campaign Building"](https://support.google.com/analytics/answer/1037445).

We default the settings to Google’s recommendations. For more information, see [Google Analytics Demystified](https://sendgrid.com/docs/ui/analytics-and-reporting/google-analytics/).*/
    pub fn get_tracking_settings_google_analytics(
        &self,
    ) -> request::GetTrackingSettingsGoogleAnalyticsRequest {
        request::GetTrackingSettingsGoogleAnalyticsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update Google Analytics Settings

**This endpoint allows you to update your current setting for Google Analytics.**

Google Analytics helps you understand how users got to your site and what they're doing there. For more information about using Google Analytics, please refer to [Google’s URL Builder](https://support.google.com/analytics/answer/1033867?hl=en) and their article on ["Best Practices for Campaign Building"](https://support.google.com/analytics/answer/1037445).

We default the settings to Google’s recommendations. For more information, see [Google Analytics Demystified](https://sendgrid.com/docs/ui/analytics-and-reporting/google-analytics/).*/
    pub fn patch_tracking_settings_google_analytics(
        &self,
    ) -> request::PatchTrackingSettingsGoogleAnalyticsRequest {
        request::PatchTrackingSettingsGoogleAnalyticsRequest {
            client: &self,
            on_behalf_of: None,
            enabled: None,
            utm_campaign: None,
            utm_content: None,
            utm_medium: None,
            utm_source: None,
            utm_term: None,
        }
    }
    /**Get Open Tracking Settings

**This endpoint allows you to retrieve your current settings for open tracking.**

Open Tracking adds an invisible image at the end of the email which can track email opens.

If the email recipient has images enabled on their email client, a request to SendGrid’s server for the invisible image is executed and an open event is logged.

These events are logged in the Statistics portal, Email Activity interface, and are reported by the Event Webhook.*/
    pub fn get_tracking_settings_open(&self) -> request::GetTrackingSettingsOpenRequest {
        request::GetTrackingSettingsOpenRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update Open Tracking Settings

**This endpoint allows you to update your current settings for open tracking.**

Open Tracking adds an invisible image at the end of the email which can track email opens.

If the email recipient has images enabled on their email client, a request to SendGrid’s server for the invisible image is executed and an open event is logged.

These events are logged in the Statistics portal, Email Activity interface, and are reported by the Event Webhook.*/
    pub fn patch_tracking_settings_open(
        &self,
    ) -> request::PatchTrackingSettingsOpenRequest {
        request::PatchTrackingSettingsOpenRequest {
            client: &self,
            on_behalf_of: None,
            enabled: None,
        }
    }
    /**Retrieve Subscription Tracking Settings

**This endpoint allows you to retrieve your current settings for subscription tracking.**

Subscription tracking adds links to the bottom of your emails that allows your recipients to subscribe to, or unsubscribe from, your emails.*/
    pub fn get_tracking_settings_subscription(
        &self,
    ) -> request::GetTrackingSettingsSubscriptionRequest {
        request::GetTrackingSettingsSubscriptionRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update Subscription Tracking Settings

**This endpoint allows you to update your current settings for subscription tracking.**

Subscription tracking adds links to the bottom of your emails that allows your recipients to subscribe to, or unsubscribe from, your emails.*/
    pub fn patch_tracking_settings_subscription(
        &self,
    ) -> request::PatchTrackingSettingsSubscriptionRequest {
        request::PatchTrackingSettingsSubscriptionRequest {
            client: &self,
            on_behalf_of: None,
            enabled: None,
            html_content: None,
            landing: None,
            plain_content: None,
            replace: None,
            url: None,
        }
    }
    /**Get a user's account information.

**This endpoint allows you to retrieve your user account details.**

Your user's account information includes the user's account type and reputation.*/
    pub fn get_user_account(&self) -> request::GetUserAccountRequest {
        request::GetUserAccountRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Retrieve your credit balance

**This endpoint allows you to retrieve the current credit balance for your account.**

Each account has a credit balance, which is a base number of emails it can send before receiving per-email charges. For more information about credits and billing, see [Billing and Plan details information](https://sendgrid.com/docs/ui/account-and-settings/billing/).*/
    pub fn get_user_credits(&self) -> request::GetUserCreditsRequest {
        request::GetUserCreditsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Retrieve your account email address

**This endpoint allows you to retrieve the email address currently on file for your account.***/
    pub fn get_user_email(&self) -> request::GetUserEmailRequest {
        request::GetUserEmailRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update your account email address

**This endpoint allows you to update the email address currently on file for your account.***/
    pub fn put_user_email(&self) -> request::PutUserEmailRequest {
        request::PutUserEmailRequest {
            client: &self,
            on_behalf_of: None,
            email: None,
        }
    }
    /**Update your password

**This endpoint allows you to update your password.***/
    pub fn put_user_password(
        &self,
        new_password: &str,
        old_password: &str,
    ) -> request::PutUserPasswordRequest {
        request::PutUserPasswordRequest {
            client: &self,
            on_behalf_of: None,
            new_password: new_password.to_owned(),
            old_password: old_password.to_owned(),
        }
    }
    ///Get a user's profile
    pub fn get_user_profile(&self) -> request::GetUserProfileRequest {
        request::GetUserProfileRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update a user's profile

**This endpoint allows you to update your current profile details.**

Any one or more of the parameters can be updated via the PATCH `/user/profile` endpoint. You must include at least one when you PATCH.*/
    pub fn patch_user_profile(&self) -> request::PatchUserProfileRequest {
        request::PatchUserProfileRequest {
            client: &self,
            on_behalf_of: None,
            address: None,
            address2: None,
            city: None,
            company: None,
            country: None,
            first_name: None,
            last_name: None,
            phone: None,
            state: None,
            website: None,
            zip: None,
        }
    }
    /**Retrieve all scheduled sends

**This endpoint allows you to retrieve all cancelled and paused scheduled send information.**

This endpoint will return only the scheduled sends that are associated with a `batch_id`. If you have scheduled a send using the `/mail/send` endpoint and the `send_at` field but no `batch_id`, the send will be scheduled for delivery; however, it will not be returned by this endpoint. For this reason, you should assign a `batch_id` to any scheduled send you may need to pause or cancel in the future.*/
    pub fn get_user_scheduled_sends(&self) -> request::GetUserScheduledSendsRequest {
        request::GetUserScheduledSendsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Cancel or pause a scheduled send

**This endpoint allows you to cancel or pause a scheduled send associated with a `batch_id`.**

Passing this endpoint a `batch_id` and status will cancel or pause the scheduled send.

Once a scheduled send is set to `pause` or `cancel` you must use the "Update a scheduled send" endpoint to change its status or the "Delete a cancellation or pause from a scheduled send" endpoint to remove the status. Passing a status change to a scheduled send that has already been paused or cancelled will result in a `400` level status code.

If the maximum number of cancellations/pauses are added to a send, a `400` level status code will be returned.*/
    pub fn post_user_scheduled_sends(
        &self,
        batch_id: &str,
        status: &str,
    ) -> request::PostUserScheduledSendsRequest {
        request::PostUserScheduledSendsRequest {
            client: &self,
            on_behalf_of: None,
            batch_id: batch_id.to_owned(),
            status: status.to_owned(),
        }
    }
    /**Retrieve scheduled send

**This endpoint allows you to retrieve the cancel/paused scheduled send information for a specific `batch_id`.***/
    pub fn get_user_scheduled_sends_batch_id(
        &self,
        batch_id: &str,
    ) -> request::GetUserScheduledSendsBatchIdRequest {
        request::GetUserScheduledSendsBatchIdRequest {
            client: &self,
            on_behalf_of: None,
            batch_id: batch_id.to_owned(),
        }
    }
    /**Delete a cancellation or pause from a scheduled send

**This endpoint allows you to delete the cancellation/pause of a scheduled send.**

Scheduled sends cancelled less than 10 minutes before the scheduled time are not guaranteed to be cancelled.*/
    pub fn delete_user_scheduled_sends_batch_id(
        &self,
        batch_id: &str,
    ) -> request::DeleteUserScheduledSendsBatchIdRequest {
        request::DeleteUserScheduledSendsBatchIdRequest {
            client: &self,
            on_behalf_of: None,
            batch_id: batch_id.to_owned(),
        }
    }
    /**Update a scheduled send

**This endpoint allows you to update the status of a scheduled send for the given `batch_id`.**

If you have already set a `cancel` or `pause` status on a scheduled send using the "Cancel or pause a scheduled send" endpoint, you can update it's status using this endpoint. Attempting to update a status once it has been set with the "Cancel or pause a scheduled send" endpoint will result in a `400` error.*/
    pub fn patch_user_scheduled_sends_batch_id(
        &self,
        batch_id: &str,
        status: &str,
    ) -> request::PatchUserScheduledSendsBatchIdRequest {
        request::PatchUserScheduledSendsBatchIdRequest {
            client: &self,
            on_behalf_of: None,
            batch_id: batch_id.to_owned(),
            status: status.to_owned(),
        }
    }
    /**Retrieve current Enforced TLS settings.

**This endpoint allows you to retrieve your current Enforced TLS settings.**

The Enforced TLS settings specify whether or not the recipient is required to support TLS or have a valid certificate.

If either `require_tls` or `require_valid_cert` is set to `true`, the recipient must support TLS 1.1 or higher or have a valid certificate. If these conditions are not met, Twilio SendGrid will drop the message and send a block event with “TLS required but not supported” as the description.*/
    pub fn get_user_settings_enforced_tls(
        &self,
    ) -> request::GetUserSettingsEnforcedTlsRequest {
        request::GetUserSettingsEnforcedTlsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update Enforced TLS settings

**This endpoint allows you to update your Enforced TLS settings.**

To require TLS from recipients, set `require_tls` to `true`. If either `require_tls` or `require_valid_cert` is set to `true`, the recipient must support TLS 1.1 or higher or have a valid certificate. If these conditions are not met, Twilio SendGrid will drop the message and send a block event with “TLS required but not supported” as the description.

> Twilio SendGrid supports TLS 1.1 and higher and does not support older versions of TLS due to security vulnerabilities.*/
    pub fn patch_user_settings_enforced_tls(
        &self,
    ) -> request::PatchUserSettingsEnforcedTlsRequest {
        request::PatchUserSettingsEnforcedTlsRequest {
            client: &self,
            on_behalf_of: None,
            require_tls: None,
            require_valid_cert: None,
            version: None,
        }
    }
    /**Retrieve your username

**This endpoint allows you to retrieve your current account username.***/
    pub fn get_user_username(&self) -> request::GetUserUsernameRequest {
        request::GetUserUsernameRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update your username

**This endpoint allows you to update the username for your account.***/
    pub fn put_user_username(&self) -> request::PutUserUsernameRequest {
        request::PutUserUsernameRequest {
            client: &self,
            on_behalf_of: None,
            username: None,
        }
    }
    /**Retrieve Event Webhook settings

**This endpoint allows you to retrieve your current event webhook settings.**

If an event type is marked as `true`, then the event webhook will include information about that event.

SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.

Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.*/
    pub fn get_user_webhooks_event_settings(
        &self,
    ) -> request::GetUserWebhooksEventSettingsRequest {
        request::GetUserWebhooksEventSettingsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Update Event Notification Settings

**This endpoint allows you to update your current event webhook settings.**

If an event type is marked as `true`, then the event webhook will include information about that event.

SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.

Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.*/
    pub fn patch_user_webhooks_event_settings(
        &self,
        args: request::PatchUserWebhooksEventSettingsRequired,
    ) -> request::PatchUserWebhooksEventSettingsRequest {
        request::PatchUserWebhooksEventSettingsRequest {
            client: &self,
            on_behalf_of: None,
            bounce: args.bounce,
            click: args.click,
            deferred: args.deferred,
            delivered: args.delivered,
            dropped: args.dropped,
            enabled: args.enabled,
            group_resubscribe: args.group_resubscribe,
            group_unsubscribe: args.group_unsubscribe,
            oauth_client_id: None,
            oauth_client_secret: None,
            oauth_token_url: None,
            open: args.open,
            processed: args.processed,
            spam_report: args.spam_report,
            unsubscribe: args.unsubscribe,
            url: args.url.to_owned(),
        }
    }
    /**Retrieve Signed Webhook Public Key

**This endpoint allows you to retrieve your signed webhook's public key.**

Once you have enabled signing of the Event Webhook, you will need the public key provided to verify the signatures on requests coming from Twilio SendGrid. You can retrieve the public key from this endpoint at any time.

For more information about cryptographically signing the Event Webhook, see [Getting Started with the Event Webhook Security Features](https://sendgrid.com/docs/for-developers/tracking-events/getting-started-event-webhook-security-features).*/
    pub fn get_user_webhooks_event_settings_signed(
        &self,
    ) -> request::GetUserWebhooksEventSettingsSignedRequest {
        request::GetUserWebhooksEventSettingsSignedRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Enable/Disable Signed Webhook

**This endpoint allows you to enable or disable signing of the Event Webhook.**

This endpoint takes a single boolean request parameter, `enabled`. You may either enable or disable signing of the Event Webhook using this endpoint. Once enabled, you can retrieve your public key using the `/webhooks/event/settings/signed` endpoint.

For more information about cryptographically signing the Event Webhook, see [Getting Started with the Event Webhook Security Features](https://sendgrid.com/docs/for-developers/tracking-events/getting-started-event-webhook-security-features).*/
    pub fn patch_user_webhooks_event_settings_signed(
        &self,
        enabled: bool,
    ) -> request::PatchUserWebhooksEventSettingsSignedRequest {
        request::PatchUserWebhooksEventSettingsSignedRequest {
            client: &self,
            on_behalf_of: None,
            enabled,
        }
    }
    /**Test Event Notification Settings

**This endpoint allows you to test your event webhook by sending a fake event notification post to the provided URL.**

SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.

Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.

>**Tip**: Retry logic for this endpoint differs from other endpoints, which use a rolling 24-hour retry.

If your web server does not return a 2xx response type, we will retry a POST request until we receive a 2xx response or the maximum time of 10 minutes has expired.*/
    pub fn post_user_webhooks_event_test(
        &self,
    ) -> request::PostUserWebhooksEventTestRequest {
        request::PostUserWebhooksEventTestRequest {
            client: &self,
            on_behalf_of: None,
            oauth_client_id: None,
            oauth_client_secret: None,
            oauth_token_url: None,
            url: None,
        }
    }
    /**Retrieve all parse settings

**This endpoint allows you to retrieve all of your current inbound parse settings.***/
    pub fn get_user_webhooks_parse_settings(
        &self,
    ) -> request::GetUserWebhooksParseSettingsRequest {
        request::GetUserWebhooksParseSettingsRequest {
            client: &self,
            on_behalf_of: None,
        }
    }
    /**Create a parse setting

**This endpoint allows you to create a new inbound parse setting.**

Creating an Inbound Parse setting requires two pieces of information: a `url` and a `hostname`.

The `hostname` must correspond to a domain authenticated by Twilio SendGrid on your account. If you need to complete domain authentication, you can use the [Twilio SendGrid App](https://app.sendgrid.com/settings/sender_auth) or the "Authenticate a domain" endpoint. See "[How to Set Up Domain Authentication](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-domain-authentication/)" for instructions.

Any email received by the `hostname` will be parsed when you complete this setup. You must also add a Twilio SendGrid MX record to this domain's DNS records. See "[Setting up the Inbound Parse Webhook](https://sendgrid.com/docs/for-developers/parsing-email/setting-up-the-inbound-parse-webhook/)" for full instructions.

The `url` represents a location where the parsed message data will be delivered. Twilio SendGrid will make an HTTP POST request to this `url` with the message data. The `url` must be publicly reachable, and your application must return a `200` status code to signal that the message data has been received.*/
    pub fn post_user_webhooks_parse_settings(
        &self,
    ) -> request::PostUserWebhooksParseSettingsRequest {
        request::PostUserWebhooksParseSettingsRequest {
            client: &self,
            on_behalf_of: None,
            hostname: None,
            send_raw: None,
            spam_check: None,
            url: None,
        }
    }
    /**Retrieve a specific parse setting

**This endpoint allows you to retrieve a specific inbound parse setting by hostname.**

You can retrieve all your Inbound Parse settings and their associated host names with the "Retrieve all parse settings" endpoint.*/
    pub fn get_user_webhooks_parse_settings_hostname(
        &self,
        hostname: &str,
    ) -> request::GetUserWebhooksParseSettingsHostnameRequest {
        request::GetUserWebhooksParseSettingsHostnameRequest {
            client: &self,
            on_behalf_of: None,
            hostname: hostname.to_owned(),
        }
    }
    /**Delete a parse setting

**This endpoint allows you to delete a specific inbound parse setting by hostname.**

You can retrieve all your Inbound Parse settings and their associated host names with the "Retrieve all parse settings" endpoint.*/
    pub fn delete_user_webhooks_parse_settings_hostname(
        &self,
        hostname: &str,
    ) -> request::DeleteUserWebhooksParseSettingsHostnameRequest {
        request::DeleteUserWebhooksParseSettingsHostnameRequest {
            client: &self,
            on_behalf_of: None,
            hostname: hostname.to_owned(),
        }
    }
    /**Update a parse setting

**This endpoint allows you to update a specific inbound parse setting by hostname.**

You can retrieve all your Inbound Parse settings and their associated host names with the "Retrieve all parse settings" endpoint.*/
    pub fn patch_user_webhooks_parse_settings_hostname(
        &self,
        hostname: &str,
    ) -> request::PatchUserWebhooksParseSettingsHostnameRequest {
        request::PatchUserWebhooksParseSettingsHostnameRequest {
            client: &self,
            on_behalf_of: None,
            hostname: hostname.to_owned(),
            send_raw: None,
            spam_check: None,
            url: None,
        }
    }
    /**Retrieves Inbound Parse Webhook statistics.

**This endpoint allows you to retrieve the statistics for your Parse Webhook useage.**

SendGrid's Inbound Parse Webhook allows you to parse the contents and attachments of incomming emails. The Parse API can then POST the parsed emails to a URL that you specify. The Inbound Parse Webhook cannot parse messages greater than 30MB in size, including all attachments.

There are a number of pre-made integrations for the SendGrid Parse Webhook which make processing events easy. You can find these integrations in the [Library Index](https://sendgrid.com/docs/Integrate/libraries.html#-Webhook-Libraries).*/
    pub fn get_user_webhooks_parse_stats(
        &self,
        start_date: &str,
    ) -> request::GetUserWebhooksParseStatsRequest {
        request::GetUserWebhooksParseStatsRequest {
            client: &self,
            limit: None,
            offset: None,
            aggregated_by: None,
            start_date: start_date.to_owned(),
            end_date: None,
            on_behalf_of: None,
        }
    }
    /**Validate an email

**This endpoint allows you to validate an email address.***/
    pub fn post_validations_email(
        &self,
        email: &str,
    ) -> request::PostValidationsEmailRequest {
        request::PostValidationsEmailRequest {
            client: &self,
            email: email.to_owned(),
            source: None,
        }
    }
    /**Get All Verified Senders

**This endpoint allows you to retrieve all the Sender Identities associated with an account.**

This endpoint will return both verified and unverified senders.

You can limit the number of results returned using the `limit`, `lastSeenID`, and `id` query string parameters.

* `limit` allows you to specify an exact number of Sender Identities to return.
* `lastSeenID` will return senders with an ID number occuring after the passed in ID. In other words, the `lastSeenID` provides a starting point from which SendGrid will iterate to find Sender Identities associated with your account.
* `id` will return information about only the Sender Identity passed in the request.*/
    pub fn get_verified_senders(&self) -> request::GetVerifiedSendersRequest {
        request::GetVerifiedSendersRequest {
            client: &self,
            limit: None,
            last_seen_id: None,
            id: None,
        }
    }
    /**Create Verified Sender Request

**This endpoint allows you to create a new Sender Identify**.

Upon successful submission of a `POST` request to this endpoint, an identity will be created, and a verification email will be sent to the address assigned to the `from_email` field. You must complete the verification process using the sent email to fully verify the sender.

If you need to resend the verification email, you can do so with the Resend Verified Sender Request, `/resend/{id}`, endpoint.

If you need to authenticate a domain rather than a Single Sender, see the [Domain Authentication API](https://sendgrid.api-docs.io/v3.0/domain-authentication/authenticate-a-domain).*/
    pub fn post_verified_senders(
        &self,
        from_email: &str,
        nickname: &str,
        reply_to: &str,
    ) -> request::PostVerifiedSendersRequest {
        request::PostVerifiedSendersRequest {
            client: &self,
            address: None,
            address2: None,
            city: None,
            country: None,
            from_email: from_email.to_owned(),
            from_name: None,
            nickname: nickname.to_owned(),
            reply_to: reply_to.to_owned(),
            reply_to_name: None,
            state: None,
            zip: None,
        }
    }
    /**Domain Warn List

**This endpoint returns a list of domains known to implement DMARC and categorizes them by failure type — hard failure or soft failure**.

Domains listed as hard failures will not deliver mail when used as a [Sender Identity](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/) due to the domain's DMARC policy settings.

For example, using a `yahoo.com` email address as a Sender Identity will likely result in the rejection of your mail. For more information about DMARC, see [Everything about DMARC](https://sendgrid.com/docs/ui/sending-email/dmarc/).*/
    pub fn get_verified_senders_domains(
        &self,
    ) -> request::GetVerifiedSendersDomainsRequest {
        request::GetVerifiedSendersDomainsRequest {
            client: &self,
        }
    }
    /**Resend Verified Sender Request

**This endpoint allows you to resend a verification email to a specified Sender Identity**.

Passing the `id` assigned to a Sender Identity to this endpoint will resend a verification email to the `from_address` associated with the Sender Identity. This can be useful if someone loses their verification email or needs to have it resent for any other reason.

You can retrieve the IDs associated with Sender Identities by passing a "Get All Verified Senders" endpoint.*/
    pub fn post_verified_senders_resend_id(
        &self,
        id: &str,
    ) -> request::PostVerifiedSendersResendIdRequest {
        request::PostVerifiedSendersResendIdRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Completed Steps

**This endpoint allows you to determine which of SendGrid’s verification processes have been completed for an account**.

This endpoint returns boolean values, `true` and `false`, for [Domain Authentication](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/#domain-authentication), `domain_verified`, and [Single Sender Verification](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/#single-sender-verification), `sender_verified`, for the account.

An account may have one, both, or neither verification steps completed. If you need to authenticate a domain rather than a Single Sender, see the "Authenticate a domain" endpoint.*/
    pub fn get_verified_senders_steps_completed(
        &self,
    ) -> request::GetVerifiedSendersStepsCompletedRequest {
        request::GetVerifiedSendersStepsCompletedRequest {
            client: &self,
        }
    }
    /**Verify Sender Request

**This endpoint allows you to verify a sender requests.**

The token is generated by SendGrid and included in a verification email delivered to the address that's pending verification.*/
    pub fn get_verified_senders_verify_token(
        &self,
        token: &str,
    ) -> request::GetVerifiedSendersVerifyTokenRequest {
        request::GetVerifiedSendersVerifyTokenRequest {
            client: &self,
            token: token.to_owned(),
        }
    }
    /**Delete Verified Sender

**This endpoint allows you to delete a Sender Identity**.

Pass the `id` assigned to a Sender Identity to this endpoint to delete the Sender Identity from your account.

You can retrieve the IDs associated with Sender Identities using the "Get All Verified Senders" endpoint.*/
    pub fn delete_verified_senders_id(
        &self,
        id: &str,
    ) -> request::DeleteVerifiedSendersIdRequest {
        request::DeleteVerifiedSendersIdRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Edit Verified Sender

**This endpoint allows you to update an existing Sender Identity**.

Pass the `id` assigned to a Sender Identity to this endpoint as a path parameter. Include any fields you wish to update in the request body in JSON format.

You can retrieve the IDs associated with Sender Identities by passing a `GET` request to the Get All Verified Senders endpoint, `/verified_senders`.

**Note:** Unlike a `PUT` request, `PATCH` allows you to update only the fields you wish to edit. Fields that are not passed as part of a request will remain unaltered.*/
    pub fn patch_verified_senders_id(
        &self,
        args: request::PatchVerifiedSendersIdRequired,
    ) -> request::PatchVerifiedSendersIdRequest {
        request::PatchVerifiedSendersIdRequest {
            client: &self,
            id: args.id.to_owned(),
            address: None,
            address2: None,
            city: None,
            country: None,
            from_email: args.from_email.to_owned(),
            from_name: None,
            nickname: args.nickname.to_owned(),
            reply_to: args.reply_to.to_owned(),
            reply_to_name: None,
            state: None,
            zip: None,
        }
    }
    /**Email DNS records to a co-worker

**This endpoint is used to share DNS records with a colleagues**

Use this endpoint to send SendGrid-generated DNS record information to a co-worker so they can enter it into your DNS provider to validate your domain and link branding.

What type of records are sent will depend on whether you have chosen Automated Security or not. When using Automated Security, SendGrid provides you with three CNAME records. If you turn Automated Security off, you are instead given TXT and MX records.

If you pass a `link_id` to this endpoint, the generated email will supply the DNS records necessary to complete [Link Branding](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-link-branding/) setup. If you pass a `domain_id` to this endpoint, the generated email will supply the DNS records needed to complete [Domain Authentication](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-domain-authentication/). Passing both IDs will generate an email with the records needed to complete both setup steps.

You can retrieve all your domain IDs from the returned `id` fields for each domain using the "List all authenticated domains" endpoint. You can retrieve all of your link IDs using the "Retrieve all branded links" endpoint.*/
    pub fn post_whitelabel_dns_email(
        &self,
        domain_id: i64,
        email: &str,
        link_id: i64,
    ) -> request::PostWhitelabelDnsEmailRequest {
        request::PostWhitelabelDnsEmailRequest {
            client: &self,
            domain_id,
            email: email.to_owned(),
            link_id,
            message: None,
        }
    }
    /**List all authenticated domains

**This endpoint allows you to retrieve a list of all domains you have authenticated.***/
    pub fn get_whitelabel_domains(&self) -> request::GetWhitelabelDomainsRequest {
        request::GetWhitelabelDomainsRequest {
            client: &self,
            limit: None,
            offset: None,
            exclude_subusers: None,
            username: None,
            domain: None,
            on_behalf_of: None,
        }
    }
    /**Authenticate a domain

**This endpoint allows you to authenticate a domain.**

If you are authenticating a domain for a subuser, you have two options:
1. Use the "username" parameter. This allows you to authenticate a domain on behalf of your subuser. This means the subuser is able to see and modify the authenticated domain.
2. Use the Association workflow (see Associate Domain section). This allows you to authenticate a domain created by the parent to a subuser. This means the subuser will default to the assigned domain, but will not be able to see or modify that authenticated domain. However, if the subuser authenticates their own domain it will overwrite the assigned domain.*/
    pub fn post_whitelabel_domains(
        &self,
        domain: &str,
    ) -> request::PostWhitelabelDomainsRequest {
        request::PostWhitelabelDomainsRequest {
            client: &self,
            on_behalf_of: None,
            automatic_security: None,
            custom_dkim_selector: None,
            custom_spf: None,
            default: None,
            domain: domain.to_owned(),
            ips: None,
            subdomain: None,
            username: None,
        }
    }
    /**Get the default authentication

**This endpoint allows you to retrieve the default authentication for a domain.**

When creating or updating a domain authentication, you can set the domain as a default. The default domain will be used to send all mail. If you have multiple authenticated domains, the authenticated domain matching the domain of the From address will be used, and the default will be overridden.

This endpoint will return a default domain and its details only if a default is set. You are not required to set a default. If you do not set a default domain, this endpoint will return general information about your domain authentication status.*/
    pub fn get_whitelabel_domains_default(
        &self,
    ) -> request::GetWhitelabelDomainsDefaultRequest {
        request::GetWhitelabelDomainsDefaultRequest {
            client: &self,
            domain: None,
            on_behalf_of: None,
        }
    }
    /**List the authenticated domain associated with the given user.

**This endpoint allows you to retrieve all of the authenticated domains that have been assigned to a specific subuser.**

Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate an authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The parent may then associate the authenticated domain via the subuser management tools.*/
    pub fn get_whitelabel_domains_subuser(
        &self,
        username: &str,
    ) -> request::GetWhitelabelDomainsSubuserRequest {
        request::GetWhitelabelDomainsSubuserRequest {
            client: &self,
            username: username.to_owned(),
        }
    }
    /**Disassociate an authenticated domain from a given user.

**This endpoint allows you to disassociate a specific authenticated domain from a subuser.**

Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate an authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The parent may then associate the authenticated domain via the subuser management tools.*/
    pub fn delete_whitelabel_domains_subuser(
        &self,
    ) -> request::DeleteWhitelabelDomainsSubuserRequest {
        request::DeleteWhitelabelDomainsSubuserRequest {
            client: &self,
            username: None,
        }
    }
    /**Retrieve an authenticated domain

**This endpoint allows you to retrieve a specific authenticated domain.***/
    pub fn get_whitelabel_domains_domain_id(
        &self,
        domain_id: &str,
    ) -> request::GetWhitelabelDomainsDomainIdRequest {
        request::GetWhitelabelDomainsDomainIdRequest {
            client: &self,
            on_behalf_of: None,
            domain_id: domain_id.to_owned(),
        }
    }
    /**Delete an authenticated domain.

**This endpoint allows you to delete an authenticated domain.***/
    pub fn delete_whitelabel_domains_domain_id(
        &self,
        domain_id: &str,
    ) -> request::DeleteWhitelabelDomainsDomainIdRequest {
        request::DeleteWhitelabelDomainsDomainIdRequest {
            client: &self,
            on_behalf_of: None,
            domain_id: domain_id.to_owned(),
        }
    }
    /**Update an authenticated domain

**This endpoint allows you to update the settings for an authenticated domain.***/
    pub fn patch_whitelabel_domains_domain_id(
        &self,
        domain_id: &str,
    ) -> request::PatchWhitelabelDomainsDomainIdRequest {
        request::PatchWhitelabelDomainsDomainIdRequest {
            client: &self,
            on_behalf_of: None,
            domain_id: domain_id.to_owned(),
            custom_spf: None,
            default: None,
        }
    }
    /**Associate an authenticated domain with a given user.

**This endpoint allows you to associate a specific authenticated domain with a subuser.**

Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate an authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The parent may then associate the authenticated domain via the subuser management tools.*/
    pub fn post_whitelabel_domains_domain_id_subuser(
        &self,
        domain_id: i64,
        username: &str,
    ) -> request::PostWhitelabelDomainsDomainIdSubuserRequest {
        request::PostWhitelabelDomainsDomainIdSubuserRequest {
            client: &self,
            domain_id,
            username: username.to_owned(),
        }
    }
    /**Add an IP to an authenticated domain

**This endpoint allows you to add an IP address to an authenticated domain.***/
    pub fn post_whitelabel_domains_id_ips(
        &self,
        id: i64,
        ip: &str,
    ) -> request::PostWhitelabelDomainsIdIpsRequest {
        request::PostWhitelabelDomainsIdIpsRequest {
            client: &self,
            on_behalf_of: None,
            id,
            ip: ip.to_owned(),
        }
    }
    /**Remove an IP from an authenticated domain.

**This endpoint allows you to remove an IP address from that domain's authentication.***/
    pub fn delete_whitelabel_domains_id_ips_ip(
        &self,
        id: i64,
        ip: &str,
    ) -> request::DeleteWhitelabelDomainsIdIpsIpRequest {
        request::DeleteWhitelabelDomainsIdIpsIpRequest {
            client: &self,
            on_behalf_of: None,
            id,
            ip: ip.to_owned(),
        }
    }
    /**Validate a domain authentication.

**This endpoint allows you to validate an authenticated domain. If it fails, it will return an error message describing why the domain could not be validated.***/
    pub fn post_whitelabel_domains_id_validate(
        &self,
        id: i64,
    ) -> request::PostWhitelabelDomainsIdValidateRequest {
        request::PostWhitelabelDomainsIdValidateRequest {
            client: &self,
            on_behalf_of: None,
            id,
        }
    }
    /**Retrieve all reverse DNS records

**This endpoint allows you to retrieve all of the Reverse DNS records created by this account.**

You may include a search key by using the `ip` query string parameter. This enables you to perform a prefix search for a given IP segment (e.g., `?ip="192."`).

Use the `limit` query string parameter to reduce the number of records returned. All records will be returned if you have fewer records than the specified limit.

The `offset` query string parameter allows you to specify a non-zero index from which records will be returned. For example, if you have ten records, `?offset=5` will return the last five records (at indexes 5 through 9). The list starts at index zero.*/
    pub fn get_whitelabel_ips(&self) -> request::GetWhitelabelIpsRequest {
        request::GetWhitelabelIpsRequest {
            client: &self,
            limit: None,
            offset: None,
            ip: None,
            on_behalf_of: None,
        }
    }
    /**Set up reverse DNS

**This endpoint allows you to set up reverse DNS.***/
    pub fn post_whitelabel_ips(
        &self,
        domain: &str,
        ip: &str,
    ) -> request::PostWhitelabelIpsRequest {
        request::PostWhitelabelIpsRequest {
            client: &self,
            on_behalf_of: None,
            domain: domain.to_owned(),
            ip: ip.to_owned(),
            subdomain: None,
        }
    }
    /**Retrieve a reverse DNS record

**This endpoint allows you to retrieve a reverse DNS record.**

You can retrieve the IDs associated with all your reverse DNS records using the "Retrieve all reverse DNS records" endpoint.*/
    pub fn get_whitelabel_ips_id(&self, id: &str) -> request::GetWhitelabelIpsIdRequest {
        request::GetWhitelabelIpsIdRequest {
            client: &self,
            on_behalf_of: None,
            id: id.to_owned(),
        }
    }
    /**Delete a reverse DNS record

**This endpoint allows you to delete a reverse DNS record.**

A call to this endpoint will respond with a 204 status code if the deletion was successful.

You can retrieve the IDs associated with all your reverse DNS records using the "Retrieve all reverse DNS records" endpoint.*/
    pub fn delete_whitelabel_ips_id(
        &self,
        id: &str,
    ) -> request::DeleteWhitelabelIpsIdRequest {
        request::DeleteWhitelabelIpsIdRequest {
            client: &self,
            on_behalf_of: None,
            id: id.to_owned(),
        }
    }
    /**Validate a reverse DNS record

**This endpoint allows you to validate a reverse DNS record.**

Always check the `valid` property of the response’s `validation_results.a_record` object. This field will indicate whether it was possible to validate the reverse DNS record. If the `validation_results.a_record.valid` is `false`, this indicates only that Twilio SendGrid could not determine the validity your reverse DNS record — it may still be valid.

If validity couldn’t be determined, you can check the value of `validation_results.a_record.reason` to find out why.

You can retrieve the IDs associated with all your reverse DNS records using the "Retrieve all reverse DNS records" endpoint.*/
    pub fn post_whitelabel_ips_id_validate(
        &self,
        id: &str,
    ) -> request::PostWhitelabelIpsIdValidateRequest {
        request::PostWhitelabelIpsIdValidateRequest {
            client: &self,
            on_behalf_of: None,
            id: id.to_owned(),
        }
    }
    /**Retrieve all branded links

**This endpoint allows you to retrieve all branded links**.

You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.*/
    pub fn get_whitelabel_links(&self) -> request::GetWhitelabelLinksRequest {
        request::GetWhitelabelLinksRequest {
            client: &self,
            limit: None,
            on_behalf_of: None,
        }
    }
    /**Create a branded link

**This endpoint allows you to create a new branded link.**

To create the link branding, supply the root domain and, optionally, the subdomain — these go into separate fields in your request body. The root domain should match your FROM email address. If you provide a  subdomain, it must be different from the subdomain you used for authenticating your domain.

You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.*/
    pub fn post_whitelabel_links(
        &self,
        domain: &str,
    ) -> request::PostWhitelabelLinksRequest {
        request::PostWhitelabelLinksRequest {
            client: &self,
            on_behalf_of: None,
            default: None,
            domain: domain.to_owned(),
            subdomain: None,
        }
    }
    /**Retrieve the default branded link

**This endpoint allows you to retrieve the default branded link.**

The default branded link is the actual URL to be used when sending messages. If you have more than one branded link, the default is determined by the following order:

* The validated branded link marked as `default` (set when you call the "Create a branded link" endpoint or by calling the "Update a branded link" endpoint on an existing link)
* Legacy branded links (migrated from the whitelabel wizard)
* Default SendGrid-branded links (i.e., `100.ct.sendgrid.net`)

You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.*/
    pub fn get_whitelabel_links_default(
        &self,
    ) -> request::GetWhitelabelLinksDefaultRequest {
        request::GetWhitelabelLinksDefaultRequest {
            client: &self,
            domain: None,
            on_behalf_of: None,
        }
    }
    /**Retrieve a subuser's branded link

**This endpoint allows you to retrieve the branded link associated with a subuser.**

Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and then validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).*/
    pub fn get_whitelabel_links_subuser(
        &self,
        username: &str,
    ) -> request::GetWhitelabelLinksSubuserRequest {
        request::GetWhitelabelLinksSubuserRequest {
            client: &self,
            username: username.to_owned(),
        }
    }
    /**Disassociate a branded link from a subuser

**This endpoint allows you to take a branded link away from a subuser.**

Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).

Your request will receive a response with a 204 status code if the disassociation was successful.*/
    pub fn delete_whitelabel_links_subuser(
        &self,
        username: &str,
    ) -> request::DeleteWhitelabelLinksSubuserRequest {
        request::DeleteWhitelabelLinksSubuserRequest {
            client: &self,
            username: username.to_owned(),
        }
    }
    /**Retrieve a branded link

**This endpoint allows you to retrieve a specific branded link by providing its ID.**

You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.*/
    pub fn get_whitelabel_links_id(
        &self,
        id: i64,
    ) -> request::GetWhitelabelLinksIdRequest {
        request::GetWhitelabelLinksIdRequest {
            client: &self,
            on_behalf_of: None,
            id,
        }
    }
    /**Delete a branded link

**This endpoint allows you to delete a branded link.**

Your request will receive a response with a 204 status code if the deletion was successful. The call does not return the link's details, so if you wish to record these make sure you call the  "Retrieve a branded link" endpoint *before* you request its deletion.

You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.*/
    pub fn delete_whitelabel_links_id(
        &self,
        id: i64,
    ) -> request::DeleteWhitelabelLinksIdRequest {
        request::DeleteWhitelabelLinksIdRequest {
            client: &self,
            on_behalf_of: None,
            id,
        }
    }
    /**Update a branded link

**This endpoint allows you to update a specific branded link. You can use this endpoint to change a branded link's default status.**

You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.*/
    pub fn patch_whitelabel_links_id(
        &self,
        id: i64,
    ) -> request::PatchWhitelabelLinksIdRequest {
        request::PatchWhitelabelLinksIdRequest {
            client: &self,
            on_behalf_of: None,
            id,
            default: None,
        }
    }
    /**Validate a branded link

**This endpoint allows you to validate a branded link.**

You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.*/
    pub fn post_whitelabel_links_id_validate(
        &self,
        id: i64,
    ) -> request::PostWhitelabelLinksIdValidateRequest {
        request::PostWhitelabelLinksIdValidateRequest {
            client: &self,
            on_behalf_of: None,
            id,
        }
    }
    /**Associate a branded link with a subuser

**This endpoint allows you to associate a branded link with a subuser account.**

Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).*/
    pub fn post_whitelabel_links_link_id_subuser(
        &self,
        link_id: i64,
    ) -> request::PostWhitelabelLinksLinkIdSubuserRequest {
        request::PostWhitelabelLinksLinkIdSubuserRequest {
            client: &self,
            link_id,
            username: None,
        }
    }
}
pub enum SendgridAuthentication {
    Authorization { authorization: String },
}
impl SendgridAuthentication {
    pub fn from_env() -> Self {
        Self::Authorization {
            authorization: std::env::var("SENDGRID_AUTHORIZATION")
                .expect("Environment variable SENDGRID_AUTHORIZATION is not set."),
        }
    }
}
