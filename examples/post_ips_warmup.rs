#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client.post_ips_warmup().ip("your ip").send().await.unwrap();
    println!("{:#?}", response);
}
