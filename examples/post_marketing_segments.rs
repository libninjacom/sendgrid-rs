#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostMarketingSegmentsRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostMarketingSegmentsRequired {
        parent_list_id: "your parent list id",
        parent_list_ids: &["your parent list ids"],
        query_dsl: "your query dsl",
        name: "your name",
    };
    let response = client.post_marketing_segments(args).send().await.unwrap();
    println!("{:#?}", response);
}
