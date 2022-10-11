#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .delete_mc_contacts()
        .delete_all_contacts("your delete all contacts")
        .ids("your ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
