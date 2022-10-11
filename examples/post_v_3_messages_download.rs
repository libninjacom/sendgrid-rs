#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let authorization = "your authorization";
    let response = client
        .post_v3_messages_download(authorization)
        .query("your query")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
