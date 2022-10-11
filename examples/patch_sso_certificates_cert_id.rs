use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let cert_id = "your cert id";
    let response = client
        .patch_sso_certificates_cert_id(cert_id)
        .enabled(true)
        .integration_id("your integration id")
        .public_certificate("your public certificate")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
