#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let template_id = "your template id";
    let response = client
        .patch_templates_template_id(template_id)
        .on_behalf_of("your on behalf of")
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
