use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .getall_singlesend_stats()
        .singlesend_ids(&["your singlesend ids"])
        .page_size(1)
        .page_token("your page token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
