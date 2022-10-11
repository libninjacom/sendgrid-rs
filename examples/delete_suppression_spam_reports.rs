use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .delete_suppression_spam_reports()
        .on_behalf_of("your on behalf of")
        .delete_all(true)
        .emails(&["your emails"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
