#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let name = "your name";
    let query_dsl = "your query dsl";
    let response = client
        .post_segments(name, query_dsl)
        .parent_list_ids(&["your parent list ids"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
