use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client
        .post_sendgrid_pre_built_design(id)
        .editor("your editor")
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
