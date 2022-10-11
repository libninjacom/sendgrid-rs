use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let domain_id = "your domain id";
    let response = client
        .delete_whitelabel_domains_domain_id(domain_id)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
