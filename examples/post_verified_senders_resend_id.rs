#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client.post_verified_senders_resend_id(id).send().await.unwrap();
    println!("{:#?}", response);
}
