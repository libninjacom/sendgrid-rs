use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_tracking_settings_subscription()
        .on_behalf_of("your on behalf of")
        .enabled(true)
        .html_content("your html content")
        .landing("your landing")
        .plain_content("your plain content")
        .replace("your replace")
        .url("your url")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
