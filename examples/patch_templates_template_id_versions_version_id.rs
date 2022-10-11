use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PatchTemplatesTemplateIdVersionsVersionIdRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PatchTemplatesTemplateIdVersionsVersionIdRequired {
        subject: "your subject",
        version_id: "your version id",
        template_id: "your template id",
        name: "your name",
    };
    let response = client
        .patch_templates_template_id_versions_version_id(args)
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
