use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let segment_id = "your segment id";
    let response = client
        .get_marketing_segments_segment_id(segment_id)
        .query_json(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
