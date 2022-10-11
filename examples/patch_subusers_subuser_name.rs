use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let subuser_name = "your subuser name";
    let response = client
        .patch_subusers_subuser_name(subuser_name)
        .disabled(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
