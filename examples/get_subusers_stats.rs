use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let subusers = "your subusers";
    let start_date = "your start date";
    let response = client
        .get_subusers_stats(subusers, start_date)
        .limit(1)
        .offset(1)
        .aggregated_by("your aggregated by")
        .end_date("your end date")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
