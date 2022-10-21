#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostSendersRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostSendersRequired {
        nickname: "your nickname",
        address: "your address",
        country: "your country",
        from: ::serde_json::json!({}),
        state: "your state",
        city: "your city",
        reply_to: ::serde_json::json!({}),
        zip: "your zip",
        address2: "your address 2",
    };
    let response = client
        .post_senders(args)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
