#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PatchVerifiedSendersIdRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PatchVerifiedSendersIdRequired {
        id: "your id",
        nickname: "your nickname",
        from_email: "your from email",
        reply_to: "your reply to",
    };
    let response = client
        .patch_verified_senders_id(args)
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
