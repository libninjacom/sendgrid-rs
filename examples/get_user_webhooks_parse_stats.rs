use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let start_date = "your start date";
    let response = client
        .get_user_webhooks_parse_stats(start_date)
        .limit("your limit")
        .offset("your offset")
        .aggregated_by("your aggregated by")
        .end_date("your end date")
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
