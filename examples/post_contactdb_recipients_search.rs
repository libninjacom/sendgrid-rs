use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let conditions = vec![
        ContactdbSegmentsConditions { and_or : Some("your and or".to_owned()), field :
        "your field".to_owned(), value : "your value".to_owned(), operator :
        "your operator".to_owned() }
    ];
    let list_id = 1;
    let response = client
        .post_contactdb_recipients_search(conditions, list_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
