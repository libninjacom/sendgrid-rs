use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_mail_settings_forward_spam()
        .on_behalf_of("your on behalf of")
        .email("your email")
        .enabled(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
