#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostSendersRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostSendersRequired {
        zip: "your zip",
        address: "your address",
        reply_to: ::serde_json::json!({}),
        state: "your state",
        address2: "your address 2",
        city: "your city",
        from: ::serde_json::json!({}),
        country: "your country",
        nickname: "your nickname",
    };
    let response = client
        .post_senders(args)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
