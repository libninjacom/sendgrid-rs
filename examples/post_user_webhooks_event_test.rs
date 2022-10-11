use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .post_user_webhooks_event_test()
        .on_behalf_of("your on behalf of")
        .oauth_client_id("your oauth client id")
        .oauth_client_secret("your oauth client secret")
        .oauth_token_url("your oauth token url")
        .url("your url")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
