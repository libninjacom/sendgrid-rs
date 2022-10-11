use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client.get_verified_senders_domains().send().await.unwrap();
    println!("{:#?}", response);
}