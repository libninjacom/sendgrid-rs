use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let domain_id = 1;
    let username = "your username";
    let response = client
        .post_whitelabel_domains_domain_id_subuser(domain_id, username)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
