use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let date = "your date";
    let subuser_name = "your subuser name";
    let response = client
        .get_subusers_subuser_name_stats_monthly(date, subuser_name)
        .sort_by_metric("your sort by metric")
        .sort_by_direction("your sort by direction")
        .limit(1)
        .offset(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
