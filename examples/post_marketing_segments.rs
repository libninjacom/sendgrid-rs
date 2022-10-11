#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostMarketingSegmentsRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostMarketingSegmentsRequired {
        query_dsl: "your query dsl",
        name: "your name",
        parent_list_ids: &["your parent list ids"],
        parent_list_id: "your parent list id",
    };
    let response = client.post_marketing_segments(args).send().await.unwrap();
    println!("{:#?}", response);
}
