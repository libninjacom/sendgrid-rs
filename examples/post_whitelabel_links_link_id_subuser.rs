use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let link_id = 1;
    let response = client
        .post_whitelabel_links_link_id_subuser(link_id)
        .username("your username")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
