#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let start_date = "your start date";
    let categories = "your categories";
    let response = client
        .get_categories_stats(start_date, categories)
        .end_date("your end date")
        .aggregated_by("your aggregated by")
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
