#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let start_date = "your start date";
    let response = client
        .get_subusers_stats_sums(start_date)
        .sort_by_direction("your sort by direction")
        .end_date("your end date")
        .limit(1)
        .offset(1)
        .aggregated_by("your aggregated by")
        .sort_by_metric("your sort by metric")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
