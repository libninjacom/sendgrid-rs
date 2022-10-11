use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client.get_sso_integrations().si(true).send().await.unwrap();
    println!("{:#?}", response);
}