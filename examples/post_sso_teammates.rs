use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostSsoTeammatesRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostSsoTeammatesRequired {
        last_name: "your last name",
        scopes: &["your scopes"],
        email: "your email",
        first_name: "your first name",
        is_read_only: true,
        is_admin: true,
    };
    let response = client.post_sso_teammates(args).send().await.unwrap();
    println!("{:#?}", response);
}
