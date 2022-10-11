#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .list_sendgrid_pre_built_designs()
        .page_size(1)
        .page_token("your page token")
        .summary(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
