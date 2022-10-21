#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
use sendgrid2::request::PostMailSendRequired;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let args = PostMailSendRequired {
        subject: "your subject",
        content: vec![::serde_json::json!({})],
        from: FromEmailObject {
            email: "your email".to_owned(),
            name: Some("your name".to_owned()),
        },
        personalizations: vec![::serde_json::json!({})],
    };
    let response = client
        .post_mail_send(args)
        .asm(::serde_json::json!({}))
        .attachments(vec![::serde_json::json!({})])
        .batch_id("your batch id")
        .categories(&["your categories"])
        .custom_args("your custom args")
        .headers(::serde_json::json!({}))
        .ip_pool_name("your ip pool name")
        .mail_settings(::serde_json::json!({}))
        .reply_to(ReplyToEmailObject {
            name: Some("your name".to_owned()),
            email: "your email".to_owned(),
        })
        .reply_to_list(vec![::serde_json::json!({})])
        .send_at(1)
        .template_id("your template id")
        .tracking_settings(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
