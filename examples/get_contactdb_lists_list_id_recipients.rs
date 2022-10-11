use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let list_id = 1;
    let response = client
        .get_contactdb_lists_list_id_recipients(list_id)
        .page(1)
        .page_size(1)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
