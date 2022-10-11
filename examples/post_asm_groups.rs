#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .post_asm_groups()
        .on_behalf_of("your on behalf of")
        .description("your description")
        .is_default(true)
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
