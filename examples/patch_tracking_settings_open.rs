use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_tracking_settings_open()
        .on_behalf_of("your on behalf of")
        .enabled(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
