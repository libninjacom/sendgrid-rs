use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let campaign_id = 1;
    let send_at = 1;
    let response = client
        .post_campaigns_campaign_id_schedules(campaign_id, send_at)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
