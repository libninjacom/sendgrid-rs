#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let count = 1;
    let response = client
        .post_ips(count)
        .subusers(&["your subusers"])
        .warmup(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
