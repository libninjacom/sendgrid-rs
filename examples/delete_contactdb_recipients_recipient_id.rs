use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let recipient_id = "your recipient id";
    let response = client
        .delete_contactdb_recipients_recipient_id(recipient_id)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}