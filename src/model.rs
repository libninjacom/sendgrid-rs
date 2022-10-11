use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TneSenderId(pub serde_json::Value, SendersIdRequestBody, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AbbvMessage {
    pub clicks_count: i64,
    pub from_email: String,
    ///iso 8601 format
    pub last_event_time: String,
    pub msg_id: String,
    pub opens_count: i64,
    pub status: String,
    pub subject: String,
    pub to_email: String,
}
impl std::fmt::Display for AbbvMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AbtestSummary {
    ///How long the A/B Testing will last
    pub duration: String,
    ///Last day to select an A/B Test Winner
    pub expiration_date: Option<String>,
    ///What percentage of your recipient will be included in your A/B testing
    pub test_percentage: i64,
    #[serde(rename = "type")]
    ///What differs between the A/B tests
    pub type_: String,
    ///How the winner will be decided
    pub winner_criteria: String,
    ///When the winner was selected
    pub winner_selected_at: Option<String>,
    ///Winner of the A/B Test
    pub winning_template_id: String,
}
impl std::fmt::Display for AbtestSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AdvancedStatsClicks {
    ///The number of links that were clicked in your emails.
    pub clicks: Option<i64>,
    ///The number of unique recipients who clicked links in your emails.
    pub unique_clicks: Option<i64>,
}
impl std::fmt::Display for AdvancedStatsClicks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdvancedStatsClicksOpens(pub AdvancedStatsClicks, AdvancedStatsOpens);
#[derive(Debug, Serialize, Deserialize)]
pub struct AdvancedStatsMailboxProvider(pub AdvancedStatsClicksOpens, serde_json::Value);
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
pub struct AllSegmentsResponse {
    #[serde(rename = "_metadata")]
    pub metadata: Option<Metadata>,
    ///Total number of contacts present in the segment
    pub contacts_count: i64,
    ///ISO8601 timestamp of when the object was created
    pub created_at: String,
    ///ID assigned to the segment when created.
    pub id: String,
    ///Name of the segment.
    pub name: String,
    ///ISO8601 timestamp of when the samples will be next updated
    pub next_sample_update: String,
    ///The array of list ids to filter contacts on when building this segment. It allows only one such list id for now. We will support more in future
    pub parent_list_ids: Vec<String>,
    ///If not set, segment contains a query for use with Segment v1 APIs. If set to '2', segment contains a SQL query for use in v2.
    pub query_version: String,
    ///ISO8601 timestamp of when the samples were last updated
    pub sample_updated_at: String,
    ///Segment status indicates whether the segment's contacts will be updated periodically
    pub status: SegmentStatusResponse,
    ///ISO8601 timestamp of when the object was last updated
    pub updated_at: String,
}
impl std::fmt::Display for AllSegmentsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiError {
    pub error_id: String,
    pub field: String,
    pub message: String,
}
impl std::fmt::Display for ApiError {
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
pub struct ApiKeyNameId {
    ///The ID of your API Key.
    pub api_key_id: Option<String>,
    ///The name of your API Key.
    pub name: Option<String>,
}
impl std::fmt::Display for ApiKeyNameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyNameIdScopes(pub serde_json::Value, ApiKeyNameId);
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationDomain {
    ///Indicates if this authenticated domain uses automated security.
    pub automatic_security: bool,
    ///Indicates whether this authenticated domain uses custom SPF.
    pub custom_spf: bool,
    ///Indicates if this is the default authenticated domain.
    pub default: bool,
    ///The DNS records used to authenticate the sending domain.
    pub dns: serde_json::Value,
    ///The domain to be authenticated.
    pub domain: String,
    ///The ID of the authenticated domain.
    pub id: f64,
    ///The IPs to be included in the custom SPF record for this authenticated domain.
    pub ips: Vec<String>,
    ///Indicates if this authenticated domain was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new authenticated domain if you need to update it.
    pub legacy: bool,
    ///The subdomain to use for this authenticated domain.
    pub subdomain: String,
    ///The ID of the user that this domain is associated with.
    pub user_id: f64,
    ///The username that this domain will be associated with.
    pub username: String,
    ///Indicates if this is a valid authenticated domain.
    pub valid: bool,
}
impl std::fmt::Display for AuthenticationDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationsLinkStatsResponse {
    #[serde(rename = "_metadata")]
    pub metadata: LinkTrackingMetadata,
    ///
    pub results: Vec<serde_json::Value>,
    pub total_clicks: i64,
}
impl std::fmt::Display for AutomationsLinkStatsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationsResponse {
    #[serde(rename = "_metadata")]
    pub metadata: Option<Metadata>,
    pub results: Vec<serde_json::Value>,
}
impl std::fmt::Display for AutomationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlocksResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BounceResponse {
    ///The unix timestamp for when the bounce record was created at SendGrid.
    pub created: Option<f64>,
    ///The email address that was added to the bounce list.
    pub email: Option<String>,
    ///The reason for the bounce. This typically will be a bounce code, an enhanced code, and a description.
    pub reason: Option<String>,
    ///Enhanced SMTP bounce response
    pub status: Option<String>,
}
impl std::fmt::Display for BounceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CampaignRequest {
    ///The categories you would like associated to this campaign.
    pub categories: Option<Vec<String>>,
    ///This is the url of the custom unsubscribe page that you provide for customers to unsubscribe from your suppression groups.
    pub custom_unsubscribe_url: Option<String>,
    ///The editor used in the UI.
    pub editor: Option<String>,
    ///The HTML of your marketing email.
    pub html_content: Option<String>,
    ///The pool of IPs that you would like to send this email from.
    pub ip_pool: Option<String>,
    ///The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs
    pub list_ids: Option<Vec<i64>>,
    ///The plain text content of your emails.
    pub plain_content: Option<String>,
    ///The segment IDs that you are sending this list to. You can have both segment IDs and list IDs. Segments are limited to 10 segment IDs.
    pub segment_ids: Option<Vec<i64>>,
    ///The ID of the "sender" identity that you have created. Your recipients will see this as the "from" on your marketing emails.
    pub sender_id: Option<i64>,
    ///The subject of your campaign that your recipients will see.
    pub subject: Option<String>,
    ///The suppression group that this marketing email belongs to, allowing recipients to opt-out of emails of this type.
    pub suppression_group_id: Option<i64>,
    ///The display title of your campaign. This will be viewable by you in the Marketing Campaigns UI.
    pub title: String,
}
impl std::fmt::Display for CampaignRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignResponse(pub CampaignRequest, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CategoryStats {
    ///The date the statistics were gathered.
    pub date: String,
    pub stats: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CategoryStats {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClickTracking {
    ///Indicates if click tracking is enabled for plain text emails.
    pub enable_text: bool,
    ///Indicates if click tracking is enabled or disabled.
    pub enabled: bool,
}
impl std::fmt::Display for ClickTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactDetails {
    #[serde(rename = "_metadata")]
    pub metadata: Option<Selfmetadata>,
    #[serde(rename = "address_line_1")]
    pub address_line1: Option<String>,
    #[serde(rename = "address_line_2")]
    pub address_line2: Option<String>,
    pub alternate_emails: Option<Vec<String>>,
    pub city: Option<String>,
    pub country: Option<String>,
    ///The ISO8601 timestamp when the contact was created.
    pub created_at: String,
    pub custom_fields: Option<CustomFieldsByName>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub id: String,
    pub last_name: Option<String>,
    pub list_ids: Vec<String>,
    pub postal_code: Option<String>,
    pub state_province_region: Option<String>,
    ///The ISO8601 timestamp when the contact was updated.
    pub updated_at: String,
}
impl std::fmt::Display for ContactDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactDetails2 {
    #[serde(rename = "_metadata")]
    pub metadata: Option<Selfmetadata>,
    #[serde(rename = "address_line_1")]
    pub address_line1: Option<String>,
    #[serde(rename = "address_line_2")]
    pub address_line2: Option<String>,
    pub alternate_emails: Option<Vec<String>>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub created_at: String,
    pub custom_fields: Option<serde_json::Value>,
    pub email: Option<String>,
    pub facebook: Option<String>,
    pub first_name: Option<String>,
    pub id: String,
    pub last_name: Option<String>,
    pub line: Option<String>,
    pub list_ids: Vec<String>,
    pub phone_number: Option<String>,
    pub postal_code: Option<String>,
    pub segment_ids: Option<Vec<String>>,
    pub state_province_region: Option<String>,
    pub unique_name: Option<String>,
    pub updated_at: String,
    pub whatsapp: Option<String>,
}
impl std::fmt::Display for ContactDetails2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactDetails3 {
    #[serde(rename = "_metadata")]
    pub metadata: Option<Selfmetadata>,
    #[serde(rename = "address_line_1")]
    pub address_line1: Option<String>,
    #[serde(rename = "address_line_2")]
    pub address_line2: Option<String>,
    pub alternate_emails: Option<Vec<String>>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub created_at: String,
    pub custom_fields: Option<serde_json::Value>,
    pub email: Option<String>,
    pub facebook: Option<String>,
    pub first_name: Option<String>,
    pub id: String,
    pub last_name: Option<String>,
    pub line: Option<String>,
    pub list_ids: Vec<String>,
    pub phone_number: Option<String>,
    pub postal_code: Option<String>,
    pub segment_ids: Vec<String>,
    pub state_province_region: Option<String>,
    pub unique_name: Option<String>,
    pub updated_at: String,
    pub whatsapp: Option<String>,
}
impl std::fmt::Display for ContactDetails3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactExport {
    #[serde(rename = "_metadata")]
    pub metadata: Option<Metadata>,
    ///The ISO8601 timestamp when the export was completed.
    pub completed_at: Option<String>,
    ///The total number of exported contacts.
    pub contact_count: Option<i64>,
    ///The ISO8601 timestamp when the export was begun.
    pub created_at: String,
    ///The ISO8601 timestamp when the exported file on S3 will expire.
    pub expires_at: String,
    pub id: String,
    ///A human readable message if the status is `failure`.
    pub message: Option<String>,
    ///The export job's status. Allowed values: `pending`, `ready`, or `failure`.
    pub status: String,
    ///The ISO8601 timestamp when the export was updated.
    pub updated_at: String,
    ///One or more download URLs for the contact file if the status is `ready`.
    pub urls: Option<Vec<String>>,
}
impl std::fmt::Display for ContactExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactImport {
    ///The ISO8601 timestamp when the job was finished.
    pub finished_at: Option<String>,
    ///The job ID.
    pub id: Option<String>,
    ///The job type. Allowed values: `upsert`, or `delete`.
    pub job_type: Option<String>,
    ///Result map of the import job.
    pub results: Option<serde_json::Value>,
    ///The ISO8601 timestamp when the job was created.
    pub started_at: Option<String>,
    ///The job state. Allowed values: `pending`, `completed`, `errored`, or `failed`.
    pub status: Option<String>,
}
impl std::fmt::Display for ContactImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactRequest {
    #[serde(rename = "address_line_1")]
    ///The first line of the address.
    pub address_line1: Option<String>,
    #[serde(rename = "address_line_2")]
    ///An optional second line for the address.
    pub address_line2: Option<String>,
    ///Additional emails associated with the contact.
    pub alternate_emails: Option<Vec<String>>,
    ///The contact's city.
    pub city: Option<String>,
    ///The contact's country. Can be a full name or an abbreviation.
    pub country: Option<String>,
    pub custom_fields: Option<CustomFieldsById>,
    ///The contact's primary email. This is required to be a valid email.
    pub email: String,
    ///The contact's personal name.
    pub first_name: Option<String>,
    ///The contact's family name.
    pub last_name: Option<String>,
    ///The contact's ZIP code or other postal code.
    pub postal_code: Option<String>,
    ///The contact's state, province, or region.
    pub state_province_region: Option<String>,
}
impl std::fmt::Display for ContactRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactSummary {
    #[serde(rename = "_metadata")]
    pub metadata: Option<Selfmetadata>,
    ///Unix Epoch Timestamp.
    pub created_at: f64,
    ///Primary email address.
    pub email: Option<String>,
    pub first_name: Option<String>,
    ///Contact UUID.
    pub id: String,
    pub last_name: Option<String>,
    ///List UUID linked with this contact.
    pub list_ids: Vec<String>,
    ///Unix Epoch Timestamp.
    pub updated_at: f64,
}
impl std::fmt::Display for ContactSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactResponse {
    #[serde(rename = "address_line_1")]
    ///First line of address of the contact. This is a reserved field.
    pub address_line1: String,
    #[serde(rename = "address_line_2")]
    ///Second line of address of the contact. This is a reserved field.
    pub address_line2: String,
    ///Alternate emails of the contact. This is a reserved field.
    pub alternate_emails: Vec<String>,
    ///City associated with the contact. This is a reserved field.
    pub city: String,
    ///Country associated with the address of the contact. This is a reserved field.
    pub country: String,
    ///The user may choose to create up to 120 custom fields or none at all. This is not a reserved field.
    pub custom_fields: serde_json::Value,
    ///Email of the contact. This is a reserved field.
    pub email: String,
    ///First name of the contact. This is a reserved field.
    pub first_name: String,
    ///ID assigned to a contact when added to the system.
    pub id: String,
    ///Last name of the contact. This is a reserved field.
    pub last_name: String,
    ///IDs of all lists the contact is part of
    pub list_ids: Option<Vec<String>>,
    ///Zipcode associated with the address of the contact. This is a reserved field.
    pub postal_code: i64,
    ///IDs of all segments the contact is part of
    pub segment_ids: Option<Vec<String>>,
    ///State associated with the contact. This is a reserved field.
    pub state_province_region: String,
}
impl std::fmt::Display for ContactResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbCustomField {
    ///The name of the field
    pub name: Option<String>,
    #[serde(rename = "type")]
    ///The type of the field.
    pub type_: Option<String>,
}
impl std::fmt::Display for ContactdbCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactdbCustomFieldWithId(pub ContactdbCustomField, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactdbCustomFieldWithIdValue(
    pub ContactdbCustomFieldWithId,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbList {
    ///The reference ID of your list.
    pub id: i64,
    ///The name of your list. Must be unique against all other list and segment names.
    pub name: String,
    ///The count of recipients currently in the list.
    pub recipient_count: i64,
}
impl std::fmt::Display for ContactdbList {
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
pub struct ContactdbRecipientCount {
    ///The count of recipients.
    pub recipient_count: f64,
}
impl std::fmt::Display for ContactdbRecipientCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbRecipientResponse {
    ///The number of errors found while adding recipients.
    pub error_count: f64,
    ///The indices of the recipient(s) sent that caused the error.
    pub error_indices: Option<Vec<f64>>,
    pub errors: Option<Vec<serde_json::Value>>,
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
pub struct ContactdbSegments {
    ///The conditions for a recipient to be included in this segment.
    pub conditions: Vec<ContactdbSegmentsConditions>,
    ///The list id from which to make this segment. Not including this ID will mean your segment is created from the main contactdb rather than a list.
    pub list_id: Option<i64>,
    ///The name of this segment.
    pub name: String,
    ///The count of recipients in this list. This is not included on creation of segments.
    pub recipient_count: Option<f64>,
}
impl std::fmt::Display for ContactdbSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContactdbSegmentsConditions {
    pub and_or: Option<String>,
    pub field: String,
    pub operator: String,
    pub value: String,
}
impl std::fmt::Display for ContactdbSegmentsConditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactdbSegmentsWithId(pub serde_json::Value, ContactdbSegments);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Contacts {
    pub address: Option<String>,
    pub address2: Option<serde_json::Value>,
    pub city: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl std::fmt::Display for Contacts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateIntegrationRequest {
    ///Indicates if the integration is complete.
    pub completed_integration: Option<bool>,
    ///Indicates if the integration is enabled.
    pub enabled: bool,
    ///An identifier provided by your IdP to identify Twilio SendGrid in the SAML interaction. This is called the "SAML Issuer ID" in the Twilio SendGrid UI.
    pub entity_id: String,
    ///The name of your integration. This name can be anything that makes sense for your organization (eg. Twilio SendGrid)
    pub name: String,
    ///The IdP's SAML POST endpoint. This endpoint should receive requests and initiate an SSO login flow. This is called the "Embed Link" in the Twilio SendGrid UI.
    pub signin_url: String,
    ///This URL is relevant only for an IdP-initiated authentication flow. If a user authenticates from their IdP, this URL will return them to their IdP when logging out.
    pub signout_url: String,
}
impl std::fmt::Display for CreateIntegrationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
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
pub struct CustomFieldsById {}
impl std::fmt::Display for CustomFieldsById {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldDefinitionsResponse {
    pub field_type: String,
    pub id: String,
    pub name: String,
}
impl std::fmt::Display for CustomFieldDefinitionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DesignCommonFields(pub DesignDuplicateInput, serde_json::Value);
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
#[derive(Debug, Serialize, Deserialize)]
pub struct DesignInput(pub DesignDuplicateInput, DesignCommonFields, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DesignOutput(pub DesignOutputSummary, DesignInput);
#[derive(Debug, Serialize, Deserialize)]
pub struct DesignOutputSummary(
    pub serde_json::Value,
    DesignDuplicateInput,
    DesignCommonFields,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainAuthentication200Response(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainAuthenticationDomainSpf {
    ///Indicates if this authenticated domain uses automated security.
    pub automatic_security: bool,
    ///Indicates if this authenticated domain uses custom SPF.
    pub custom_spf: bool,
    ///Indicates if this is the default domain.
    pub default: bool,
    ///The DNS records for this authenticated domain.
    pub dns: serde_json::Value,
    ///The domain authenticated.
    pub domain: String,
    ///The ID of the authenticated domain.
    pub id: i64,
    ///The IP addresses that are included in the SPF record for this authenticated domain.
    pub ips: Vec<serde_json::Value>,
    ///Indicates if this authenticated domain was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new authenticated domain if you need to update it.
    pub legacy: bool,
    ///The subdomain that was used to create this authenticated domain.
    pub subdomain: Option<String>,
    ///The user_id of the account that this authenticated domain is associated with.
    pub user_id: i64,
    ///The username of the account that this authenticated domain is associated with.
    pub username: String,
    ///Indicates if this is a valid authenticated domain .
    pub valid: bool,
}
impl std::fmt::Display for DomainAuthenticationDomainSpf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmailActivityResponseCommonFields {
    ///The 'From' email address used to deliver the message. This address should be a verified sender in your Twilio SendGrid account.
    pub from_email: Option<String>,
    ///A unique ID assigned to the message. This ID can be used to retrieve activity data for the specific message.
    pub msg_id: Option<String>,
    ///The message's status.
    pub status: Option<String>,
    ///The email's subject line.
    pub subject: Option<String>,
    ///The intended recipient's email address.
    pub to_email: Option<String>,
}
impl std::fmt::Display for EmailActivityResponseCommonFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EnforcedTlsRequestResponse {
    ///Indicates if you want to require your recipients to support TLS.
    pub require_tls: Option<bool>,
    ///Indicates if you want to require your recipients to have a valid certificate.
    pub require_valid_cert: Option<bool>,
    ///The minimum required TLS certificate version.
    pub version: Option<f64>,
}
impl std::fmt::Display for EnforcedTlsRequestResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error {
    pub error_id: Option<String>,
    pub field: Option<String>,
    pub message: String,
    pub parameter: Option<String>,
}
impl std::fmt::Display for Error {
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
pub struct ErrorsSegV2 {
    pub errors: Vec<serde_json::Value>,
}
impl std::fmt::Display for ErrorsSegV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventWebhookResponse {
    ///Receiving server could not or would not accept message.
    pub bounce: bool,
    ///Recipient clicked on a link within the message. You need to enable Click Tracking for getting this type of event.
    pub click: bool,
    ///Recipient's email server temporarily rejected message.
    pub deferred: bool,
    ///Message has been successfully delivered to the receiving server.
    pub delivered: bool,
    ///You may see the following drop reasons: Invalid SMTPAPI header, Spam Content (if spam checker app enabled), Unsubscribed Address, Bounced Address, Spam Reporting Address, Invalid, Recipient List over Package Quota
    pub dropped: bool,
    ///Indicates if the event webhook is enabled.
    pub enabled: bool,
    ///Recipient resubscribes to specific group by updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_resubscribe: bool,
    ///Recipient unsubscribe from specific group, by either direct link or updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_unsubscribe: bool,
    ///The client ID Twilio SendGrid sends to your OAuth server or service provider to generate an OAuth access token.
    pub oauth_client_id: Option<String>,
    ///The URL where Twilio SendGrid sends the Client ID and Client Secret to generate an access token. This should be your OAuth server or service provider.
    pub oauth_token_url: Option<String>,
    ///Recipient has opened the HTML message. You need to enable Open Tracking for getting this type of event.
    pub open: bool,
    ///Message has been received and is ready to be delivered.
    pub processed: bool,
    ///Recipient marked a message as spam.
    pub spam_report: bool,
    ///Recipient clicked on message's subscription management link. You need to enable Subscription Tracking for getting this type of event.
    pub unsubscribe: bool,
    ///The URL that you want the event webhook to POST to.
    pub url: String,
}
impl std::fmt::Display for EventWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventWebhookUpdateOauthRequest {
    ///Receiving server could not or would not accept message.
    pub bounce: bool,
    ///Recipient clicked on a link within the message. You need to enable Click Tracking for getting this type of event.
    pub click: bool,
    ///Recipient's email server temporarily rejected message.
    pub deferred: bool,
    ///Message has been successfully delivered to the receiving server.
    pub delivered: bool,
    ///You may see the following drop reasons: Invalid SMTPAPI header, Spam Content (if spam checker app enabled), Unsubscribed Address, Bounced Address, Spam Reporting Address, Invalid, Recipient List over Package Quota
    pub dropped: bool,
    ///Indicates if the event webhook is enabled.
    pub enabled: bool,
    ///Recipient resubscribes to specific group by updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_resubscribe: bool,
    ///Recipient unsubscribe from specific group, by either direct link or updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_unsubscribe: bool,
    ///The client ID Twilio SendGrid sends to your OAuth server or service provider to generate an OAuth access token. When passing data in this field, you must also include the oauth_token_url field.
    pub oauth_client_id: Option<String>,
    ///This secret is needed only once to create an access token. SendGrid will store this secret, allowing you to update your Client ID and Token URL without passing the secret to SendGrid again.  When passing data in this field, you must also include the oauth_client_id and oauth_token_url fields.
    pub oauth_client_secret: Option<String>,
    ///The URL where Twilio SendGrid sends the Client ID and Client Secret to generate an access token. This should be your OAuth server or service provider. When passing data in this field, you must also include the oauth_client_id field.
    pub oauth_token_url: Option<String>,
    ///Recipient has opened the HTML message. You need to enable Open Tracking for getting this type of event.
    pub open: bool,
    ///Message has been received and is ready to be delivered.
    pub processed: bool,
    ///Recipient marked a message as spam.
    pub spam_report: bool,
    ///Recipient clicked on message's subscription management link. You need to enable Subscription Tracking for getting this type of event.
    pub unsubscribe: bool,
    ///The URL that you want the event webhook to POST to.
    pub url: String,
}
impl std::fmt::Display for EventWebhookUpdateOauthRequest {
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
pub struct FullSegment(pub SegmentSummary, serde_json::Value, SegmentWriteV2);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GlobalEmptyRequest {}
impl std::fmt::Display for GlobalEmptyRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GlobalErrorResponseSchema {
    pub errors: Option<Vec<serde_json::Value>>,
    pub id: Option<String>,
}
impl std::fmt::Display for GlobalErrorResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalId(pub i64);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GoogleAnalyticsSettings {
    ///Indicates if Google Analytics is enabled.
    pub enabled: Option<bool>,
    ///The name of the campaign.
    pub utm_campaign: Option<String>,
    ///Used to differentiate ads
    pub utm_content: Option<String>,
    ///Name of the marketing medium (e.g. "Email").
    pub utm_medium: Option<String>,
    ///Name of the referrer source.
    pub utm_source: Option<String>,
    ///Any paid keywords.
    pub utm_term: Option<String>,
}
impl std::fmt::Display for GoogleAnalyticsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
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
pub struct IpWarmupResponse(pub Vec<serde_json::Value>);
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LinkTrackingMetadata {
    ///The number of items in the entire list, i.e., across all pages.
    pub count: Option<f64>,
    ///The URL of the next page of results. If this field isn't present, you're at the end of the list.
    pub next: Option<String>,
    ///The URL of the previous page of results. If this field isn't present, you're at the start of the list.
    pub prev: Option<String>,
    #[serde(rename = "self")]
    ///The URL of the current page of results.
    pub self_: Option<String>,
}
impl std::fmt::Display for LinkTrackingMetadata {
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
    ///The ID of the branded link.
    pub id: i64,
    ///Indicates if this link branding was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create new link branding if you need to update it.
    pub legacy: bool,
    ///The subdomain used to generate the DNS records for this link branding. This subdomain must be different from the subdomain used for your authenticated domain.
    pub subdomain: Option<String>,
    ///The ID of the user that this link branding is associated with.
    pub user_id: i64,
    ///The username of the account that this link branding is associated with.
    pub username: String,
    ///Indicates if this link branding is valid.
    pub valid: bool,
}
impl std::fmt::Display for LinkBranding200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct List {
    #[serde(rename = "_metadata")]
    pub metadata: Option<Selfmetadata>,
    ///The number of contacts currently stored on the list.
    pub contact_count: Option<i64>,
    ///The generated ID for your list.
    pub id: Option<String>,
    ///The name you gave your list.
    pub name: Option<String>,
}
impl std::fmt::Display for List {
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
pub struct MailSettingsAddressWhitelabel {
    ///Indicates if you have an email address whitelist enabled.
    pub enabled: Option<bool>,
    ///All email addresses that are currently on the whitelist.
    pub list: Option<Vec<String>>,
}
impl std::fmt::Display for MailSettingsAddressWhitelabel {
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
pub struct MailSettingsFooter {
    ///Indicates if the Footer mail setting is currently enabled.
    pub enabled: Option<bool>,
    ///The custom HTML content of your email footer.
    pub html_content: Option<String>,
    ///The plain text content of your email footer.
    pub plain_content: Option<String>,
}
impl std::fmt::Display for MailSettingsFooter {
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
pub struct MakoEvent {
    ///Used with "deferred" events to indicate the attempt number out of 10. One "deferred" entry will exists under events array for each time a message was deferred with error message from the server.
    pub attempt_num: Option<i64>,
    ///Use to distinguish between types of bounces
    pub bounce_type: String,
    ///Name of event
    pub event_name: String,
    ///Client recipient used to click or open message
    pub http_user_agent: String,
    ///For example mx.gmail.com
    pub mx_server: String,
    ///Date of when event occurred
    pub processed: String,
    ///Explanation of what caused "bounced", "deferred", or "blocked". Usually contains error message from the server - e.g. message from gmail why mail was deferred
    pub reason: Option<String>,
    ///Used with "clicked" event to indicate which url the user clicked.
    pub url: String,
}
impl std::fmt::Display for MakoEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    pub api_key_id: String,
    pub asm_group_id: i64,
    ///Categories users associated to the message
    pub categories: Vec<String>,
    ///List of events related to email message
    pub events: Vec<serde_json::Value>,
    pub from_email: String,
    pub msg_id: String,
    ///This is the IP of the user who sent the message.
    pub originating_ip: String,
    ///IP used to send to the remote MTA. Used by UI to display IP in detailed view
    pub outbound_ip: String,
    ///Whether or not the outbound IP is dedicated vs shared
    pub outbound_ip_type: String,
    ///Quick summary of the status of a message
    pub status: String,
    pub subject: String,
    ///Teammate's username
    pub teammate: String,
    pub template_id: String,
    pub to_email: String,
    ///JSON hash of arbitrary key-value pairs
    pub unique_args: String,
}
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    ///The number of items in the entire list, i.e., across all pages.
    pub count: Option<f64>,
    ///The URL of the next page of results. If this field isn't present, you're at the end of the list.
    pub next: Option<String>,
    ///The URL of the previous page of results. If this field isn't present, you're at the start of the list.
    pub prev: Option<String>,
    #[serde(rename = "self")]
    ///The URL of the current page of results.
    pub self_: Option<String>,
}
impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metrics {
    pub bounce_drops: i64,
    pub bounces: i64,
    pub clicks: i64,
    pub delivered: i64,
    pub invalid_emails: i64,
    pub opens: i64,
    pub requests: i64,
    pub spam_report_drops: i64,
    pub spam_reports: i64,
    pub unique_clicks: i64,
    pub unique_opens: i64,
    pub unsubscribes: i64,
}
impl std::fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Monitor {
    ///The email address to which Sendgrid should send emails for monitoring.
    pub email: String,
    ///The frequency at which to forward monitoring emails. An email will be sent when your subuser sends this many (e.g., 1,000) emails.
    pub frequency: f64,
}
impl std::fmt::Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ParseSetting {
    ///A specific and unique domain or subdomain that you have created to use exclusively to parse your incoming email. For example, `parse.yourdomain.com`.
    pub hostname: Option<String>,
    ///Indicates if you would like SendGrid to post the original MIME-type content of your parsed email. When this parameter is set to `true`, SendGrid will send a JSON payload of the content of your email.
    pub send_raw: Option<bool>,
    ///Indicates if you would like SendGrid to check the content parsed from your emails for spam before POSTing them to your domain.
    pub spam_check: Option<bool>,
    ///The public URL where you would like SendGrid to POST the data parsed from your email. Any emails sent with the given hostname provided (whose MX records have been updated to point to SendGrid) will be parsed and POSTed to this URL.
    pub url: Option<String>,
}
impl std::fmt::Display for ParseSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PartnerSettingsNewRelic {
    ///Indicates if your subuser statistics will be sent to your New Relic Dashboard.
    pub enable_subuser_statistics: Option<bool>,
    ///Indicates if this setting is enabled.
    pub enabled: bool,
    ///The license key provided with your New Relic account.
    pub license_key: String,
}
impl std::fmt::Display for PartnerSettingsNewRelic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReplyToEmailObject {
    ///The email address where any replies or bounces will be returned.
    pub email: String,
    ///A name or title associated with the `reply_to` email address.
    pub name: Option<String>,
}
impl std::fmt::Display for ReplyToEmailObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReservedFieldDefinitionsResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct ReverseDns {
    pub a_record: serde_json::Value,
    ///The root, or sending, domain.
    pub domain: String,
    ///The ID of the Reverse DNS.
    pub id: i64,
    ///The IP address that this Reverse DNS was created for.
    pub ip: String,
    ///A Unix epoch timestamp representing the last time of a validation attempt.
    pub last_validation_attempt_at: Option<i64>,
    ///Indicates if this Reverse DNS was created using the legacy whitelabel tool. If it is a legacy whitelabel, it will still function, but you'll need to create a new Reverse DNS if you need to update it.
    pub legacy: bool,
    ///The reverse DNS record for the IP address. This points to the Reverse DNS subdomain.
    pub rdns: String,
    ///The subdomain created for this reverse DNS. This is where the rDNS record points.
    pub subdomain: Option<String>,
    ///The users who are able to send mail from the IP address.
    pub users: Vec<serde_json::Value>,
    ///Indicates if this is a valid Reverse DNS.
    pub valid: bool,
}
impl std::fmt::Display for ReverseDns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentQueryJson {
    pub contacts: Option<serde_json::Value>,
}
impl std::fmt::Display for SegmentQueryJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentResponse {
    ///Total number of contacts present in the segment
    pub contacts_count: i64,
    ///A subset of all contacts that are in this segment
    pub contacts_sample: Vec<ContactResponse>,
    ///ISO8601 timestamp of when the object was created
    pub created_at: String,
    ///ID assigned to the segment when created.
    pub id: String,
    ///Name of the segment.
    pub name: String,
    ///ISO8601 timestamp of when the samples will be next updated
    pub next_sample_update: String,
    ///The array of list ids to filter contacts on when building this segment. It allows only one such list id for now. We will support more in future
    pub parent_list_ids: Vec<String>,
    ///SQL query which will filter contacts based on the conditions provided
    pub query_dsl: String,
    ///If not set, segment contains a Query for use with Segment v1 APIs. If set to '2', segment contains a SQL query for use in v2.
    pub query_version: String,
    ///ISO8601 timestamp of when the samples were last updated
    pub sample_updated_at: String,
    ///Segment status indicates whether the segment's contacts will be updated periodically
    pub status: SegmentStatusResponse,
    ///ISO8601 timestamp of when the object was last updated
    pub updated_at: String,
}
impl std::fmt::Display for SegmentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentStatusResponse {
    ///Describes any errors that were encountered during query validation
    pub error_message: Option<String>,
    ///Status of query validation. PENDING, VALID, or INVALID
    pub query_validation: String,
}
impl std::fmt::Display for SegmentStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentSummary {
    pub contacts_count: i64,
    /**ISO8601 of created timestamp
*/
    pub created_at: String,
    pub id: String,
    pub name: Option<String>,
    ///ISO8601 string that is equal to `sample_updated_at` plus an internally calculated offset that depends on how often contacts enter or exit segments as the scheduled pipeline updates the samples.
    pub next_sample_update: Option<String>,
    ///The id of the list if this segment is a child of a list.  This implies the query `AND CONTAINS(list_ids, ${parent_list_id})`
    pub parent_list_id: Option<String>,
    ///ISO8601 timestamp the sample was last updated
    pub sample_updated_at: String,
    ///ISO8601 timestamp the object was last updated
    pub updated_at: String,
}
impl std::fmt::Display for SegmentSummary {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentWrite {
    pub name: String,
    ///Use this field for adding your query string.
    pub query_dsl: String,
}
impl std::fmt::Display for SegmentWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SegmentWriteV2 {
    ///Name of the segment.
    pub name: String,
    ///The array of list ids to filter contacts on when building this segment. It allows only one such list id for now. We will support more in future
    pub parent_list_ids: Option<Vec<String>>,
    ///SQL query which will filter contacts based on the conditions provided
    pub query_dsl: String,
}
impl std::fmt::Display for SegmentWriteV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Selfmetadata {
    #[serde(rename = "self")]
    ///A link to this object.
    pub self_: Option<String>,
}
impl std::fmt::Display for Selfmetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SenderIdRequest {
    ///The physical address of the sender identity.
    pub address: Option<String>,
    #[serde(rename = "address_2")]
    ///Additional sender identity address information.
    pub address2: Option<String>,
    ///The city of the sender identity.
    pub city: Option<String>,
    ///The country of the sender identity.
    pub country: Option<String>,
    pub from: Option<serde_json::Value>,
    ///A nickname for the sender identity. Not used for sending.
    pub nickname: Option<String>,
    pub reply_to: Option<serde_json::Value>,
    ///The state of the sender identity.
    pub state: Option<String>,
    ///The zipcode of the sender identity.
    pub zip: Option<String>,
}
impl std::fmt::Display for SenderIdRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SenderId(pub SenderIdRequest, serde_json::Value, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SendersIdRequestBody {
    ///The physical address of the sender identity.
    pub address: String,
    #[serde(rename = "address_2")]
    ///Additional sender identity address information.
    pub address2: Option<String>,
    ///The city of the sender identity.
    pub city: String,
    ///The country of the sender identity.
    pub country: String,
    pub from: serde_json::Value,
    ///A nickname for the sender identity. Not used for sending.
    pub nickname: String,
    pub reply_to: Option<serde_json::Value>,
    ///The state of the sender identity.
    pub state: Option<String>,
    ///The zipcode of the sender identity.
    pub zip: Option<String>,
}
impl std::fmt::Display for SendersIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SinglesendRequest {
    ///The categories to associate with this Single Send.
    pub categories: Option<Vec<String>>,
    pub email_config: Option<serde_json::Value>,
    ///The name of the Single Send.
    pub name: String,
    ///The ISO 8601 time at which to send the Single Send. This must be in future or the string "now". Emails can be scheduled up to 72 hours in advance. However, this scheduling constraint does not apply to campaigns sent via [Marketing Campaigns](https://docs.sendgrid.com/ui/sending-email/how-to-send-email-with-marketing-campaigns/).
    pub send_at: Option<String>,
    pub send_to: Option<serde_json::Value>,
}
impl std::fmt::Display for SinglesendRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SinglesendResponse(pub SinglesendRequest, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SinglesendResponseShort {
    pub abtest: Option<AbtestSummary>,
    ///categories to associate with this Single Send
    pub categories: Vec<String>,
    ///the ISO 8601 time at which the Single Send was created
    pub created_at: String,
    pub id: String,
    ///true if the Single Send's AB Test functionality has been toggled on
    pub is_abtest: bool,
    ///name of the Single Send
    pub name: String,
    ///The ISO 8601 time at which to send the Single Send. This must be in future or the string "now". Emails can be scheduled up to 72 hours in advance. However, this scheduling constraint does not apply to campaigns sent via [Marketing Campaigns](https://docs.sendgrid.com/ui/sending-email/how-to-send-email-with-marketing-campaigns/).
    pub send_at: Option<String>,
    ///current status of the Single Send
    pub status: String,
    ///the ISO 8601 time at which the Single Send was last updated
    pub updated_at: String,
}
impl std::fmt::Display for SinglesendResponseShort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SinglesendSchedule {
    ///The ISO 8601 time at which to send the Single Send. This must be in future or the string "now". Emails can be scheduled up to 72 hours in advance. However, this scheduling constraint does not apply to campaigns sent via [Marketing Campaigns](https://docs.sendgrid.com/ui/sending-email/how-to-send-email-with-marketing-campaigns/).
    pub send_at: String,
    pub status: Option<String>,
}
impl std::fmt::Display for SinglesendSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SinglesendSearch {
    ///categories to associate with this Single Send, match any single send that has at least one of the categories
    pub categories: Option<Vec<String>>,
    ///leading and trailing wildcard search on name of the Single Send
    pub name: Option<String>,
    ///current status of the Single Send
    pub status: Option<Vec<String>>,
}
impl std::fmt::Display for SinglesendSearch {
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
pub struct SinglesendsLinkStatsResponse {
    #[serde(rename = "_metadata")]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct SinglesendsResponse {
    #[serde(rename = "_metadata")]
    pub metadata: Metadata,
    pub results: Vec<serde_json::Value>,
}
impl std::fmt::Display for SinglesendsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SpamReportsResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SsoCertificateBody {
    ///A unique ID assigned to the certificate by SendGrid.
    pub id: Option<f64>,
    ///An ID that matches a certificate to a specific IdP integration.
    pub intergration_id: Option<String>,
    ///A unix timestamp (e.g., 1603915954) that indicates the time after which the certificate is no longer valid.
    pub not_after: Option<f64>,
    ///A unix timestamp (e.g., 1603915954) that indicates the time before which the certificate is not valid.
    pub not_before: Option<f64>,
    ///This certificate is used by Twilio SendGrid to verify that SAML requests are coming from Okta. This is called the X509 certificate in the Twilio SendGrid UI.
    pub public_certificate: Option<String>,
}
impl std::fmt::Display for SsoCertificateBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoErrorResponse(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoIntegration(pub CreateIntegrationRequest, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SsoTeammateCommonFields {
    ///The Teammate’s email address. This email address will also function as the Teammate’s username and must match the address assigned to the user in your IdP. This address cannot be changed after the Teammate is created.
    pub email: String,
    ///The Teammate’s first name.
    pub first_name: String,
    ///Indicates if the Teammate has admin permissions.
    pub is_admin: Option<bool>,
    ///Indicates if the Teammate has read_only permissions.
    pub is_read_only: Option<bool>,
    ///The Teammate’s last name.
    pub last_name: String,
}
impl std::fmt::Display for SsoTeammateCommonFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoTeammateRequest(pub SsoTeammateCommonFields, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoTeammateResponse(pub SsoTeammateCommonFields, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SsoTeammatesPatchResponse(pub SsoTeammateResponse, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct StatsAdvancedGlobalStats(pub AdvancedStatsClicksOpens, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct StatsAdvancedStatsBaseSchema(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionTrackingSettings {
    ///Indicates if subscription tracking is enabled.
    pub enabled: Option<bool>,
    ///The information and HTML for your unsubscribe link.
    pub html_content: Option<String>,
    ///The HTML that will be displayed on the page that your customers will see after clicking unsubscribe, hosted on SendGrid’s server.
    pub landing: Option<String>,
    ///The information in plain text for your unsubscribe link. You should have the “<% %>” tag in your content, otherwise the user will have no URL for unsubscribing.
    pub plain_content: Option<String>,
    ///Your custom defined replacement tag for your templates. Use this tag to place your unsubscribe content anywhere in your emailtemplate.
    pub replace: Option<String>,
    ///The URL where you would like your users sent to unsubscribe.
    pub url: Option<String>,
}
impl std::fmt::Display for SubscriptionTrackingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Subuser {
    ///Whether or not the user is enabled or disabled.
    pub disabled: bool,
    ///The email address to contact this subuser.
    pub email: String,
    ///The ID of this subuser.
    pub id: f64,
    ///The name by which this subuser will be referred.
    pub username: String,
}
impl std::fmt::Display for Subuser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubuserPost {
    pub authorization_token: Option<String>,
    pub credit_allocation: Option<serde_json::Value>,
    ///The email address for this subuser.
    pub email: String,
    pub signup_session_token: Option<String>,
    ///The user ID for this subuser.
    pub user_id: f64,
    ///The username of the subuser.
    pub username: String,
}
impl std::fmt::Display for SubuserPost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubuserStats {
    ///The date the statistics were gathered.
    pub date: Option<String>,
    ///The list of statistics.
    pub stats: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for SubuserStats {
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
pub struct SuppressionGroup {
    ///A description of the suppression group.
    pub description: String,
    ///The id of the suppression group.
    pub id: f64,
    ///Indicates if this is the default suppression group.
    pub is_default: Option<bool>,
    pub last_email_sent_at: Option<i64>,
    ///The name of the suppression group. Each group created by a user must have a unique name.
    pub name: String,
    ///The unsubscribes associated with this group.
    pub unsubscribes: Option<i64>,
}
impl std::fmt::Display for SuppressionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
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
pub struct ToEmailArray(pub Vec<serde_json::Value>);
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
pub struct TransactionalTemplatesTemplateLean {
    ///Defines the generation of the template.
    pub generation: String,
    ///The ID of the transactional template.
    pub id: String,
    ///The name for the transactional template.
    pub name: String,
    #[serde(rename = "updated_at ")]
    ///The date and time that this transactional template version was updated.
    pub updated_at: String,
    ///The different versions of this transactional template.
    pub versions: Option<Vec<TransactionalTemplatesVersionOutputLean>>,
}
impl std::fmt::Display for TransactionalTemplatesTemplateLean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionalTemplatesVersionOutputLean {
    ///Set the version as the active version associated with the template. Only one version of a template can be active. The first version created for a template will automatically be set to Active.
    pub active: Option<i64>,
    ///The editor used in the UI.
    pub editor: Option<String>,
    ///If true, plain_content is always generated from html_content. If false, plain_content is not altered.
    pub generate_plain_content: Option<bool>,
    ///ID of the transactional template version.
    pub id: Option<String>,
    ///Name of the transactional template version.
    pub name: Option<String>,
    ///Subject of the new transactional template version.
    pub subject: Option<String>,
    ///ID of the transactional template.
    pub template_id: Option<String>,
    ///A Thumbnail preview of the template's html content.
    pub thumbnail_url: Option<String>,
    ///The date and time that this transactional template version was updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for TransactionalTemplatesVersionOutputLean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionalTemplate(
    pub TransactionalTemplatesTemplateLean,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionalTemplateVersionCreate {
    ///Set the version as the active version associated with the template (0 is inactive, 1 is active). Only one version of a template can be active. The first version created for a template will automatically be set to Active.
    pub active: Option<i64>,
    ///The editor used in the UI.
    pub editor: Option<String>,
    ///If true, plain_content is always generated from html_content. If false, plain_content is not altered.
    pub generate_plain_content: Option<bool>,
    ///The HTML content of the version. Maximum of 1048576 bytes allowed.
    pub html_content: Option<String>,
    ///Name of the transactional template version.
    pub name: String,
    ///Text/plain content of the transactional template version. Maximum of 1048576 bytes allowed.
    pub plain_content: Option<String>,
    ///Subject of the new transactional template version.
    pub subject: String,
    ///For dynamic templates only, the mock json data that will be used for template preview and test sends.
    pub test_data: Option<String>,
}
impl std::fmt::Display for TransactionalTemplateVersionCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionalTemplateVersionOutput(
    pub serde_json::Value,
    TransactionalTemplateVersionCreate,
    TransactionalTemplatesVersionOutputLean,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserProfile {
    ///The street address for this user profile.
    pub address: Option<String>,
    ///An optional second line for the street address of this user profile.
    pub address2: Option<String>,
    ///The city for the user profile.
    pub city: Option<String>,
    ///That company that this user profile is associated with.
    pub company: Option<String>,
    ///Th country of this user profile.
    pub country: Option<String>,
    ///The first name of the user.
    pub first_name: Option<String>,
    ///The last name of the user.
    pub last_name: Option<String>,
    ///The phone number for the user.
    pub phone: Option<String>,
    ///The state for this user.
    pub state: Option<String>,
    ///The website associated with this user.
    pub website: Option<String>,
    ///The zip code for this user.
    pub zip: Option<String>,
}
impl std::fmt::Display for UserProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserScheduledSendStatus(pub MailBatchId, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VerifiedSenderRequestSchema {
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
impl std::fmt::Display for VerifiedSenderRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VerifiedSenderResponseSchema {
    pub address: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub from_email: Option<String>,
    pub from_name: Option<String>,
    pub id: Option<i64>,
    pub locked: Option<bool>,
    pub nickname: Option<String>,
    pub reply_to: Option<String>,
    pub reply_to_name: Option<String>,
    pub state: Option<String>,
    pub verified: Option<bool>,
    pub zip: Option<String>,
}
impl std::fmt::Display for VerifiedSenderResponseSchema {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebhooksEventWebhookRequest {
    ///Receiving server could not or would not accept message.
    pub bounce: bool,
    ///Recipient clicked on a link within the message. You need to enable Click Tracking for getting this type of event.
    pub click: bool,
    ///Recipient's email server temporarily rejected message.
    pub deferred: bool,
    ///Message has been successfully delivered to the receiving server.
    pub delivered: bool,
    ///You may see the following drop reasons: Invalid SMTPAPI header, Spam Content (if spam checker app enabled), Unsubscribed Address, Bounced Address, Spam Reporting Address, Invalid, Recipient List over Package Quota
    pub dropped: bool,
    ///Indicates if the event webhook is enabled.
    pub enabled: bool,
    ///Recipient resubscribes to specific group by updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_resubscribe: bool,
    ///Recipient unsubscribe from specific group, by either direct link or updating preferences. You need to enable Subscription Tracking for getting this type of event.
    pub group_unsubscribe: bool,
    ///The client ID Twilio SendGrid sends to your OAuth server or service provider to generate an OAuth access token. When passing data in this field, you must also include the oauth_token_url field.
    pub oauth_client_id: Option<String>,
    ///The URL where Twilio SendGrid sends the Client ID and Client Secret to generate an access token. This should be your OAuth server or service provider. When passing data in this field, you must also include the oauth_client_id field.
    pub oauth_token_url: Option<String>,
    ///Recipient has opened the HTML message. You need to enable Open Tracking for getting this type of event.
    pub open: bool,
    ///Message has been received and is ready to be delivered.
    pub processed: bool,
    ///Recipient marked a message as spam.
    pub spam_report: bool,
    ///Recipient clicked on message's subscription management link. You need to enable Subscription Tracking for getting this type of event.
    pub unsubscribe: bool,
    ///The URL that you want the event webhook to POST to.
    pub url: String,
}
impl std::fmt::Display for WebhooksEventWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
