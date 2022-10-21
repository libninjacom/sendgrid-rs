#![allow(unused_imports)]
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let contacts = vec![
        ContactRequest { custom_fields : Some(CustomFieldsById {}), first_name :
        Some("your first name".to_owned()), last_name : Some("your last name"
        .to_owned()), postal_code : Some("your postal code".to_owned()), email :
        "your email".to_owned(), city : Some("your city".to_owned()),
        state_province_region : Some("your state province region".to_owned()), country :
        Some("your country".to_owned()), address_line1 : Some("your address line 1"
        .to_owned()), address_line2 : Some("your address line 2".to_owned()),
        alternate_emails : Some(vec!["your alternate emails".to_owned()]) }
    ];
    let response = client
        .put_mc_contacts(contacts)
        .list_ids(&["your list ids"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
