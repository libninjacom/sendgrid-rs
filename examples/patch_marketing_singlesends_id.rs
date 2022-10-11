use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let name = "your name";
    let response = client
        .patch_marketing_singlesends_id(id, name)
        .categories(&["your categories"])
        .email_config(::serde_json::json!({}))
        .send_at("your send at")
        .send_to(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
