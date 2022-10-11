use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .get_mc_lists()
        .page_size(1.0)
        .page_token("your page token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
