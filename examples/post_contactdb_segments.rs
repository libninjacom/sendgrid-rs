use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let conditions = vec![
        ContactdbSegmentsConditions { operator : "your operator".to_owned(), and_or :
        Some("your and or".to_owned()), value : "your value".to_owned(), field :
        "your field".to_owned() }
    ];
    let name = "your name";
    let response = client
        .post_contactdb_segments(conditions, name)
        .on_behalf_of("your on behalf of")
        .list_id(1)
        .recipient_count(1.0)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
