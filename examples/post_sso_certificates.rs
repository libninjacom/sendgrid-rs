use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let integration_id = "your integration id";
    let public_certificate = "your public certificate";
    let response = client
        .post_sso_certificates(integration_id, public_certificate)
        .enabled(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
