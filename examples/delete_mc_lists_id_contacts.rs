#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let contact_ids = "your contact ids";
    let id = "your id";
    let response = client
        .delete_mc_lists_id_contacts(contact_ids, id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
