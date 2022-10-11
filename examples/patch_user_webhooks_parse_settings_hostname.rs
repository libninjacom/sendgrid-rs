#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let hostname = "your hostname";
    let response = client
        .patch_user_webhooks_parse_settings_hostname(hostname)
        .on_behalf_of("your on behalf of")
        .send_raw(true)
        .spam_check(true)
        .url("your url")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
