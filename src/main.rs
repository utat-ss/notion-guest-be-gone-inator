use clap::{arg, command};
use notion::ids::PageId;
use notion::NotionApi;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use futures::executor::block_on;

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
        .arg(arg!([API_KEY] "API key for Notion API").required(true))
        .arg(arg!([PAGE_ID] "ID of the Notion page").required(true))
        .arg(arg!([EMAIL_LIST] "Path to the CSV file containing email list").required(true))
        .get_matches();
    let api_key = matches
        .get_one::<String>("API_KEY")
        .expect("API_KEY is required");
    let page_id = PageId::from_str(
        matches
            .get_one::<String>("PAGE_ID")
            .expect("PAGE_ID is required"),
    )
    .expect("Page id should be parsable");
    // let page_id = page_id_str
    let email_list = matches
        .get_one::<String>("EMAIL_LIST")
        .expect("EMAIL_LIST is required");
    let email_list_path = Path::new(&email_list);

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
    let page = block_on(client.get_page(page_id));
}

