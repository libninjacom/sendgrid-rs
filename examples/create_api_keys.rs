#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let name = "your name";
    let response = client
        .create_api_keys(name)
        .on_behalf_of("your on behalf of")
        .scopes(&["your scopes"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
