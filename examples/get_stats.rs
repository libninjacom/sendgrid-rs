#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let start_date = "your start date";
    let response = client
        .get_stats(start_date)
        .on_behalf_of("your on behalf of")
        .limit(1)
        .offset(1)
        .aggregated_by("your aggregated by")
        .end_date("your end date")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
