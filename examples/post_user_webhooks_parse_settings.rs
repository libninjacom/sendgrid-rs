use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .post_user_webhooks_parse_settings()
        .on_behalf_of("your on behalf of")
        .hostname("your hostname")
        .send_raw(true)
        .spam_check(true)
        .url("your url")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
