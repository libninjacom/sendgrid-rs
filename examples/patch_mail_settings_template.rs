use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_mail_settings_template()
        .on_behalf_of("your on behalf of")
        .enabled(true)
        .html_content("your html content")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
