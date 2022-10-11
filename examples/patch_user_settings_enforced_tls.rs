#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_user_settings_enforced_tls()
        .on_behalf_of("your on behalf of")
        .require_tls(true)
        .require_valid_cert(true)
        .version(1.0)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
