use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let id = "your id";
    let response = client
        .get_singlesend_stat(id)
        .aggregated_by("your aggregated by")
        .start_date("your start date")
        .end_date("your end date")
        .timezone("your timezone")
        .page_size(1)
        .page_token("your page token")
        .group_by(&["your group by"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
