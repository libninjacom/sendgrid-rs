use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .get_automations_stats_export()
        .ids(&["your ids"])
        .timezone("your timezone")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
