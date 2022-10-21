#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostMarketingSendersRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostMarketingSendersRequired {
        country: "your country",
        city: "your city",
        nickname: "your nickname",
        from: ::serde_json::json!({}),
        address: "your address",
    };
    let response = client
        .post_marketing_senders(args)
        .on_behalf_of("your on behalf of")
        .address2("your address 2")
        .reply_to(::serde_json::json!({}))
        .state("your state")
        .zip("your zip")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
