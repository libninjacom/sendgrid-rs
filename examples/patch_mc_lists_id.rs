#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client.patch_mc_lists_id(id).name("your name").send().await.unwrap();
    println!("{:#?}", response);
}
