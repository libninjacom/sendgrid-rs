#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostDesignRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostDesignRequired {
        name: "your name",
        editor: "your editor",
        generate_plain_content: true,
        subject: "your subject",
        categories: &["your categories"],
        html_content: "your html content",
        plain_content: "your plain content",
    };
    let response = client.post_design(args).send().await.unwrap();
    println!("{:#?}", response);
}
