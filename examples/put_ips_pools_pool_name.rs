#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let pool_name = "your pool name";
    let response = client
        .put_ips_pools_pool_name(pool_name)
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
