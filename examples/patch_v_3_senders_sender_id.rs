use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let sender_id = 1;
    let response = client
        .patch_v3_senders_sender_id(sender_id)
        .on_behalf_of("your on behalf of")
        .address("your address")
        .address2("your address 2")
        .city("your city")
        .country("your country")
        .from(::serde_json::json!({}))
        .nickname("your nickname")
        .reply_to(::serde_json::json!({}))
        .state("your state")
        .zip("your zip")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
