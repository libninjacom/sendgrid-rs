use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client.get_sso_integrations_id(id).si(true).send().await.unwrap();
    println!("{:#?}", response);
}
