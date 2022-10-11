#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let token = "your token";
    let response = client.get_verified_senders_verify_token(token).send().await.unwrap();
    println!("{:#?}", response);
}
