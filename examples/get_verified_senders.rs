#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .get_verified_senders()
        .limit(1.0)
        .last_seen_id(1.0)
        .id(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
