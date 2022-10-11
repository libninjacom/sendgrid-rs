use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let list_id = 1;
    let recipient_id = "your recipient id";
    let response = client
        .post_contactdb_lists_list_id_recipients_recipient_id(list_id, recipient_id)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
