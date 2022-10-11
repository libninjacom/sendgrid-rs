#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let name = "your name";
    let response = client
        .post_marketing_singlesends(name)
        .categories(&["your categories"])
        .email_config(::serde_json::json!({}))
        .send_at("your send at")
        .send_to(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
