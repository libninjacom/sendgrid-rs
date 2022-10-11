use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client
        .post_marketing_singlesends_id(id)
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
