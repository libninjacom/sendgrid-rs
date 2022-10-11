#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostSubusersRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostSubusersRequired {
        username: "your username",
        ips: &["your ips"],
        password: "your password",
        email: "your email",
    };
    let response = client.post_subusers(args).send().await.unwrap();
    println!("{:#?}", response);
}
