use clap::{App, Arg};
use csv;
use notion::ids::PageId;
use notion::NotionApi;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("My Notion Program")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A program to remove guest access to a Notion page")
        .arg(
            Arg::with_name("API_KEY")
                .help("API key for Notion API")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PAGE_ID")
                .help("ID of the Notion page")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("EMAIL_LIST")
                .help("Path to the CSV file containing email list")
                .required(true)
                .index(3),
        )
        .get_matches();

    let api_key = matches.value_of("API_KEY").unwrap();
    let page_id = matches.value_of("PAGE_ID").unwrap();
    let email_list_path = Path::new(matches.value_of("EMAIL_LIST").unwrap());

    let client = NotionApi::new(api_key);

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
