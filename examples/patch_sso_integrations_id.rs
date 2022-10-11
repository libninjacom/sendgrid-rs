#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PatchSsoIntegrationsIdRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PatchSsoIntegrationsIdRequired {
        id: "your id",
        name: "your name",
        signout_url: "your signout url",
        enabled: true,
        entity_id: "your entity id",
        signin_url: "your signin url",
    };
    let response = client
        .patch_sso_integrations_id(args)
        .si(true)
        .completed_integration(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
