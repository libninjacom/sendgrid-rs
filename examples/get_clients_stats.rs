use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let start_date = "your start date";
    let response = client
        .get_clients_stats(start_date)
        .on_behalf_of("your on behalf of")
        .end_date("your end date")
        .aggregated_by("your aggregated by")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
