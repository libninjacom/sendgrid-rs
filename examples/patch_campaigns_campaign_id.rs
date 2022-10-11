use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PatchCampaignsCampaignIdRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PatchCampaignsCampaignIdRequired {
        categories: &["your categories"],
        html_content: "your html content",
        title: "your title",
        plain_content: "your plain content",
        campaign_id: 1,
        subject: "your subject",
    };
    let response = client
        .patch_campaigns_campaign_id(args)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
