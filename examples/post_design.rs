use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostDesignRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostDesignRequired {
        editor: "your editor",
        plain_content: "your plain content",
        subject: "your subject",
        name: "your name",
        generate_plain_content: true,
        html_content: "your html content",
        categories: &["your categories"],
    };
    let response = client.post_design(args).send().await.unwrap();
    println!("{:#?}", response);
}
