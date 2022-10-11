use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .get_whitelabel_domains()
        .limit(1)
        .offset(1)
        .exclude_subusers(true)
        .username("your username")
        .domain("your domain")
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
