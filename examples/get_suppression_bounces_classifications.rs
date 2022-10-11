#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let accept = "your accept";
    let response = client
        .get_suppression_bounces_classifications(accept)
        .start_date("your start date")
        .end_date("your end date")
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
