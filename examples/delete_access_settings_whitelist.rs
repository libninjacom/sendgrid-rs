#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .delete_access_settings_whitelist()
        .on_behalf_of("your on behalf of")
        .ids(vec![1])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
