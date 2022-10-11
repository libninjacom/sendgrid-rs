use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let template_id = "your template id";
    let name = "your name";
    let subject = "your subject";
    let response = client
        .post_templates_template_id_versions(template_id, name, subject)
        .on_behalf_of("your on behalf of")
        .active(1)
        .editor("your editor")
        .generate_plain_content(true)
        .html_content("your html content")
        .plain_content("your plain content")
        .test_data("your test data")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
