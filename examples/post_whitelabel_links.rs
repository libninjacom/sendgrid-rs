use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let domain = "your domain";
    let response = client
        .post_whitelabel_links(domain)
        .on_behalf_of("your on behalf of")
        .default(true)
        .subdomain("your subdomain")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
