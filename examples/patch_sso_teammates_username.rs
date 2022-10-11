#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let username = "your username";
    let response = client
        .patch_sso_teammates_username(username)
        .first_name("your first name")
        .is_admin(true)
        .last_name("your last name")
        .scopes(&["your scopes"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
