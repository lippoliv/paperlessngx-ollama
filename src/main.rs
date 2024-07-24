use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Documents {
    id: u32,
    content: String,
}

#[derive(Deserialize, Debug)]
struct DocumentsResponse {
    count: u32,
    results: Vec<Documents>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "{paperlessngx_url}/api/documents/?tags__id__all={tags_filter}",
        paperlessngx_url = env!("PAPERLESSNGX_URL"),
        tags_filter = env!("PAPERLESSNGX_TAGS")
    );
    let response = Client::new()
        .get(&request_url)
        .header(
             "Authorization",
            format!(
                "Token {token}",
                token = env!("PAPERLESSNGX_TOKEN")
            ),
        )
        .send()
        .await?;

    if response.status().is_success() {
        let documents = response.json::<DocumentsResponse>().await?;
        println!("{:?}", documents);
        Ok(())
    } else {
        println!("{:?}", response);
        panic!("Couldn't load documents")
    }
}
