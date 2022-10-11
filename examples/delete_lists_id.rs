#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client
        .delete_lists_id(id)
        .delete_contacts(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
