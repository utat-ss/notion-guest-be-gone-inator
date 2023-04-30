use clap::{arg, command};
use notion::ids::PageId;
use notion::NotionApi;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .arg(arg!([API_KEY] "API key for Notion API").required(true))
        .arg(arg!([PAGE_ID] "ID of the Notion page").required(true))
        .arg(arg!([EMAIL_LIST] "Path to the CSV file containing email list").required(true))
        .get_matches();

    let api_key = matches
        .get_one::<String>("API_KEY")
        .expect("API_KEY is required");
    let page_id = matches
        .get_one::<String>("PAGE_ID")
        .expect("PAGE_ID is required");

    // Create a separate variable for the email list
    let email_list = matches
        .get_one::<String>("EMAIL_LIST")
        .expect("EMAIL_LIST is required");
    let email_list_path = Path::new(&email_list);

    let client = NotionApi::new(api_key.to_string());

    let reader = csv::Reader::from_path(email_list_path)?;
    for result in reader.into_records() {
        let record = result?;
        let email = &record[0];
        println!("Email: {}", email);
    }

    Ok(())
}

// fn revoke_guest_access(client: NotionAPI, page_id: &PageId, user_id: &str) {
//     let page = client.get_page(page_id);
//     let mut properties = page.properties.clone();

//     if let Some(permissions) = properties.page_user_permissions_mut() {
//         permissions.retain(|permission| permission.user_id() != user_id);
//     }

//     client.update_page_properties(page_id, properties);
// }
