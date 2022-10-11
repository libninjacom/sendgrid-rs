use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .post_marketing_singlesends_search()
        .page_size(1)
        .page_token("your page token")
        .categories(&["your categories"])
        .name("your name")
        .status(&["your status"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
