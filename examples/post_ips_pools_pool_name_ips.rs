#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let pool_name = "your pool name";
    let response = client
        .post_ips_pools_pool_name_ips(pool_name)
        .ip("your ip")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
