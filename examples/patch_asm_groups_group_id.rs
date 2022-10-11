use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PatchAsmGroupsGroupIdRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PatchAsmGroupsGroupIdRequired {
        is_default: true,
        description: "your description",
        group_id: "your group id",
        name: "your name",
    };
    let response = client
        .patch_asm_groups_group_id(args)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
