#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .get_ips()
        .ip("your ip")
        .exclude_whitelabels(true)
        .limit(1)
        .offset(1)
        .subuser("your subuser")
        .sort_by_direction("your sort by direction")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
