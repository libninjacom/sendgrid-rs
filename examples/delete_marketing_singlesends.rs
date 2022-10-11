use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .delete_marketing_singlesends()
        .ids(&["your ids"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
