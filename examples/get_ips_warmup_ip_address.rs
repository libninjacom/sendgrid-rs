use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let ip_address = "your ip address";
    let response = client.get_ips_warmup_ip_address(ip_address).send().await.unwrap();
    println!("{:#?}", response);
}
