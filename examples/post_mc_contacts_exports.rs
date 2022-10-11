#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .post_mc_contacts_exports()
        .file_type("your file type")
        .list_ids(&["your list ids"])
        .max_file_size(1)
        .notifications(::serde_json::json!({}))
        .segment_ids(&["your segment ids"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
