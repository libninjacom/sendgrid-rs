#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .patch_tracking_settings_google_analytics()
        .on_behalf_of("your on behalf of")
        .enabled(true)
        .utm_campaign("your utm campaign")
        .utm_content("your utm content")
        .utm_medium("your utm medium")
        .utm_source("your utm source")
        .utm_term("your utm term")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
