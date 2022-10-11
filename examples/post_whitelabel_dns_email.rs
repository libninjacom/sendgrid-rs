use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let domain_id = 1;
    let email = "your email";
    let link_id = 1;
    let response = client
        .post_whitelabel_dns_email(domain_id, email, link_id)
        .message("your message")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
