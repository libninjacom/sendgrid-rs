#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let group_id = "your group id";
    let recipient_emails = &["your recipient emails"];
    let response = client
        .post_asm_groups_group_id_suppressions_search(group_id, recipient_emails)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
