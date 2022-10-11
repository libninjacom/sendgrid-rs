#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .post_contactdb_custom_fields()
        .on_behalf_of("your on behalf of")
        .name("your name")
        .type_("your type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
