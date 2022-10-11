use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .get_subusers()
        .username("your username")
        .limit(1)
        .offset(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
