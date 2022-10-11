use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let template_id = "your template id";
    let version_id = "your version id";
    let response = client
        .delete_templates_template_id_versions_version_id(template_id, version_id)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
