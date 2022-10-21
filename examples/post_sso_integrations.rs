#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostSsoIntegrationsRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostSsoIntegrationsRequired {
        enabled: true,
        entity_id: "your entity id",
        signout_url: "your signout url",
        name: "your name",
        signin_url: "your signin url",
    };
    let response = client
        .post_sso_integrations(args)
        .completed_integration(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
