use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let authorization = "your authorization";
    let msg_id = "your msg id";
    let response = client
        .get_v3_messages_msg_id(authorization, msg_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
