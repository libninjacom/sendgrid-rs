#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let username = "your username";
    let is_admin = true;
    let scopes = &["your scopes"];
    let response = client
        .patch_v3_teammates_username(username, is_admin, scopes)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
