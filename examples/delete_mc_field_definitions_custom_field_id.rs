#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let custom_field_id = "your custom field id";
    let response = client
        .delete_mc_field_definitions_custom_field_id(custom_field_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
