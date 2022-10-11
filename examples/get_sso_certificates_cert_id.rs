use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let cert_id = "your cert id";
    let response = client.get_sso_certificates_cert_id(cert_id).send().await.unwrap();
    println!("{:#?}", response);
}
