use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let title = "your title";
    let response = client
        .post_campaigns(title)
        .on_behalf_of("your on behalf of")
        .categories(&["your categories"])
        .custom_unsubscribe_url("your custom unsubscribe url")
        .editor("your editor")
        .html_content("your html content")
        .ip_pool("your ip pool")
        .list_ids(&[1])
        .plain_content("your plain content")
        .segment_ids(&[1])
        .sender_id(1)
        .subject("your subject")
        .suppression_group_id(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
