#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let segment_id = "your segment id";
    let response = client
        .delete_marketing_segments_segment_id(segment_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
