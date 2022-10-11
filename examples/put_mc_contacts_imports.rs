#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let field_mappings = vec![::serde_json::json!({})];
    let file_type = "your file type";
    let response = client
        .put_mc_contacts_imports(field_mappings, file_type)
        .list_ids(&["your list ids"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
