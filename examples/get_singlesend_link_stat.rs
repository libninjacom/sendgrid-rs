use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client
        .get_singlesend_link_stat(id)
        .page_size(1)
        .page_token("your page token")
        .group_by(&["your group by"])
        .ab_variation_id("your ab variation id")
        .ab_phase_id("your ab phase id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
