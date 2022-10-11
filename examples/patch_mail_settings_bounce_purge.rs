use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_mail_settings_bounce_purge()
        .on_behalf_of("your on behalf of")
        .enabled(true)
        .hard_bounces(1)
        .soft_bounces(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
