#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let email = "your email";
    let is_admin = true;
    let scopes = &["your scopes"];
    let response = client
        .post_v3_teammates(email, is_admin, scopes)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
