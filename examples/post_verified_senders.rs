#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let from_email = "your from email";
    let nickname = "your nickname";
    let reply_to = "your reply to";
    let response = client
        .post_verified_senders(from_email, nickname, reply_to)
        .address("your address")
        .address2("your address 2")
        .city("your city")
        .country("your country")
        .from_name("your from name")
        .reply_to_name("your reply to name")
        .state("your state")
        .zip("your zip")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
