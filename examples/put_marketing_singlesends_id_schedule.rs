#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let send_at = "your send at";
    let response = client
        .put_marketing_singlesends_id_schedule(id, send_at)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
