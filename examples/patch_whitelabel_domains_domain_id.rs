#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let domain_id = "your domain id";
    let response = client
        .patch_whitelabel_domains_domain_id(domain_id)
        .on_behalf_of("your on behalf of")
        .custom_spf(true)
        .default(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
