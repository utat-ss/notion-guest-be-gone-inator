use clap::{arg, command};
use dotenv::dotenv;
use futures::executor::block_on;
use notion::ids::PageId;
use notion::NotionApi;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct EmailRecord {
    #[serde(rename = "full name")]
    full_name: String,
    #[serde(rename = "display name")]
    display_name: String,
    email: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .arg(arg!([PAGE_ID] "ID of the Notion page from which to remove guest permissions. This will be the root page of a Notion workspace in most cases").required(true))
        .arg(arg!([EMAIL_LIST] "Path to the CSV file containing email list").required(true))
        .get_matches();
    let page_id = PageId::from_str(
        matches
            .get_one::<String>("PAGE_ID")
            .expect("PAGE_ID is required"),
    )
    .expect("Page id should be parsable");
    let email_list = matches
        .get_one::<String>("EMAIL_LIST")
        .expect("EMAIL_LIST is required");
    let email_list_path = Path::new(&email_list);

    dotenv().ok();
    let api_key = env::var("NOTION_KEY").expect("API_KEY not found in .env");

    let client = NotionApi::new(api_key.to_string())?;

    let mut reader = csv::Reader::from_path(email_list_path)?;
    for result in reader.deserialize() {
        let record: EmailRecord = result?;
        let email = &record.email;
        println!("Email: {}", email);
        revoke_guest_access(&client, &page_id, &email);
    }

    Ok(())
}

fn revoke_guest_access(client: &NotionApi, page_id: &PageId, user_id: &str) {
    let page = block_on(client.get_page(page_id)).expect("Page should be addressable");
    let properties = page.properties;
    println!("{properties:#?}");
}
