use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let request_id = "your request id";
    let response = client
        .delete_v3_scopes_requests_request_id(request_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
