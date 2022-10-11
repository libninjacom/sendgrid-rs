use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let subuser_name = "your subuser name";
    let email = "your email";
    let frequency = 1.0;
    let response = client
        .post_subusers_subuser_name_monitor(subuser_name, email, frequency)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
