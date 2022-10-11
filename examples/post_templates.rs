use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let name = "your name";
    let response = client
        .post_templates(name)
        .on_behalf_of("your on behalf of")
        .generation("your generation")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
