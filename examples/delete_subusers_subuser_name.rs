#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let subuser_name = "your subuser name";
    let response = client
        .delete_subusers_subuser_name(subuser_name)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
