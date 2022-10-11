#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let segment_id = "your segment id";
    let response = client
        .get_segments_segment_id(segment_id)
        .contacts_sample(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
