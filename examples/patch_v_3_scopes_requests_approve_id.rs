use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let request_id = "your request id";
    let response = client
        .patch_v3_scopes_requests_approve_id(request_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
