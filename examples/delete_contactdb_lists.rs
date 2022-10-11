#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let body = ::serde_json::json!({});
    let response = client
        .delete_contactdb_lists(body)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
