#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client
        .get_automation_link_stat(id)
        .group_by(&["your group by"])
        .step_ids(&["your step ids"])
        .page_size(1)
        .page_token("your page token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
