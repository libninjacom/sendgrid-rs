#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let authorization = "your authorization";
    let download_uuid = "your download uuid";
    let response = client
        .get_v3_messages_download_download_uuid(authorization, download_uuid)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
