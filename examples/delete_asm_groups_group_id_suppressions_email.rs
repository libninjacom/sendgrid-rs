use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let group_id = "your group id";
    let email = "your email";
    let response = client
        .delete_asm_groups_group_id_suppressions_email(group_id, email)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
