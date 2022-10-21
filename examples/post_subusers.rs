#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostSubusersRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostSubusersRequired {
        password: "your password",
        username: "your username",
        email: "your email",
        ips: &["your ips"],
    };
    let response = client.post_subusers(args).send().await.unwrap();
    println!("{:#?}", response);
}
