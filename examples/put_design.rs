#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client
        .put_design(id)
        .categories(&["your categories"])
        .generate_plain_content(true)
        .html_content("your html content")
        .name("your name")
        .plain_content("your plain content")
        .subject("your subject")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
