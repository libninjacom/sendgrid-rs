use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let emails = &["your emails"];
    let template_id = "your template id";
    let response = client
        .post_marketing_test_send_email(emails, template_id)
        .custom_unsubscribe_url("your custom unsubscribe url")
        .from_address("your from address")
        .sender_id(1)
        .suppression_group_id(1)
        .version_id_override("your version id override")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
