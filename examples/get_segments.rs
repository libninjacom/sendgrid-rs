#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .get_segments()
        .ids(vec![::serde_json::json!({})])
        .parent_list_ids("your parent list ids")
        .no_parent_list_id(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
