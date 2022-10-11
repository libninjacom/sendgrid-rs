use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let username = "your username";
    let response = client.get_whitelabel_domains_subuser(username).send().await.unwrap();
    println!("{:#?}", response);
}
