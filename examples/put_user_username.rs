#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .put_user_username()
        .on_behalf_of("your on behalf of")
        .username("your username")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
