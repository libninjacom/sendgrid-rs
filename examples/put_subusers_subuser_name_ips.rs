use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let subuser_name = "your subuser name";
    let body = ::serde_json::json!({});
    let response = client
        .put_subusers_subuser_name_ips(subuser_name, body)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
