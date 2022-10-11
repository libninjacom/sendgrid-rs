use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let segment_id = "your segment id";
    let name = "your name";
    let query_dsl = "your query dsl";
    let response = client
        .patch_marketing_segments_segment_id(segment_id, name, query_dsl)
        .parent_list_ids(&["your parent list ids"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
