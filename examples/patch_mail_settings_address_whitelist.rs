use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_mail_settings_address_whitelist()
        .on_behalf_of("your on behalf of")
        .enabled(true)
        .list(&["your list"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
