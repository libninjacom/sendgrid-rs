#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .delete_whitelabel_domains_subuser()
        .username("your username")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
