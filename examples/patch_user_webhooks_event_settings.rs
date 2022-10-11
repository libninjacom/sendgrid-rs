use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PatchUserWebhooksEventSettingsRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PatchUserWebhooksEventSettingsRequired {
        group_resubscribe: true,
        deferred: true,
        processed: true,
        unsubscribe: true,
        click: true,
        enabled: true,
        group_unsubscribe: true,
        open: true,
        spam_report: true,
        bounce: true,
        dropped: true,
        url: "your url",
        delivered: true,
    };
    let response = client
        .patch_user_webhooks_event_settings(args)
        .on_behalf_of("your on behalf of")
        .oauth_client_id("your oauth client id")
        .oauth_client_secret("your oauth client secret")
        .oauth_token_url("your oauth token url")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
