use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let integration_id = "your integration id";
    let response = client
        .get_sso_integrations_integration_id_certificates(integration_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
