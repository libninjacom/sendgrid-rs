#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let type_ = "your type";
    let response = client
        .post_alerts(type_)
        .authorization("your authorization")
        .on_behalf_of("your on behalf of")
        .email_to("your email to")
        .frequency("your frequency")
        .percentage(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
