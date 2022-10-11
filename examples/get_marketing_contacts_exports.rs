#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client.get_marketing_contacts_exports().send().await.unwrap();
    println!("{:#?}", response);
}
