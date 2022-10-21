use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailSettingsFooter {
    ///The custom HTML content of your email footer.
    pub html_content: Option<String>,
    ///Indicates if the Footer mail setting is currently enabled.
    pub enabled: Option<bool>,
    ///The plain text content of your email footer.
    pub plain_content: Option<String>,
}
impl std::fmt::Display for MailSettingsFooter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostMarketingSendersRequired {
    pub country: String,
    pub city: String,
    pub nickname: String,
    pub from: serde_json::Value,
    pub address: String,
}
impl std::fmt::Display for PostMarketingSendersRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactdbCustomFieldWithId {
    pub contactdb_custom_field: ContactdbCustomField,
    ///The ID of the custom field.
    pub id: f64,
}
impl std::fmt::Display for ContactdbCustomFieldWithId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IpPool {
    ///The name of the IP pool.
    pub name: String,
}
impl std::fmt::Display for IpPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SinglesendRequest {
    pub email_config: Option<serde_json::Value>,
    ///The name of the Single Send.
    pub name: String,
    ///The categories to associate with this Single Send.
    pub categories: Option<Vec<String>>,
    ///The ISO 8601 time at which to send the Single Send. This must be in future or the string "now". Emails can be scheduled up to 72 hours in advance. However, this scheduling constraint does not apply to campaigns sent via [Marketing Campaigns](https://docs.sendgrid.com/ui/sending-email/how-to-send-email-with-marketing-campaigns/).
    pub send_at: Option<String>,
    pub send_to: Option<serde_json::Value>,
}
impl std::fmt::Display for SinglesendRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbRecipientCount {
    ///The count of recipients.
    pub recipient_count: f64,
}
impl std::fmt::Display for ContactdbRecipientCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleContactRequest {
    pub contact: Option<serde_json::Value>,
    ///The contact's list IDs.
    pub list_ids: Option<Vec<String>>,
}
impl std::fmt::Display for SingleContactRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyNameIdScopes {
    pub api_key_name_id: ApiKeyNameId,
    ///The permissions this API Key has access to.
    pub scopes: Vec<String>,
}
impl std::fmt::Display for ApiKeyNameIdScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PartnerSettingsNewRelic {
    ///Indicates if your subuser statistics will be sent to your New Relic Dashboard.
    pub enable_subuser_statistics: Option<bool>,
    ///The license key provided with your New Relic account.
    pub license_key: String,
    ///Indicates if this setting is enabled.
    pub enabled: bool,
}
impl std::fmt::Display for PartnerSettingsNewRelic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FromEmailObject {
    ///The 'From' email address used to deliver the message. This address should be a verified sender in your Twilio SendGrid account.
    pub email: String,
    ///A name or title associated with the sending email address.
    pub name: Option<String>,
}
impl std::fmt::Display for FromEmailObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    ///The name you gave your list.
    pub name: Option<String>,
    ///The number of contacts currently stored on the list.
    pub contact_count: Option<i64>,
    ///The generated ID for your list.
    pub id: Option<String>,
    pub metadata: Option<Selfmetadata>,
}
impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IpAccessResponse {
    ///An array listing all of your allowed IPs.
    pub result: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for IpAccessResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IpWarmupResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldsById {}
impl std::fmt::Display for CustomFieldsById {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdvancedStatsClicksOpens {
    ///The individual events and their stats.
    pub advanced_stats_clicks: AdvancedStatsClicks,
    ///The individual events and their stats.
    pub advanced_stats_opens: AdvancedStatsOpens,
}
impl std::fmt::Display for AdvancedStatsClicksOpens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbList {
    ///The reference ID of your list.
    pub id: i64,
    ///The count of recipients currently in the list.
    pub recipient_count: i64,
    ///The name of your list. Must be unique against all other list and segment names.
    pub name: String,
}
impl std::fmt::Display for ContactdbList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClickTracking {
    ///Indicates if click tracking is enabled or disabled.
    pub enabled: bool,
    ///Indicates if click tracking is enabled for plain text emails.
    pub enable_text: bool,
}
impl std::fmt::Display for ClickTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Link {
    pub href: Option<String>,
    pub rel: Option<String>,
}
impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoIntegration {
    ///The URL where your IdP should POST its SAML response. This is the Twilio SendGrid URL that is responsible for receiving and parsing a SAML assertion. This is the same URL as the Single Sign-On URL when using SendGrid.
    pub audience_url: String,
    pub create_integration_request: CreateIntegrationRequest,
    ///The URL where your IdP should POST its SAML response. This is the Twilio SendGrid URL that is responsible for receiving and parsing a SAML assertion. This is the same URL as the Audience URL when using SendGrid.
    pub single_signon_url: String,
    ///A timestamp representing the last time the configuration was modified.
    pub last_updated: f64,
    ///A unique ID assigned to the configuration by SendGrid.
    pub id: String,
}
impl std::fmt::Display for SsoIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TneSenderId {
    ///The time the sender identity was last updated.
    pub updated_at: i64,
    ///The time the sender identity was created.
    pub created_at: i64,
    ///Only verified sender identities can be used to send email.
    pub verified: serde_json::Value,
    ///The unique identifier of the sender.
    pub id: i64,
    pub senders_id_request_body: SendersIdRequestBody,
    ///A sender identity is locked when it is associated with a campaign in the Draft, Scheduled, or In Progress state. You can't update or delete a locked sender identity.
    pub locked: bool,
}
impl std::fmt::Display for TneSenderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbSegments {
    ///The count of recipients in this list. This is not included on creation of segments.
    pub recipient_count: Option<f64>,
    ///The name of this segment.
    pub name: String,
    ///The list id from which to make this segment. Not including this ID will mean your segment is created from the main contactdb rather than a list.
    pub list_id: Option<i64>,
    ///The conditions for a recipient to be included in this segment.
    pub conditions: Vec<ContactdbSegmentsConditions>,
}
impl std::fmt::Display for ContactdbSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkBranding200Response {
    ///Indicates if this is the default link branding.
    pub default: bool,
    ///The DNS records generated for this link branding.
    pub dns: serde_json::Value,
    ///The root domain of the branded link.
    pub domain: String,
    ///The ID of the user that this link branding is associated with.
    pub user_id: i64,
    ///The subdomain used to generate the DNS records for this link branding. This subdomain must be different from the subdomain used for your authenticated domain.
    pub subdomain: Option<String>,
    ///The username of the account that this link branding is associated with.
    pub username: String,
    ///The ID of the branded link.
    pub id: i64,
    ///Indicates if this link branding was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create new link branding if you need to update it.
    pub legacy: bool,
    ///Indicates if this link branding is valid.
    pub valid: bool,
}
impl std::fmt::Display for LinkBranding200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoErrorResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoTeammatesPatchResponse {
    ///The Teammate’s street address.
    pub address: String,
    ///A website associated with the Teammate
    pub website: String,
    ///The Teammate’s stored phone number.
    pub phone: String,
    pub sso_teammate_response: SsoTeammateResponse,
    ///The Teammate’s company name.
    pub company: String,
    ///The Teammate’s zip code.
    pub zip: String,
    ///The Teammate's city.
    pub city: String,
    pub email: String,
    ///The Teammate’s country of residence.
    pub country: String,
    ///The Teammate’s apartment number, suite number, or other secondary address information that is not part of the physical street address.
    pub address2: String,
    ///The Teammate’s state or province.
    pub state: String,
    ///A Teammate can be an “admin,” “owner,” or “teammate.” Each role is associated with the scope of the Teammate’s permissions.
    pub user_type: String,
    ///The permission scopes assigned to the Teammate.
    pub scopes: Vec<String>,
}
impl std::fmt::Display for SsoTeammatesPatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactExport {
    ///The ISO8601 timestamp when the exported file on S3 will expire.
    pub expires_at: String,
    pub id: String,
    ///The ISO8601 timestamp when the export was begun.
    pub created_at: String,
    ///A human readable message if the status is `failure`.
    pub message: Option<String>,
    ///One or more download URLs for the contact file if the status is `ready`.
    pub urls: Option<Vec<String>>,
    ///The ISO8601 timestamp when the export was completed.
    pub completed_at: Option<String>,
    ///The ISO8601 timestamp when the export was updated.
    pub updated_at: String,
    pub metadata: Option<Metadata>,
    ///The total number of exported contacts.
    pub contact_count: Option<i64>,
    ///The export job's status. Allowed values: `pending`, `ready`, or `failure`.
    pub status: String,
}
impl std::fmt::Display for ContactExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReverseDns {
    ///A Unix epoch timestamp representing the last time of a validation attempt.
    pub last_validation_attempt_at: Option<i64>,
    ///The root, or sending, domain.
    pub domain: String,
    ///The ID of the Reverse DNS.
    pub id: i64,
    ///The IP address that this Reverse DNS was created for.
    pub ip: String,
    ///The subdomain created for this reverse DNS. This is where the rDNS record points.
    pub subdomain: Option<String>,
    ///The users who are able to send mail from the IP address.
    pub users: Vec<serde_json::Value>,
    ///Indicates if this is a valid Reverse DNS.
    pub valid: bool,
    pub a_record: serde_json::Value,
    ///The reverse DNS record for the IP address. This points to the Reverse DNS subdomain.
    pub rdns: String,
    ///Indicates if this Reverse DNS was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new Reverse DNS if you need to update it.
    pub legacy: bool,
}
impl std::fmt::Display for ReverseDns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DesignInput {
    pub design_duplicate_input: DesignDuplicateInput,
    pub design_common_fields: DesignCommonFields,
    ///Plain text content of the Design.
    pub plain_content: String,
    ///The HTML content of the Design.
    pub html_content: String,
}
impl std::fmt::Display for DesignInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchAsmGroupsGroupIdRequired {
    pub is_default: bool,
    pub group_id: String,
    pub description: String,
    pub name: String,
}
impl std::fmt::Display for PatchAsmGroupsGroupIdRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostMarketingSegmentsRequired {
    pub parent_list_id: String,
    pub parent_list_ids: Vec<String>,
    pub query_dsl: String,
    pub name: String,
}
impl std::fmt::Display for PostMarketingSegmentsRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserProfile {
    ///The street address for this user profile.
    pub address: Option<String>,
    ///That company that this user profile is associated with.
    pub company: Option<String>,
    ///The first name of the user.
    pub first_name: Option<String>,
    ///An optional second line for the street address of this user profile.
    pub address2: Option<String>,
    ///Th country of this user profile.
    pub country: Option<String>,
    ///The last name of the user.
    pub last_name: Option<String>,
    ///The website associated with this user.
    pub website: Option<String>,
    ///The city for the user profile.
    pub city: Option<String>,
    ///The phone number for the user.
    pub phone: Option<String>,
    ///The zip code for this user.
    pub zip: Option<String>,
    ///The state for this user.
    pub state: Option<String>,
}
impl std::fmt::Display for UserProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentUpdate {
    ///Name of the segment.
    pub name: Option<String>,
    ///SQL query which will filter contacts based on the conditions provided
    pub query_dsl: Option<String>,
}
impl std::fmt::Display for SegmentUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StatsAdvancedStatsBaseSchema(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Subuser {
    ///Whether or not the user is enabled or disabled.
    pub disabled: bool,
    ///The name by which this subuser will be referred.
    pub username: String,
    ///The email address to contact this subuser.
    pub email: String,
    ///The ID of this subuser.
    pub id: f64,
}
impl std::fmt::Display for Subuser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionalTemplateVersionCreate {
    ///The editor used in the UI.
    pub editor: Option<String>,
    ///For dynamic templates only, the mock json data that will be used for template preview and test sends.
    pub test_data: Option<String>,
    ///Subject of the new transactional template version.
    pub subject: String,
    ///Set the version as the active version associated with the template (0 is inactive, 1 is active). Only one version of a template can be active. The first version created for a template will automatically be set to Active.
    pub active: Option<i64>,
    ///The HTML content of the version. Maximum of 1048576 bytes allowed.
    pub html_content: Option<String>,
    ///Text/plain content of the transactional template version. Maximum of 1048576 bytes allowed.
    pub plain_content: Option<String>,
    ///Name of the transactional template version.
    pub name: String,
    ///If true, plain_content is always generated from html_content. If false, plain_content is not altered.
    pub generate_plain_content: Option<bool>,
}
impl std::fmt::Display for TransactionalTemplateVersionCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Webhook {
    ///The one time nonce to use when "signature" is "hmac-sha1"
    pub nonce: String,
    ///The URL to invoke in the webhook
    pub url: String,
}
impl std::fmt::Display for Webhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReservedFieldDefinitionsResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvalidEmail {
    ///A Unix timestamp indicating when the email address was added to the invalid emails list.
    pub created: Option<i64>,
    ///The email address that was marked as invalid.
    pub email: Option<String>,
    ///The reason that the email address was marked as invalid.
    pub reason: Option<String>,
}
impl std::fmt::Display for InvalidEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ToEmailArray(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DesignCommonFields {
    ///Subject of the Design.
    pub subject: String,
    ///The list of categories applied to the design
    pub categories: Vec<String>,
    pub design_duplicate_input: DesignDuplicateInput,
    ///If true, plain_content is always generated from html_content. If false, plain_content is not altered.
    pub generate_plain_content: bool,
}
impl std::fmt::Display for DesignCommonFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailSettingsAddressWhitelabel {
    ///All email addresses that are currently on the whitelist.
    pub list: Option<Vec<String>>,
    ///Indicates if you have an email address whitelist enabled.
    pub enabled: Option<bool>,
}
impl std::fmt::Display for MailSettingsAddressWhitelabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReplyToEmailObject {
    ///A name or title associated with the `reply_to` email address.
    pub name: Option<String>,
    ///The email address where any replies or bounces will be returned.
    pub email: String,
}
impl std::fmt::Display for ReplyToEmailObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchUserWebhooksEventSettingsRequired {
    pub bounce: bool,
    pub spam_report: bool,
    pub click: bool,
    pub enabled: bool,
    pub unsubscribe: bool,
    pub url: String,
    pub deferred: bool,
    pub processed: bool,
    pub dropped: bool,
    pub group_resubscribe: bool,
    pub delivered: bool,
    pub open: bool,
    pub group_unsubscribe: bool,
}
impl std::fmt::Display for PatchUserWebhooksEventSettingsRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentSummary {
    ///ISO8601 timestamp the object was last updated
    pub updated_at: String,
    /**ISO8601 of created timestamp
*/
    pub created_at: String,
    ///The id of the list if this segment is a child of a list.  This implies the query `AND CONTAINS(list_ids, ${parent_list_id})`
    pub parent_list_id: Option<String>,
    pub id: String,
    ///ISO8601 timestamp the sample was last updated
    pub sample_updated_at: String,
    ///ISO8601 string that is equal to `sample_updated_at` plus an internally calculated offset that depends on how often contacts enter or exit segments as the scheduled pipeline updates the samples.
    pub next_sample_update: Option<String>,
    pub contacts_count: i64,
    pub name: Option<String>,
}
impl std::fmt::Display for SegmentSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionalTemplateWarning {
    ///Warning message for the user
    pub message: Option<String>,
}
impl std::fmt::Display for TransactionalTemplateWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metrics {
    pub bounces: i64,
    pub unsubscribes: i64,
    pub clicks: i64,
    pub spam_report_drops: i64,
    pub invalid_emails: i64,
    pub requests: i64,
    pub unique_clicks: i64,
    pub spam_reports: i64,
    pub bounce_drops: i64,
    pub unique_opens: i64,
    pub opens: i64,
    pub delivered: i64,
}
impl std::fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AllSegmentsResponse {
    ///If not set, segment contains a query for use with Segment v1 APIs. If set to '2', segment contains a SQL query for use in v2.
    pub query_version: String,
    ///ISO8601 timestamp of when the object was created
    pub created_at: String,
    ///Segment status indicates whether the segment's contacts will be updated periodically
    pub status: SegmentStatusResponse,
    ///Total number of contacts present in the segment
    pub contacts_count: i64,
    pub metadata: Option<Metadata>,
    ///ISO8601 timestamp of when the samples will be next updated
    pub next_sample_update: String,
    ///ISO8601 timestamp of when the object was last updated
    pub updated_at: String,
    ///ISO8601 timestamp of when the samples were last updated
    pub sample_updated_at: String,
    ///Name of the segment.
    pub name: String,
    ///The array of list ids to filter contacts on when building this segment. It allows only one such list id for now. We will support more in future
    pub parent_list_ids: Vec<String>,
    ///ID assigned to the segment when created.
    pub id: String,
}
impl std::fmt::Display for AllSegmentsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Monitor {
    ///The frequency at which to forward monitoring emails. An email will be sent when your subuser sends this many (e.g., 1,000) emails.
    pub frequency: f64,
    ///The email address to which Sendgrid should send emails for monitoring.
    pub email: String,
}
impl std::fmt::Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiError {
    pub message: String,
    pub field: String,
    pub error_id: String,
}
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbRecipient {
    pub recipients: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for ContactdbRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VerifiedSenderResponseSchema {
    pub verified: Option<bool>,
    pub zip: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub from_name: Option<String>,
    pub reply_to: Option<String>,
    pub nickname: Option<String>,
    pub from_email: Option<String>,
    pub locked: Option<bool>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub reply_to_name: Option<String>,
    pub address: Option<String>,
    pub id: Option<i64>,
}
impl std::fmt::Display for VerifiedSenderResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationsLinkStatsResponse {
    pub total_clicks: i64,
    ///
    pub results: Vec<serde_json::Value>,
    pub metadata: LinkTrackingMetadata,
}
impl std::fmt::Display for AutomationsLinkStatsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GlobalErrorResponseSchema {
    pub id: Option<String>,
    pub errors: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for GlobalErrorResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiErrors {
    pub errors: Option<Vec<ApiError>>,
}
impl std::fmt::Display for ApiErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AbbvMessage {
    ///iso 8601 format
    pub last_event_time: String,
    pub from_email: String,
    pub msg_id: String,
    pub subject: String,
    pub to_email: String,
    pub opens_count: i64,
    pub status: String,
    pub clicks_count: i64,
}
impl std::fmt::Display for AbbvMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    ///The URL of the previous page of results. If this field isn't present, you're at the start of the list.
    pub prev: Option<String>,
    ///The number of items in the entire list, i.e., across all pages.
    pub count: Option<f64>,
    ///The URL of the current page of results.
    pub self_: Option<String>,
    ///The URL of the next page of results. If this field isn't present, you're at the end of the list.
    pub next: Option<String>,
}
impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldDefinitionsResponse {
    pub name: String,
    pub field_type: String,
    pub id: String,
}
impl std::fmt::Display for CustomFieldDefinitionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventWebhookUpdateOauthRequest {
    ///Message has been successfully delivered to the receiving server.
    pub delivered: bool,
    ///The client ID Twilio SendGrid sends to your OAuth server or service provider to generate an OAuth access token. When passing data in this field, you must also include the oauth_token_url field.
    pub oauth_client_id: Option<String>,
    ///Recipient clicked on message's subscription management link. You need to enable Subscription Tracking for getting this type of event.
    pub unsubscribe: bool,
    ///Message has been received and is ready to be delivered.
    pub processed: bool,
    ///Recipient clicked on a link within the message. You need to enable Click Tracking for getting this type of event.
    pub click: bool,
    ///Indicates if the event webhook is enabled.
    pub enabled: bool,
    ///This secret is needed only once to create an access token. SendGrid will store this secret, allowing you to update your Client ID and Token URL without passing the secret to SendGrid again.  When passing data in this field, you must also include the oauth_client_id and oauth_token_url fields.
    pub oauth_client_secret: Option<String>,
    ///Receiving server could not or would not accept message.
    pub bounce: bool,
    ///Recipient resubscribes to specific group by updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_resubscribe: bool,
    ///You may see the following drop reasons: Invalid SMTPAPI header, Spam Content (if spam checker app enabled), Unsubscribed Address, Bounced Address, Spam Reporting Address, Invalid, Recipient List over Package Quota
    pub dropped: bool,
    ///Recipient's email server temporarily rejected message.
    pub deferred: bool,
    ///Recipient marked a message as spam.
    pub spam_report: bool,
    ///Recipient unsubscribe from specific group, by either direct link or updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_unsubscribe: bool,
    ///The URL that you want the event webhook to POST to.
    pub url: String,
    ///Recipient has opened the HTML message. You need to enable Open Tracking for getting this type of event.
    pub open: bool,
    ///The URL where Twilio SendGrid sends the Client ID and Client Secret to generate an access token. This should be your OAuth server or service provider. When passing data in this field, you must also include the oauth_client_id field.
    pub oauth_token_url: Option<String>,
}
impl std::fmt::Display for EventWebhookUpdateOauthRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Selfmetadata {
    ///A link to this object.
    pub self_: Option<String>,
}
impl std::fmt::Display for Selfmetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationDomain {
    ///The IPs to be included in the custom SPF record for this authenticated domain.
    pub ips: Vec<String>,
    ///The domain to be authenticated.
    pub domain: String,
    ///The ID of the authenticated domain.
    pub id: f64,
    ///The subdomain to use for this authenticated domain.
    pub subdomain: String,
    ///Indicates whether this authenticated domain uses custom SPF.
    pub custom_spf: bool,
    ///Indicates if this authenticated domain was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new authenticated domain if you need to update it.
    pub legacy: bool,
    ///The username that this domain will be associated with.
    pub username: String,
    ///Indicates if this is the default authenticated domain.
    pub default: bool,
    ///Indicates if this authenticated domain uses automated security.
    pub automatic_security: bool,
    ///The DNS records used to authenticate the sending domain.
    pub dns: serde_json::Value,
    ///The ID of the user that this domain is associated with.
    pub user_id: f64,
    ///Indicates if this is a valid authenticated domain.
    pub valid: bool,
}
impl std::fmt::Display for AuthenticationDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailSettingsForwardSpam {
    ///The email address where you would like the spam reports to be forwarded.
    pub email: Option<String>,
    ///Indicates if the Forward Spam setting is enabled.
    pub enabled: Option<bool>,
}
impl std::fmt::Display for MailSettingsForwardSpam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SinglesendWarning {
    pub warnings: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for SinglesendWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostMailSendRequired {
    pub subject: String,
    pub content: Vec<serde_json::Value>,
    pub from: FromEmailObject,
    pub personalizations: Vec<serde_json::Value>,
}
impl std::fmt::Display for PostMailSendRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoTeammateResponse {
    ///This should be set to the Teammate's email address.
    pub username: String,
    pub sso_teammate_common_fields: SsoTeammateCommonFields,
    ///Indicates if the Teammate authenticates with SendGrid using SSO or with a username and password.
    pub is_sso: bool,
}
impl std::fmt::Display for SsoTeammateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AdvancedStatsOpens {
    ///The total number of times your emails were opened by recipients.
    pub opens: Option<i64>,
    ///The number of unique recipients who opened your emails.
    pub unique_opens: Option<i64>,
}
impl std::fmt::Display for AdvancedStatsOpens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationsResponse {
    pub metadata: Option<Metadata>,
    pub results: Vec<serde_json::Value>,
}
impl std::fmt::Display for AutomationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CampaignRequest {
    ///The pool of IPs that you would like to send this email from.
    pub ip_pool: Option<String>,
    ///The subject of your campaign that your recipients will see.
    pub subject: Option<String>,
    ///The categories you would like associated to this campaign.
    pub categories: Option<Vec<String>>,
    ///The plain text content of your emails.
    pub plain_content: Option<String>,
    ///This is the url of the custom unsubscribe page that you provide for customers to unsubscribe from your suppression groups.
    pub custom_unsubscribe_url: Option<String>,
    ///The ID of the "sender" identity that you have created. Your recipients will see this as the "from" on your marketing emails.
    pub sender_id: Option<i64>,
    ///The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs
    pub list_ids: Option<Vec<i64>>,
    ///The segment IDs that you are sending this list to. You can have both segment IDs and list IDs. Segments are limited to 10 segment IDs.
    pub segment_ids: Option<Vec<i64>>,
    ///The suppression group that this marketing email belongs to, allowing recipients to opt-out of emails of this type.
    pub suppression_group_id: Option<i64>,
    ///The display title of your campaign. This will be viewable by you in the Marketing Campaigns UI.
    pub title: String,
    ///The editor used in the UI.
    pub editor: Option<String>,
    ///The HTML of your marketing email.
    pub html_content: Option<String>,
}
impl std::fmt::Display for CampaignRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventWebhookResponse {
    ///The client ID Twilio SendGrid sends to your OAuth server or service provider to generate an OAuth access token.
    pub oauth_client_id: Option<String>,
    ///The URL that you want the event webhook to POST to.
    pub url: String,
    ///Recipient resubscribes to specific group by updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_resubscribe: bool,
    ///Message has been received and is ready to be delivered.
    pub processed: bool,
    ///Indicates if the event webhook is enabled.
    pub enabled: bool,
    ///Recipient has opened the HTML message. You need to enable Open Tracking for getting this type of event.
    pub open: bool,
    ///The URL where Twilio SendGrid sends the Client ID and Client Secret to generate an access token. This should be your OAuth server or service provider.
    pub oauth_token_url: Option<String>,
    ///Message has been successfully delivered to the receiving server.
    pub delivered: bool,
    ///You may see the following drop reasons: Invalid SMTPAPI header, Spam Content (if spam checker app enabled), Unsubscribed Address, Bounced Address, Spam Reporting Address, Invalid, Recipient List over Package Quota
    pub dropped: bool,
    ///Recipient clicked on message's subscription management link. You need to enable Subscription Tracking for getting this type of event.
    pub unsubscribe: bool,
    ///Receiving server could not or would not accept message.
    pub bounce: bool,
    ///Recipient's email server temporarily rejected message.
    pub deferred: bool,
    ///Recipient marked a message as spam.
    pub spam_report: bool,
    ///Recipient clicked on a link within the message. You need to enable Click Tracking for getting this type of event.
    pub click: bool,
    ///Recipient unsubscribe from specific group, by either direct link or updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_unsubscribe: bool,
}
impl std::fmt::Display for EventWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlocksResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SenderId {
    ///The time the sender identity was last updated.
    pub updated_at: i64,
    ///The unique identifier of the sender identity.
    pub id: i64,
    ///If the sender identity is verified or not. Only verified sender identities can be used to send email.
    pub verified: bool,
    pub sender_id_request: SenderIdRequest,
    ///The time the sender identity was created.
    pub created_at: i64,
    ///True when the sender id is associated to a campaign in the Draft, Scheduled, or In Progress status. You cannot update or delete a locked sender identity.
    pub locked: bool,
}
impl std::fmt::Display for SenderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostSsoTeammatesRequired {
    pub first_name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_read_only: bool,
    pub last_name: String,
    pub scopes: Vec<String>,
}
impl std::fmt::Display for PostSsoTeammatesRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubuserStats {
    ///The list of statistics.
    pub stats: Option<Vec<serde_json::Value>>,
    ///The date the statistics were gathered.
    pub date: Option<String>,
}
impl std::fmt::Display for SubuserStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VerifiedSenderRequestSchema {
    pub nickname: String,
    pub address2: Option<String>,
    pub reply_to_name: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub from_name: Option<String>,
    pub state: Option<String>,
    pub from_email: String,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub reply_to: String,
}
impl std::fmt::Display for VerifiedSenderRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchCampaignsCampaignIdRequired {
    pub campaign_id: i64,
    pub categories: Vec<String>,
    pub subject: String,
    pub html_content: String,
    pub title: String,
    pub plain_content: String,
}
impl std::fmt::Display for PatchCampaignsCampaignIdRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AbtestSummary {
    ///How the winner will be decided
    pub winner_criteria: String,
    ///Winner of the A/B Test
    pub winning_template_id: String,
    ///What percentage of your recipient will be included in your A/B testing
    pub test_percentage: i64,
    ///How long the A/B Testing will last
    pub duration: String,
    ///When the winner was selected
    pub winner_selected_at: Option<String>,
    ///Last day to select an A/B Test Winner
    pub expiration_date: Option<String>,
    ///What differs between the A/B tests
    pub type_: String,
}
impl std::fmt::Display for AbtestSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalEmptyRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SuppressionsRequest {
    ///The array of email addresses to add or find.
    pub recipient_emails: Vec<String>,
}
impl std::fmt::Display for SuppressionsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SendersIdRequestBody {
    ///The country of the sender identity.
    pub country: String,
    ///The physical address of the sender identity.
    pub address: String,
    ///The city of the sender identity.
    pub city: String,
    pub from: serde_json::Value,
    ///Additional sender identity address information.
    pub address2: Option<String>,
    ///The zipcode of the sender identity.
    pub zip: Option<String>,
    ///The state of the sender identity.
    pub state: Option<String>,
    ///A nickname for the sender identity. Not used for sending.
    pub nickname: String,
    pub reply_to: Option<serde_json::Value>,
}
impl std::fmt::Display for SendersIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailSettingsBouncePurge {
    ///Indicates if the bounce purge mail setting is enabled.
    pub enabled: Option<bool>,
    ///The number of days after which SendGrid will purge all contacts from your hard bounces suppression lists.
    pub hard_bounces: Option<i64>,
    ///The number of days after which SendGrid will purge all contacts from your soft bounces suppression lists.
    pub soft_bounces: Option<i64>,
}
impl std::fmt::Display for MailSettingsBouncePurge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailSettingsForwardBounce {
    ///The email address that you would like your bounce reports forwarded to.
    pub email: Option<String>,
    ///Indicates if the bounce forwarding mail setting is enabled.
    pub enabled: Option<bool>,
}
impl std::fmt::Display for MailSettingsForwardBounce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionalTemplatesVersionOutputLean {
    ///ID of the transactional template.
    pub template_id: Option<String>,
    ///Name of the transactional template version.
    pub name: Option<String>,
    ///The date and time that this transactional template version was updated.
    pub updated_at: Option<String>,
    ///Subject of the new transactional template version.
    pub subject: Option<String>,
    ///The editor used in the UI.
    pub editor: Option<String>,
    ///ID of the transactional template version.
    pub id: Option<String>,
    ///If true, plain_content is always generated from html_content. If false, plain_content is not altered.
    pub generate_plain_content: Option<bool>,
    ///Set the version as the active version associated with the template. Only one version of a template can be active. The first version created for a template will automatically be set to Active.
    pub active: Option<i64>,
    ///A Thumbnail preview of the template's html content.
    pub thumbnail_url: Option<String>,
}
impl std::fmt::Display for TransactionalTemplatesVersionOutputLean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionalTemplateVersionOutput {
    pub transactional_templates_version_output_lean: TransactionalTemplatesVersionOutputLean,
    pub warnings: Vec<TransactionalTemplateWarning>,
    pub transactional_template_version_create: TransactionalTemplateVersionCreate,
}
impl std::fmt::Display for TransactionalTemplateVersionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbCustomField {
    ///The name of the field
    pub name: Option<String>,
    ///The type of the field.
    pub type_: Option<String>,
}
impl std::fmt::Display for ContactdbCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IpPoolResponse {
    ///The name of the IP pool.
    pub name: Option<String>,
}
impl std::fmt::Display for IpPoolResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentResponse {
    ///ISO8601 timestamp of when the samples will be next updated
    pub next_sample_update: String,
    ///A subset of all contacts that are in this segment
    pub contacts_sample: Vec<ContactResponse>,
    ///Segment status indicates whether the segment's contacts will be updated periodically
    pub status: SegmentStatusResponse,
    ///ID assigned to the segment when created.
    pub id: String,
    ///Name of the segment.
    pub name: String,
    ///ISO8601 timestamp of when the object was last updated
    pub updated_at: String,
    ///ISO8601 timestamp of when the object was created
    pub created_at: String,
    ///If not set, segment contains a Query for use with Segment v1 APIs. If set to '2', segment contains a SQL query for use in v2.
    pub query_version: String,
    ///The array of list ids to filter contacts on when building this segment. It allows only one such list id for now. We will support more in future
    pub parent_list_ids: Vec<String>,
    ///ISO8601 timestamp of when the samples were last updated
    pub sample_updated_at: String,
    ///SQL query which will filter contacts based on the conditions provided
    pub query_dsl: String,
    ///Total number of contacts present in the segment
    pub contacts_count: i64,
}
impl std::fmt::Display for SegmentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SinglesendsLinkStatsResponse {
    pub metadata: LinkTrackingMetadata,
    ///This is the index of the link's location in the email contents.
    pub results: Vec<serde_json::Value>,
    pub total_clicks: Option<i64>,
}
impl std::fmt::Display for SinglesendsLinkStatsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostDesignRequired {
    pub plain_content: String,
    pub editor: String,
    pub categories: Vec<String>,
    pub name: String,
    pub generate_plain_content: bool,
    pub subject: String,
    pub html_content: String,
}
impl std::fmt::Display for PostDesignRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactDetails3 {
    pub line: Option<String>,
    pub address_line2: Option<String>,
    pub country: Option<String>,
    pub id: String,
    pub custom_fields: Option<serde_json::Value>,
    pub state_province_region: Option<String>,
    pub first_name: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: Option<String>,
    pub email: Option<String>,
    pub city: Option<String>,
    pub facebook: Option<String>,
    pub created_at: String,
    pub alternate_emails: Option<Vec<String>>,
    pub segment_ids: Vec<String>,
    pub list_ids: Vec<String>,
    pub address_line1: Option<String>,
    pub metadata: Option<Selfmetadata>,
    pub updated_at: String,
    pub whatsapp: Option<String>,
    pub last_name: Option<String>,
    pub unique_name: Option<String>,
}
impl std::fmt::Display for ContactDetails3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactdbCustomFieldWithIdValue {
    ///The value of this recipient's custom field
    pub value: Option<String>,
    pub contactdb_custom_field_with_id: ContactdbCustomFieldWithId,
}
impl std::fmt::Display for ContactdbCustomFieldWithIdValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostSsoIntegrationsRequired {
    pub enabled: bool,
    pub entity_id: String,
    pub signout_url: String,
    pub name: String,
    pub signin_url: String,
}
impl std::fmt::Display for PostSsoIntegrationsRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Errors {
    pub errors: Vec<serde_json::Value>,
}
impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebhooksEventWebhookRequest {
    ///Message has been successfully delivered to the receiving server.
    pub delivered: bool,
    ///Recipient unsubscribe from specific group, by either direct link or updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_unsubscribe: bool,
    ///Recipient resubscribes to specific group by updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_resubscribe: bool,
    ///Recipient's email server temporarily rejected message.
    pub deferred: bool,
    ///Recipient has opened the HTML message. You need to enable Open Tracking for getting this type of event.
    pub open: bool,
    ///Message has been received and is ready to be delivered.
    pub processed: bool,
    ///The client ID Twilio SendGrid sends to your OAuth server or service provider to generate an OAuth access token. When passing data in this field, you must also include the oauth_token_url field.
    pub oauth_client_id: Option<String>,
    ///Recipient clicked on message's subscription management link. You need to enable Subscription Tracking for getting this type of event.
    pub unsubscribe: bool,
    ///Receiving server could not or would not accept message.
    pub bounce: bool,
    ///Recipient marked a message as spam.
    pub spam_report: bool,
    ///The URL where Twilio SendGrid sends the Client ID and Client Secret to generate an access token. This should be your OAuth server or service provider. When passing data in this field, you must also include the oauth_client_id field.
    pub oauth_token_url: Option<String>,
    ///The URL that you want the event webhook to POST to.
    pub url: String,
    ///You may see the following drop reasons: Invalid SMTPAPI header, Spam Content (if spam checker app enabled), Unsubscribed Address, Bounced Address, Spam Reporting Address, Invalid, Recipient List over Package Quota
    pub dropped: bool,
    ///Recipient clicked on a link within the message. You need to enable Click Tracking for getting this type of event.
    pub click: bool,
    ///Indicates if the event webhook is enabled.
    pub enabled: bool,
}
impl std::fmt::Display for WebhooksEventWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchVerifiedSendersIdRequired {
    pub id: String,
    pub nickname: String,
    pub from_email: String,
    pub reply_to: String,
}
impl std::fmt::Display for PatchVerifiedSendersIdRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SinglesendsResponse {
    pub results: Vec<serde_json::Value>,
    pub metadata: Metadata,
}
impl std::fmt::Display for SinglesendsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CategoryStats {
    pub stats: Option<Vec<serde_json::Value>>,
    ///The date the statistics were gathered.
    pub date: String,
}
impl std::fmt::Display for CategoryStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SinglesendResponseShort {
    ///the ISO 8601 time at which the Single Send was created
    pub created_at: String,
    pub abtest: Option<AbtestSummary>,
    pub id: String,
    ///categories to associate with this Single Send
    pub categories: Vec<String>,
    ///name of the Single Send
    pub name: String,
    ///true if the Single Send's AB Test functionality has been toggled on
    pub is_abtest: bool,
    ///the ISO 8601 time at which the Single Send was last updated
    pub updated_at: String,
    ///The ISO 8601 time at which to send the Single Send. This must be in future or the string "now". Emails can be scheduled up to 72 hours in advance. However, this scheduling constraint does not apply to campaigns sent via [Marketing Campaigns](https://docs.sendgrid.com/ui/sending-email/how-to-send-email-with-marketing-campaigns/).
    pub send_at: Option<String>,
    ///current status of the Single Send
    pub status: String,
}
impl std::fmt::Display for SinglesendResponseShort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchSsoIntegrationsIdRequired {
    pub name: String,
    pub signin_url: String,
    pub id: String,
    pub signout_url: String,
    pub entity_id: String,
    pub enabled: bool,
}
impl std::fmt::Display for PatchSsoIntegrationsIdRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionTrackingSettings {
    ///The information in plain text for your unsubscribe link. You should have the “<% %>” tag in your content, otherwise the user will have no URL for unsubscribing.
    pub plain_content: Option<String>,
    ///The information and HTML for your unsubscribe link.
    pub html_content: Option<String>,
    ///Your custom defined replacement tag for your templates. Use this tag to place your unsubscribe content anywhere in your emailtemplate.
    pub replace: Option<String>,
    ///The HTML that will be displayed on the page that your customers will see after clicking unsubscribe, hosted on SendGrid’s server.
    pub landing: Option<String>,
    ///Indicates if subscription tracking is enabled.
    pub enabled: Option<bool>,
    ///The URL where you would like your users sent to unsubscribe.
    pub url: Option<String>,
}
impl std::fmt::Display for SubscriptionTrackingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbRecipientResponse {
    pub errors: Option<Vec<serde_json::Value>>,
    ///The number of errors found while adding recipients.
    pub error_count: f64,
    ///The indices of the recipient(s) sent that caused the error.
    pub error_indices: Option<Vec<f64>>,
    ///The count of new recipients added to the contactdb.
    pub new_count: f64,
    ///The recipient IDs of the recipients that already existed from this request.
    pub persisted_recipients: Vec<String>,
    ///The recipients who were updated from this request.
    pub updated_count: f64,
}
impl std::fmt::Display for ContactdbRecipientResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentWriteV2 {
    ///The array of list ids to filter contacts on when building this segment. It allows only one such list id for now. We will support more in future
    pub parent_list_ids: Option<Vec<String>>,
    ///SQL query which will filter contacts based on the conditions provided
    pub query_dsl: String,
    ///Name of the segment.
    pub name: String,
}
impl std::fmt::Display for SegmentWriteV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SenderIdRequest {
    ///The city of the sender identity.
    pub city: Option<String>,
    ///The country of the sender identity.
    pub country: Option<String>,
    ///Additional sender identity address information.
    pub address2: Option<String>,
    pub from: Option<serde_json::Value>,
    ///The state of the sender identity.
    pub state: Option<String>,
    ///The physical address of the sender identity.
    pub address: Option<String>,
    pub reply_to: Option<serde_json::Value>,
    ///The zipcode of the sender identity.
    pub zip: Option<String>,
    ///A nickname for the sender identity. Not used for sending.
    pub nickname: Option<String>,
}
impl std::fmt::Display for SenderIdRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DesignOutput {
    pub design_output_summary: DesignOutputSummary,
    pub design_input: DesignInput,
}
impl std::fmt::Display for DesignOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdvancedStatsMailboxProvider {
    ///The number of emails SendGrid was able to confirm were actually delivered to a recipient.
    pub delivered: i64,
    ///The number of recipients who marked your email as spam.
    pub spam_reports: i64,
    ///Requests from your website, application, or mail client via SMTP Relay or the Web API that SendGrid processed.
    pub processed: i64,
    ///The number of emails that were not allowed to be delivered by ISPs.
    pub blocks: i64,
    ///The number of emails that temporarily could not be delivered.
    pub deferred: i64,
    ///The number of emails that were requested to be delivered.
    pub requests: i64,
    ///The individual events and their stats.
    pub advanced_stats_clicks_opens: AdvancedStatsClicksOpens,
    ///The number of emails that bounced instead of being delivered.
    pub bounces: i64,
    ///The number of emails that were not delivered due to the recipient email address being on a suppression list.
    pub drops: i64,
}
impl std::fmt::Display for AdvancedStatsMailboxProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignResponse {
    pub campaign_request: CampaignRequest,
    ///The status of your campaign.
    pub status: String,
    pub id: i64,
}
impl std::fmt::Display for CampaignResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FullSegment {
    ///AST representation of the query DSL
    pub query_json: serde_json::Value,
    pub segment_summary: SegmentSummary,
    pub segment_write_v2: SegmentWriteV2,
    pub contacts_sample: Vec<ContactResponse>,
}
impl std::fmt::Display for FullSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentQueryJson {
    pub contacts: Option<serde_json::Value>,
}
impl std::fmt::Display for SegmentQueryJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostSubusersRequired {
    pub password: String,
    pub username: String,
    pub email: String,
    pub ips: Vec<String>,
}
impl std::fmt::Display for PostSubusersRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    ///List of events related to email message
    pub events: Vec<serde_json::Value>,
    pub from_email: String,
    pub template_id: String,
    ///JSON hash of arbitrary key-value pairs
    pub unique_args: String,
    pub subject: String,
    ///This is the IP of the user who sent the message.
    pub originating_ip: String,
    ///Teammate's username
    pub teammate: String,
    ///Quick summary of the status of a message
    pub status: String,
    pub api_key_id: String,
    ///IP used to send to the remote MTA. Used by UI to display IP in detailed view
    pub outbound_ip: String,
    ///Whether or not the outbound IP is dedicated vs shared
    pub outbound_ip_type: String,
    pub to_email: String,
    pub asm_group_id: i64,
    ///Categories users associated to the message
    pub categories: Vec<String>,
    pub msg_id: String,
}
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbSegmentsConditions {
    pub and_or: Option<String>,
    pub field: String,
    pub value: String,
    pub operator: String,
}
impl std::fmt::Display for ContactdbSegmentsConditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub permissions: Option<serde_json::Value>,
    pub username: Option<String>,
}
impl std::fmt::Display for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentStatusResponse {
    ///Status of query validation. PENDING, VALID, or INVALID
    pub query_validation: String,
    ///Describes any errors that were encountered during query validation
    pub error_message: Option<String>,
}
impl std::fmt::Display for SegmentStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalId(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactDetails2 {
    pub postal_code: Option<String>,
    pub email: Option<String>,
    pub facebook: Option<String>,
    pub first_name: Option<String>,
    pub unique_name: Option<String>,
    pub list_ids: Vec<String>,
    pub segment_ids: Option<Vec<String>>,
    pub address_line2: Option<String>,
    pub country: Option<String>,
    pub id: String,
    pub whatsapp: Option<String>,
    pub address_line1: Option<String>,
    pub created_at: String,
    pub line: Option<String>,
    pub custom_fields: Option<serde_json::Value>,
    pub phone_number: Option<String>,
    pub alternate_emails: Option<Vec<String>>,
    pub last_name: Option<String>,
    pub metadata: Option<Selfmetadata>,
    pub state_province_region: Option<String>,
    pub updated_at: String,
    pub city: Option<String>,
}
impl std::fmt::Display for ContactDetails2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorsSegV2 {
    pub errors: Vec<serde_json::Value>,
}
impl std::fmt::Display for ErrorsSegV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubuserPost {
    ///The email address for this subuser.
    pub email: String,
    pub authorization_token: Option<String>,
    pub credit_allocation: Option<serde_json::Value>,
    ///The user ID for this subuser.
    pub user_id: f64,
    ///The username of the subuser.
    pub username: String,
    pub signup_session_token: Option<String>,
}
impl std::fmt::Display for SubuserPost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SinglesendSearch {
    ///current status of the Single Send
    pub status: Option<Vec<String>>,
    ///leading and trailing wildcard search on name of the Single Send
    pub name: Option<String>,
    ///categories to associate with this Single Send, match any single send that has at least one of the categories
    pub categories: Option<Vec<String>>,
}
impl std::fmt::Display for SinglesendSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactImport {
    ///The ISO8601 timestamp when the job was finished.
    pub finished_at: Option<String>,
    ///The job ID.
    pub id: Option<String>,
    ///The ISO8601 timestamp when the job was created.
    pub started_at: Option<String>,
    ///The job state. Allowed values: `pending`, `completed`, `errored`, or `failed`.
    pub status: Option<String>,
    ///The job type. Allowed values: `upsert`, or `delete`.
    pub job_type: Option<String>,
    ///Result map of the import job.
    pub results: Option<serde_json::Value>,
}
impl std::fmt::Display for ContactImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionalTemplate {
    pub warning: TransactionalTemplateWarning,
    pub transactional_templates_template_lean: TransactionalTemplatesTemplateLean,
}
impl std::fmt::Display for TransactionalTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactSummary {
    pub first_name: Option<String>,
    ///Primary email address.
    pub email: Option<String>,
    pub last_name: Option<String>,
    ///Unix Epoch Timestamp.
    pub updated_at: f64,
    pub metadata: Option<Selfmetadata>,
    ///List UUID linked with this contact.
    pub list_ids: Vec<String>,
    ///Contact UUID.
    pub id: String,
    ///Unix Epoch Timestamp.
    pub created_at: f64,
}
impl std::fmt::Display for ContactSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailSettingsTemplate {
    ///Indicates if the legacy email template setting is enabled.
    pub enabled: Option<bool>,
    ///The HTML content that you want to use for your legacy email template.
    pub html_content: Option<String>,
}
impl std::fmt::Display for MailSettingsTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailSettingsPatch {
    ///The email address of the recipient.
    pub email: Option<String>,
    ///Indicates if the mail setting is enabled.
    pub enabled: Option<bool>,
}
impl std::fmt::Display for MailSettingsPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ParseSetting {
    ///Indicates if you would like SendGrid to post the original MIME-type content of your parsed email. When this parameter is set to `true`, SendGrid will send a JSON payload of the content of your email.
    pub send_raw: Option<bool>,
    ///Indicates if you would like SendGrid to check the content parsed from your emails for spam before POSTing them to your domain.
    pub spam_check: Option<bool>,
    ///The public URL where you would like SendGrid to POST the data parsed from your email. Any emails sent with the given hostname provided (whose MX records have been updated to point to SendGrid) will be parsed and POSTed to this URL.
    pub url: Option<String>,
    ///A specific and unique domain or subdomain that you have created to use exclusively to parse your incoming email. For example, `parse.yourdomain.com`.
    pub hostname: Option<String>,
}
impl std::fmt::Display for ParseSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error {
    pub field: Option<String>,
    pub error_id: Option<String>,
    pub message: String,
    pub parameter: Option<String>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmailActivityResponseCommonFields {
    ///The message's status.
    pub status: Option<String>,
    ///The intended recipient's email address.
    pub to_email: Option<String>,
    ///A unique ID assigned to the message. This ID can be used to retrieve activity data for the specific message.
    pub msg_id: Option<String>,
    ///The 'From' email address used to deliver the message. This address should be a verified sender in your Twilio SendGrid account.
    pub from_email: Option<String>,
    ///The email's subject line.
    pub subject: Option<String>,
}
impl std::fmt::Display for EmailActivityResponseCommonFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentWrite {
    ///Use this field for adding your query string.
    pub query_dsl: String,
    pub name: String,
}
impl std::fmt::Display for SegmentWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SinglesendSchedule {
    pub status: Option<String>,
    ///The ISO 8601 time at which to send the Single Send. This must be in future or the string "now". Emails can be scheduled up to 72 hours in advance. However, this scheduling constraint does not apply to campaigns sent via [Marketing Campaigns](https://docs.sendgrid.com/ui/sending-email/how-to-send-email-with-marketing-campaigns/).
    pub send_at: String,
}
impl std::fmt::Display for SinglesendSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AdvancedStatsClicks {
    ///The number of unique recipients who clicked links in your emails.
    pub unique_clicks: Option<i64>,
    ///The number of links that were clicked in your emails.
    pub clicks: Option<i64>,
}
impl std::fmt::Display for AdvancedStatsClicks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactResponse {
    ///Email of the contact. This is a reserved field.
    pub email: String,
    ///Second line of address of the contact. This is a reserved field.
    pub address_line2: String,
    ///ID assigned to a contact when added to the system.
    pub id: String,
    ///IDs of all segments the contact is part of
    pub segment_ids: Option<Vec<String>>,
    ///First line of address of the contact. This is a reserved field.
    pub address_line1: String,
    ///The user may choose to create up to 120 custom fields or none at all. This is not a reserved field.
    pub custom_fields: serde_json::Value,
    ///Country associated with the address of the contact. This is a reserved field.
    pub country: String,
    ///First name of the contact. This is a reserved field.
    pub first_name: String,
    ///IDs of all lists the contact is part of
    pub list_ids: Option<Vec<String>>,
    ///Alternate emails of the contact. This is a reserved field.
    pub alternate_emails: Vec<String>,
    ///State associated with the contact. This is a reserved field.
    pub state_province_region: String,
    ///City associated with the contact. This is a reserved field.
    pub city: String,
    ///Zipcode associated with the address of the contact. This is a reserved field.
    pub postal_code: i64,
    ///Last name of the contact. This is a reserved field.
    pub last_name: String,
}
impl std::fmt::Display for ContactResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DesignDuplicateInput {
    ///The editor used in the UI.
    pub editor: Option<String>,
    ///The name of the new design.
    pub name: Option<String>,
}
impl std::fmt::Display for DesignDuplicateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionalTemplatesTemplateLean {
    ///The different versions of this transactional template.
    pub versions: Option<Vec<TransactionalTemplatesVersionOutputLean>>,
    ///The date and time that this transactional template version was updated.
    pub updated_at: String,
    ///Defines the generation of the template.
    pub generation: String,
    ///The ID of the transactional template.
    pub id: String,
    ///The name for the transactional template.
    pub name: String,
}
impl std::fmt::Display for TransactionalTemplatesTemplateLean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserScheduledSendStatus {
    pub mail_batch_id: MailBatchId,
    ///The status of the scheduled send.
    pub status: String,
}
impl std::fmt::Display for UserScheduledSendStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostSendersRequired {
    pub nickname: String,
    pub address: String,
    pub country: String,
    pub from: serde_json::Value,
    pub state: String,
    pub city: String,
    pub reply_to: serde_json::Value,
    pub zip: String,
    pub address2: String,
}
impl std::fmt::Display for PostSendersRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PatchTemplatesTemplateIdVersionsVersionIdRequired {
    pub name: String,
    pub template_id: String,
    pub version_id: String,
    pub subject: String,
}
impl std::fmt::Display for PatchTemplatesTemplateIdVersionsVersionIdRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SsoCertificateBody {
    ///A unique ID assigned to the certificate by SendGrid.
    pub id: Option<f64>,
    ///A unix timestamp (e.g., 1603915954) that indicates the time before which the certificate is not valid.
    pub not_before: Option<f64>,
    ///A unix timestamp (e.g., 1603915954) that indicates the time after which the certificate is no longer valid.
    pub not_after: Option<f64>,
    ///This certificate is used by Twilio SendGrid to verify that SAML requests are coming from Okta. This is called the X509 certificate in the Twilio SendGrid UI.
    pub public_certificate: Option<String>,
    ///An ID that matches a certificate to a specific IdP integration.
    pub intergration_id: Option<String>,
}
impl std::fmt::Display for SsoCertificateBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CcBccEmailObject {
    ///The intended recipient's email address.
    pub email: String,
    ///The intended recipient's name.
    pub name: Option<String>,
}
impl std::fmt::Display for CcBccEmailObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactdbSegmentsWithId {
    pub contactdb_segments: ContactdbSegments,
    ///The ID of the segment.
    pub id: f64,
}
impl std::fmt::Display for ContactdbSegmentsWithId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequest {
    pub custom_fields: Option<CustomFieldsById>,
    ///The contact's personal name.
    pub first_name: Option<String>,
    ///The contact's family name.
    pub last_name: Option<String>,
    ///The contact's ZIP code or other postal code.
    pub postal_code: Option<String>,
    ///The contact's primary email. This is required to be a valid email.
    pub email: String,
    ///The contact's city.
    pub city: Option<String>,
    ///The contact's state, province, or region.
    pub state_province_region: Option<String>,
    ///The contact's country. Can be a full name or an abbreviation.
    pub country: Option<String>,
    ///The first line of the address.
    pub address_line1: Option<String>,
    ///An optional second line for the address.
    pub address_line2: Option<String>,
    ///Additional emails associated with the contact.
    pub alternate_emails: Option<Vec<String>>,
}
impl std::fmt::Display for ContactRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SpamReportsResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SsoTeammateCommonFields {
    ///The Teammate’s first name.
    pub first_name: String,
    ///The Teammate’s email address. This email address will also function as the Teammate’s username and must match the address assigned to the user in your IdP. This address cannot be changed after the Teammate is created.
    pub email: String,
    ///The Teammate’s last name.
    pub last_name: String,
    ///Indicates if the Teammate has admin permissions.
    pub is_admin: Option<bool>,
    ///Indicates if the Teammate has read_only permissions.
    pub is_read_only: Option<bool>,
}
impl std::fmt::Display for SsoTeammateCommonFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SuppressionGroup {
    ///Indicates if this is the default suppression group.
    pub is_default: Option<bool>,
    pub last_email_sent_at: Option<i64>,
    ///The unsubscribes associated with this group.
    pub unsubscribes: Option<i64>,
    ///A description of the suppression group.
    pub description: String,
    ///The name of the suppression group. Each group created by a user must have a unique name.
    pub name: String,
    ///The id of the suppression group.
    pub id: f64,
}
impl std::fmt::Display for SuppressionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Contacts {
    pub address: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub address2: Option<serde_json::Value>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
}
impl std::fmt::Display for Contacts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactDetails {
    pub metadata: Option<Selfmetadata>,
    ///The ISO8601 timestamp when the contact was updated.
    pub updated_at: String,
    ///The ISO8601 timestamp when the contact was created.
    pub created_at: String,
    pub custom_fields: Option<CustomFieldsByName>,
    pub state_province_region: Option<String>,
    pub alternate_emails: Option<Vec<String>>,
    pub email: Option<String>,
    pub address_line2: Option<String>,
    pub country: Option<String>,
    pub first_name: Option<String>,
    pub address_line1: Option<String>,
    pub last_name: Option<String>,
    pub city: Option<String>,
    pub id: String,
    pub list_ids: Vec<String>,
    pub postal_code: Option<String>,
}
impl std::fmt::Display for ContactDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiKeyNameId {
    ///The name of your API Key.
    pub name: Option<String>,
    ///The ID of your API Key.
    pub api_key_id: Option<String>,
}
impl std::fmt::Display for ApiKeyNameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoTeammateRequest {
    pub sso_teammate_common_fields: SsoTeammateCommonFields,
    ///The permission scopes assigned to the Teammate.
    pub scopes: Vec<String>,
}
impl std::fmt::Display for SsoTeammateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GoogleAnalyticsSettings {
    ///Name of the referrer source.
    pub utm_source: Option<String>,
    ///Used to differentiate ads
    pub utm_content: Option<String>,
    ///Indicates if Google Analytics is enabled.
    pub enabled: Option<bool>,
    ///The name of the campaign.
    pub utm_campaign: Option<String>,
    ///Name of the marketing medium (e.g. "Email").
    pub utm_medium: Option<String>,
    ///Any paid keywords.
    pub utm_term: Option<String>,
}
impl std::fmt::Display for GoogleAnalyticsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainAuthentication200Response(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct StatsAdvancedGlobalStats {
    ///The number of emails that temporarily could not be delivered.
    pub deferred: i64,
    ///The number of emails that were dropped due to a recipient previously marking your emails as spam.
    pub spam_report_drops: i64,
    ///The number of emails dropped due to a recipient unsubscribing from your emails.
    pub unsubscribe_drops: i64,
    ///The number of emails that were not allowed to be delivered by ISPs.
    pub blocks: i64,
    ///The number of emails that bounced instead of being delivered.
    pub bounces: i64,
    ///The number of emails that were dropped because of a bounce.
    pub bounce_drops: i64,
    ///The number of recipients who unsubscribed from your emails.
    pub unsubscribes: i64,
    ///The number of recipients who had malformed email addresses or whose mail provider reported the address as invalid.
    pub invalid_emails: i64,
    ///The number of emails that were requested to be delivered.
    pub requests: i64,
    ///The number of recipients who marked your email as spam.
    pub spam_reports: i64,
    ///Requests from your website, application, or mail client via SMTP Relay or the API that SendGrid processed.
    pub processed: i64,
    ///The individual events and their stats.
    pub advanced_stats_clicks_opens: AdvancedStatsClicksOpens,
    ///The number of emails SendGrid was able to confirm were actually delivered to a recipient.
    pub delivered: i64,
}
impl std::fmt::Display for StatsAdvancedGlobalStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SuppressionGroupRequestBase {
    ///A brief description of your suppression group. Required when creating a group.
    pub description: Option<String>,
    ///Indicates if you would like this to be your default suppression group.
    pub is_default: Option<bool>,
    ///The name of your suppression group. Required when creating a group.
    pub name: Option<String>,
}
impl std::fmt::Display for SuppressionGroupRequestBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldsByName {}
impl std::fmt::Display for CustomFieldsByName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DesignOutputSummary {
    ///Datetime that Design was created.
    pub created_at: String,
    pub design_common_fields: DesignCommonFields,
    ///ID of the Design.
    pub id: String,
    ///Datetime that Design was last updated.
    pub updated_at: String,
    ///A Thumbnail preview of the template's html content.
    pub thumbnail_url: String,
    pub design_duplicate_input: DesignDuplicateInput,
}
impl std::fmt::Display for DesignOutputSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EnforcedTlsRequestResponse {
    ///Indicates if you want to require your recipients to have a valid certificate.
    pub require_valid_cert: Option<bool>,
    ///The minimum required TLS certificate version.
    pub version: Option<f64>,
    ///Indicates if you want to require your recipients to support TLS.
    pub require_tls: Option<bool>,
}
impl std::fmt::Display for EnforcedTlsRequestResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentSummaryV2 {
    pub results: Option<Vec<SegmentSummary>>,
}
impl std::fmt::Display for SegmentSummaryV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LinkTrackingMetadata {
    ///The URL of the previous page of results. If this field isn't present, you're at the start of the list.
    pub prev: Option<String>,
    ///The number of items in the entire list, i.e., across all pages.
    pub count: Option<f64>,
    ///The URL of the current page of results.
    pub self_: Option<String>,
    ///The URL of the next page of results. If this field isn't present, you're at the end of the list.
    pub next: Option<String>,
}
impl std::fmt::Display for LinkTrackingMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MakoEvent {
    ///Use to distinguish between types of bounces
    pub bounce_type: String,
    ///Used with "deferred" events to indicate the attempt number out of 10. One "deferred" entry will exists under events array for each time a message was deferred with error message from the server.
    pub attempt_num: Option<i64>,
    ///For example mx.gmail.com
    pub mx_server: String,
    ///Date of when event occurred
    pub processed: String,
    ///Client recipient used to click or open message
    pub http_user_agent: String,
    ///Explanation of what caused "bounced", "deferred", or "blocked". Usually contains error message from the server - e.g. message from gmail why mail was deferred
    pub reason: Option<String>,
    ///Name of event
    pub event_name: String,
    ///Used with "clicked" event to indicate which url the user clicked.
    pub url: String,
}
impl std::fmt::Display for MakoEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainAuthenticationDomainSpf {
    ///The user_id of the account that this authenticated domain is associated with.
    pub user_id: i64,
    ///Indicates if this authenticated domain uses custom SPF.
    pub custom_spf: bool,
    ///The domain authenticated.
    pub domain: String,
    ///The username of the account that this authenticated domain is associated with.
    pub username: String,
    ///The IP addresses that are included in the SPF record for this authenticated domain.
    pub ips: Vec<serde_json::Value>,
    ///The subdomain that was used to create this authenticated domain.
    pub subdomain: Option<String>,
    ///The DNS records for this authenticated domain.
    pub dns: serde_json::Value,
    ///Indicates if this authenticated domain uses automated security.
    pub automatic_security: bool,
    ///Indicates if this is a valid authenticated domain .
    pub valid: bool,
    ///Indicates if this is the default domain.
    pub default: bool,
    ///The ID of the authenticated domain.
    pub id: i64,
    ///Indicates if this authenticated domain was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new authenticated domain if you need to update it.
    pub legacy: bool,
}
impl std::fmt::Display for DomainAuthenticationDomainSpf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BounceResponse {
    ///Enhanced SMTP bounce response
    pub status: Option<String>,
    ///The email address that was added to the bounce list.
    pub email: Option<String>,
    ///The unix timestamp for when the bounce record was created at SendGrid.
    pub created: Option<f64>,
    ///The reason for the bounce. This typically will be a bounce code, an enhanced code, and a description.
    pub reason: Option<String>,
}
impl std::fmt::Display for BounceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SinglesendResponse {
    ///the ISO 8601 time at which the Single Send was created
    pub created_at: String,
    ///current status of the Single Send
    pub status: String,
    ///the ISO 8601 time at which the Single Send was last updated
    pub updated_at: String,
    pub warnings: Vec<serde_json::Value>,
    pub id: String,
    pub singlesend_request: SinglesendRequest,
}
impl std::fmt::Display for SinglesendResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailBatchId {
    pub batch_id: String,
}
impl std::fmt::Display for MailBatchId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateIntegrationRequest {
    ///An identifier provided by your IdP to identify Twilio SendGrid in the SAML interaction. This is called the "SAML Issuer ID" in the Twilio SendGrid UI.
    pub entity_id: String,
    ///The IdP's SAML POST endpoint. This endpoint should receive requests and initiate an SSO login flow. This is called the "Embed Link" in the Twilio SendGrid UI.
    pub signin_url: String,
    ///Indicates if the integration is enabled.
    pub enabled: bool,
    ///The name of your integration. This name can be anything that makes sense for your organization (eg. Twilio SendGrid)
    pub name: String,
    ///This URL is relevant only for an IdP-initiated authentication flow. If a user authenticates from their IdP, this URL will return them to their IdP when logging out.
    pub signout_url: String,
    ///Indicates if the integration is complete.
    pub completed_integration: Option<bool>,
}
impl std::fmt::Display for CreateIntegrationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
