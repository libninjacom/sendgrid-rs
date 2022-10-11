#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let domain = "your domain";
    let response = client
        .post_whitelabel_domains(domain)
        .on_behalf_of("your on behalf of")
        .automatic_security(true)
        .custom_dkim_selector("your custom dkim selector")
        .custom_spf(true)
        .default(true)
        .ips(&["your ips"])
        .subdomain("your subdomain")
        .username("your username")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
