#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_user_profile()
        .on_behalf_of("your on behalf of")
        .address("your address")
        .address2("your address 2")
        .city("your city")
        .company("your company")
        .country("your country")
        .first_name("your first name")
        .last_name("your last name")
        .phone("your phone")
        .state("your state")
        .website("your website")
        .zip("your zip")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
