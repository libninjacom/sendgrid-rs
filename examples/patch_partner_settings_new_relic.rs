use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_partner_settings_new_relic()
        .on_behalf_of("your on behalf of")
        .enable_subuser_statistics(true)
        .enabled(true)
        .license_key("your license key")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
