use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let sender_id = 1;
    let response = client
        .get_v3_senders_sender_id(sender_id)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
