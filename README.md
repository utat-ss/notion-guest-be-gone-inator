# Notion GuestBeGoneInator
A script to mass purge guest access from a Notion page

# Usage
The script requires an API key, a page ID, and a csv file containing an email list.

To get the page ID, navigate to the page in the browser. The page ID is the long string of characters folloowing the page name in the url, for example,

https://www.notion.so/utat-ss/UTAT-Space-Systems-`<page ID>`

This page should be the root page of the workspace.

To get or create an API key, visit https://www.notion.so/my-integrations while logged in as the workspace owner.

1. Duplicate the `.env.example` file and rename the duplicate to `.env`, and replace the placeholder values


Then run
```
cargo run <PAGE_ID> <EMAIL_LIST>
```