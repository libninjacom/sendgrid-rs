#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let ids = &["your ids"];
    let response = client.post_marketing_contacts_batch(ids).send().await.unwrap();
    println!("{:#?}", response);
}
