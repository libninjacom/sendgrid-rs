#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client.get_mc_lists_id_contacts_count(id).send().await.unwrap();
    println!("{:#?}", response);
}
