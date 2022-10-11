use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let emails = &["your emails"];
    let response = client
        .post_marketing_contacts_search_emails(emails)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
