use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let name = "your name";
    let response = client
        .patch_contactdb_segments_segment_id(name)
        .segment_id("your segment id")
        .on_behalf_of("your on behalf of")
        .conditions(
            vec![
                ContactdbSegmentsConditions { operator : "your operator".to_owned(),
                and_or : Some("your and or".to_owned()), value : "your value".to_owned(),
                field : "your field".to_owned() }
            ],
        )
        .list_id(1.0)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}