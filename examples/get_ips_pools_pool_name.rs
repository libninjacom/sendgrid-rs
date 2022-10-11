use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let pool_name = "your pool name";
    let response = client.get_ips_pools_pool_name(pool_name).send().await.unwrap();
    println!("{:#?}", response);
}
