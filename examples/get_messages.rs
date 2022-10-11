#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let query = "your query";
    let authorization = "your authorization";
    let response = client
        .get_messages(query, authorization)
        .limit(1.0)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
