#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let email = "your email";
    let response = client
        .post_validations_email(email)
        .source("your source")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
