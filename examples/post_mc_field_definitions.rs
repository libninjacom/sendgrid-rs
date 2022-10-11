#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let field_type = "your field type";
    let name = "your name";
    let response = client
        .post_mc_field_definitions(field_type, name)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
