use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostSendersRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostSendersRequired {
        address2: "your address 2",
        nickname: "your nickname",
        city: "your city",
        country: "your country",
        zip: "your zip",
        from: ::serde_json::json!({}),
        state: "your state",
        address: "your address",
        reply_to: ::serde_json::json!({}),
    };
    let response = client
        .post_senders(args)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
