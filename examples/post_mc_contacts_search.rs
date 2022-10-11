use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let query = "your query";
    let response = client.post_mc_contacts_search(query).send().await.unwrap();
    println!("{:#?}", response);
}
