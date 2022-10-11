use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let api_key_id = "your api key id";
    let name = "your name";
    let response = client
        .put_api_keys_api_key_id(api_key_id, name)
        .on_behalf_of("your on behalf of")
        .scopes(&["your scopes"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
